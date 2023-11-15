use std::collections::{HashMap, VecDeque};

use common::problem::day::{AoCProblem, AocPart, Solution};

pub struct Day12 {
    height_map: Vec<Vec<char>>,
}

impl AoCProblem for Day12 {
    fn prepare(input: String) -> Self {
        Self {
            height_map: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        // For part 1, we use BFS to find the fewest possible steps between 'S' (start) and
        // 'E' (end).
        let mut starting_point = None;
        for (i, row) in self.height_map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 'S' {
                    starting_point = Some((i as isize, j as isize));
                }
            }
        }

        match starting_point {
            Some((x, y)) => match bfs(&self.height_map, x, y, AocPart::One) {
                Some(d) => d,
                None => unreachable!("should be reachable"),
            },
            None => unreachable!("must have starting point"),
        }
        .into()
    }

    fn part2(&mut self) -> Solution {
        // For part 2, we just use the same BFS as part 1, but in "reverse"; that is,
        // start at the 'E' and look for the nearest 'a'.
        let mut starting_point = None;
        for (i, row) in self.height_map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 'E' {
                    starting_point = Some((i as isize, j as isize));
                }
            }
        }

        match starting_point {
            Some((x, y)) => match bfs(&self.height_map, x, y, AocPart::Two) {
                Some(d) => d,
                None => unreachable!("should be reachable"),
            },
            None => unreachable!("must have starting point"),
        }
        .into()
    }

    fn day() -> u32 {
        12
    }
}

const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// Runs the BFS algorithm on the given height map, using the given constraints of the problem
/// to find the correct answer.
///
/// # Parameters
/// - `height_map`: The height map.
/// - `x_pos`: The x-position.
/// - `y_pos`: The y-position.
/// - `part`: The part of the puzzle that this BFS algorithm should run for.
///
/// # Returns
/// The distance between the starting point and the end point (depending on the `part`).
fn bfs(height_map: &[Vec<char>], x_pos: isize, y_pos: isize, part: AocPart) -> Option<usize> {
    let destination = match part {
        AocPart::One => 'E',
        AocPart::Two => 'a',
    };

    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    queue.push_back((x_pos, y_pos));
    let mut dist_map: HashMap<(isize, isize), usize> = HashMap::new();
    dist_map.insert((x_pos, y_pos), 0);

    while let Some(pt) = queue.pop_front() {
        let (x, y) = pt;
        let x_u = x as usize;
        let y_u = y as usize;

        let elevation = match height_map[x_u][y_u] {
            'S' if part == AocPart::One => 'a',
            'E' => 'z',
            otherwise => otherwise,
        };

        let curr_dist = *dist_map.get(&pt).unwrap();
        if height_map[x_u][y_u] == destination {
            return Some(curr_dist);
        }

        // Consider all neighbors
        for (dx, dy) in NEIGHBORS {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x < 0
                || new_x >= height_map.len() as isize
                || new_y < 0
                || new_y >= height_map[new_x as usize].len() as isize
            {
                continue;
            }

            let proposed_elevation = match height_map[new_x as usize][new_y as usize] {
                'S' if part == AocPart::One => 'a',
                'E' => 'z',
                otherwise => otherwise,
            };

            match part {
                AocPart::One if proposed_elevation as isize - elevation as isize > 1 => continue,
                AocPart::Two if elevation as isize - proposed_elevation as isize > 1 => continue,
                _ => {}
            }

            if dist_map.contains_key(&(new_x, new_y)) {
                continue;
            }

            dist_map.insert((new_x, new_y), curr_dist + 1);
            queue.push_back((new_x, new_y));
        }
    }

    None
}
