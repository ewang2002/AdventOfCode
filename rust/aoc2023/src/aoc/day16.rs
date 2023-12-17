use std::collections::{HashSet, VecDeque};

use common::problem::day::{AoCProblem, Solution};

pub struct Day16 {
    contraption: Vec<Vec<char>>,
}

impl AoCProblem for Day16 {
    fn prepare(input: String) -> Self {
        Self {
            contraption: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        fn get_mirror_next_direction(direction: (isize, isize), tile: char) -> (isize, isize) {
            match tile {
                '/' => (-direction.1, -direction.0),
                '\\' => (direction.1, direction.0),
                _ => direction,
            }
        }

        // (row_idx, col_idx, drow, dcol)
        let mut queue: VecDeque<(isize, isize, isize, isize)> = VecDeque::new();

        // We start at the top left corner (0, 0) and are going right (0, 1)
        match self.contraption[0][0] {
            // it continues in the same direction.
            '.' | '-' => {
                queue.push_back((0, 0, 0, 1));
            }
            '/' => {
                queue.push_back((0, 0, -1, 0));
            }
            '\\' => {
                queue.push_back((0, 0, 1, 0));
            }
            '|' => {
                queue.push_back((0, 0, 1, 0));
                queue.push_back((0, 0, -1, 0));
            }
            _ => panic!("Invalid starting tile"),
        }

        let mut seen_configurations: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        let mut seen_points: HashSet<(isize, isize)> = HashSet::new();
        while let Some(config @ (row_idx, col_idx, drow, dcol)) = queue.pop_front() {
            // If we've already seen this configuration, then we don't need
            // to repeat it again.
            if seen_configurations.contains(&config) {
                continue;
            }

            seen_configurations.insert(config);
            seen_points.insert((row_idx, col_idx));

            let next_row_idx = row_idx + drow;
            let next_col_idx = col_idx + dcol;

            if next_row_idx < 0
                || next_row_idx >= self.contraption.len() as isize
                || next_col_idx < 0
                || next_col_idx >= self.contraption[0].len() as isize
            {
                continue;
            }

            let next_tile = self.contraption[next_row_idx as usize][next_col_idx as usize];
            match next_tile {
                '.' => {
                    queue.push_back((next_row_idx, next_col_idx, drow, dcol));
                }
                '-' => match (drow, dcol) {
                    (0, 1) | (0, -1) => {
                        queue.push_back((next_row_idx, next_col_idx, drow, dcol));
                    }
                    _ => {
                        queue.push_back((next_row_idx, next_col_idx, 0, 1));
                        queue.push_back((next_row_idx, next_col_idx, 0, -1));
                    }
                },
                '|' => match (drow, dcol) {
                    (1, 0) | (-1, 0) => {
                        queue.push_back((next_row_idx, next_col_idx, drow, dcol));
                    }
                    _ => {
                        queue.push_back((next_row_idx, next_col_idx, 1, 0));
                        queue.push_back((next_row_idx, next_col_idx, -1, 0));
                    }
                },
                '/' | '\\' => {
                    let (new_drow, new_dcol) = get_mirror_next_direction((drow, dcol), next_tile);
                    queue.push_back((next_row_idx, next_col_idx, new_drow, new_dcol));
                }
                _ => panic!("Invalid tile {}", next_tile),
            }
        }

        seen_points.len().into()
    }

    fn part2(&mut self) -> Solution {
        // Rough approach [brute-force]: we can just iterate over all edge points and figure out
        // which one would result in the most number of points being hit.
        //
        // Note that one iteration takes about 25ms in debug mode, so this is a reasonable approach.
        0.into()
    }

    fn day() -> u32 {
        16
    }

    fn year() -> u32 {
        2023
    }
}
