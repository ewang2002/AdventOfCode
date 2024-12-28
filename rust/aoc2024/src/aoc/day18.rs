use common::problem::day::{AoCProblem, Solution};
use std::collections::{HashMap, HashSet, VecDeque};

// For real input, use 70
// For test input, use 6
const MEM_SPACE_GOAL: isize = 70;

// For real input, use 1024
// For test input, use 12
const P1_FALLEN_BYTES: isize = 1024;

const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub struct Day18 {
    byte_positions: Vec<(isize, isize)>,
}

impl AoCProblem for Day18 {
    fn prepare(input: String) -> Self {
        Self {
            byte_positions: input
                .lines()
                .map(|l| l.split_once(',').unwrap())
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let corrupted_locations = self
            .byte_positions
            .iter()
            .take(P1_FALLEN_BYTES as usize)
            // Reverse (l, r) to (r, l) to fit the general (i, j) convention where
            // i = row index, and j = column index.
            .map(|&(l, r)| (r, l))
            .collect::<HashSet<_>>();

        let mut distances = HashMap::new();
        distances.insert((0, 0), 0);

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        let mut seen = HashSet::new();
        while let Some(p @ (i, j)) = queue.pop_front() {
            if !(0..=MEM_SPACE_GOAL).contains(&i) || !(0..=MEM_SPACE_GOAL).contains(&j) {
                continue;
            }

            if corrupted_locations.contains(&(i, j)) {
                continue;
            }

            if i == MEM_SPACE_GOAL && j == MEM_SPACE_GOAL {
                break;
            }

            if seen.contains(&p) {
                continue;
            }

            seen.insert(p);
            let new_distance = *distances.get(&(i, j)).unwrap() + 1;
            for (di, dj) in NEIGHBORS {
                let this_dist = distances.entry((i + di, j + dj)).or_insert(usize::MAX);
                if new_distance >= *this_dist {
                    continue;
                }

                *this_dist = new_distance;
                queue.push_back((i + di, j + dj));
            }
        }

        distances
            .get(&(MEM_SPACE_GOAL, MEM_SPACE_GOAL))
            .unwrap()
            .into()
    }

    fn part2(&mut self) -> Solution {
        // Another classic brute-force solution.
        //
        // To get an idea of the approach used here, assume that the bytes in the input file are
        // represented as so:
        //              BBBBBBBBBBBBBBBBB BBBBBBBBBBBBBBBBBBBBBBBBBBBBXBBBBBBBBB
        //
        // where X represents the first byte that will completely prevent the exit from being
        // reachable, and all bytes before the space are bytes that have already "fallen" (used
        // in part 1), and all bytes after the space are bytes to be considered for part 2.
        //
        // Rather than running BFS while accounting for each additional byte (which would take
        // roughly 6 seconds in release mode using my actual input), I first attempt to cut the
        // number of potential bytes down into a much more manageable amount. To do so, rather
        // than considering each additional byte, we can consider each additional group of bytes.
        //
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB|BBBBXBBB|BBBBBB
        //
        // Here, each group of bytes is separated by a pipe. We can run BFS while assuming
        // the following bytes cause the corresponding coordinate to be corrupted:
        //
        //              BBBBBBBBBBBBBBBBB
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB|BBBBXBBB
        //
        // We see that the fourth group has a byte that causes the exit to be unreachable. So,
        // we can now run BFS using all the bytes from the preceding groups, plus each individual
        // byte in the new group; that is, we can run BFS with the following bytes causing
        // corruption at the corresponding coordinate, like so:
        //
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB|B
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB|BB
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB|BBB
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB|BBBB
        //              BBBBBBBBBBBBBBBBB |BBBBBBBB|BBBBBBBB|BBBBBBBB|BBBBX
        //
        // thus, we find the byte that blocks off all exits. This approach takes ~2 seconds in
        // debug mode and ~60ms in release with the actual input.
        fn can_reach_end(
            bad_locs: &HashSet<(isize, isize)>,
            add_bad_locs: &[(isize, isize)],
        ) -> bool {
            let mut queue = VecDeque::new();
            queue.push_back((0, 0));

            let mut seen = HashSet::new();
            while let Some(p @ (i, j)) = queue.pop_front() {
                if !(0..=MEM_SPACE_GOAL).contains(&i) || !(0..=MEM_SPACE_GOAL).contains(&j) {
                    continue;
                }

                if bad_locs.contains(&p) || add_bad_locs.contains(&p) {
                    continue;
                }

                if i == MEM_SPACE_GOAL && j == MEM_SPACE_GOAL {
                    return true;
                }

                if seen.contains(&p) {
                    continue;
                }

                seen.insert(p);
                for (di, dj) in NEIGHBORS {
                    queue.push_back((i + di, j + dj));
                }
            }

            false
        }

        let corrupted_locations = self
            .byte_positions
            .iter()
            .take(P1_FALLEN_BYTES as usize)
            .map(|&(l, r)| (r, l))
            .collect::<HashSet<_>>();

        let possible_corrupted_locs = self
            .byte_positions
            .iter()
            .skip(P1_FALLEN_BYTES as usize)
            .map(|&(l, r)| (r, l))
            .collect::<Vec<_>>();

        let mut prev = 0;
        let mut right = 1;
        loop {
            if right < possible_corrupted_locs.len()
                && can_reach_end(&corrupted_locations, &possible_corrupted_locs[0..right])
            {
                prev = right;
                right += 100;
            } else {
                break;
            }
        }

        for i in prev..right {
            if can_reach_end(&corrupted_locations, &possible_corrupted_locs[0..i]) {
                continue;
            }

            let (l, r) = possible_corrupted_locs[i - 1];
            return format!("{r},{l}").into();
        }

        0.into()
    }

    fn day() -> u32 {
        18
    }

    fn year() -> u32 {
        2024
    }
}
