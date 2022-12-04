use crate::aoc::aoc_problem::AoCProblem;

pub struct Day04 {
    // (left pair min, left pair max, right pair min, right pair max)
    assignments: Vec<(usize, usize, usize, usize)>,
}

impl AoCProblem<usize, usize> for Day04 {
    fn prepare(input: &str) -> Self {
        Self {
            assignments: input
                .lines()
                .map(|l| l.split_once(',').unwrap())
                .map(|(left, right)| {
                    (
                        left.split_once('-').unwrap(),
                        right.split_once('-').unwrap(),
                    )
                })
                .map(|((l_min, l_max), (r_min, r_max))| {
                    (
                        l_min.parse().unwrap(),
                        l_max.parse().unwrap(),
                        r_min.parse().unwrap(),
                        r_max.parse().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> usize {
        let mut overlapping_pairs = 0;
        for (l_min, l_max, r_min, r_max) in &self.assignments {
            // Case 1
            // l    .234567.
            // r    ...45...
            if l_min <= r_min && r_max <= l_max {
                overlapping_pairs += 1;
                continue;
            }

            // Case 2
            // l    ...45...
            // r    .234567.
            if r_min <= l_min && l_max <= r_max {
                overlapping_pairs += 1;
                continue;
            }
        }

        overlapping_pairs
    }

    fn part2(&mut self) -> usize {
        let mut overlapping_pairs = 0;
        for (l_min, l_max, r_min, r_max) in &self.assignments {
            // Case 1
            // l    .234567.
            // r    ...45...
            if l_min <= r_min && r_max <= l_max {
                overlapping_pairs += 1;
                continue;
            }

            // Case 2
            // l    ...45...
            // r    .234567.
            if r_min <= l_min && l_max <= r_max {
                overlapping_pairs += 1;
                continue;
            }

            // Case 3
            // l    .234567.
            // r    ..345678
            if l_min <= r_min && l_max <= r_max && r_min <= l_max {
                overlapping_pairs += 1;
                continue;
            }

            // Case 4
            // l    ..345678
            // r    .234567.
            if r_min <= l_min && r_max <= l_max && l_min <= r_max {
                overlapping_pairs += 1;
                continue;
            }
        }

        overlapping_pairs
    }
}
