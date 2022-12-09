use crate::aoc::aoc_problem::{AoCProblem, Solution};
use std::cmp::max;

pub struct Day08 {
    tree_grid: Vec<Vec<u32>>,
}

impl AoCProblem for Day08 {
    fn prepare(input: &str) -> Self {
        Self {
            tree_grid: input
                .lines()
                .into_iter()
                .map(|l| {
                    l.chars()
                        .into_iter()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut num_visible = 2 * self.tree_grid.len() + 2 * self.tree_grid[0].len() - 4;

        // How many of these elements in the interior are visible?
        for i in 1..self.tree_grid.len() - 1 {
            for j in 1..self.tree_grid[i].len() - 1 {
                let mut valid_up = true;
                let mut valid_down = true;
                let mut valid_right = true;
                let mut valid_left = true;

                // Check up direction
                let mut up_idx: isize = (i as isize) - 1;
                while (up_idx as isize) >= 0 {
                    if self.tree_grid[up_idx as usize][j] >= self.tree_grid[i][j] {
                        valid_up = false;
                        break;
                    }

                    up_idx -= 1;
                }

                // Check down direction
                let mut down_idx: usize = i + 1;
                while down_idx < self.tree_grid[i].len() {
                    if self.tree_grid[down_idx][j] >= self.tree_grid[i][j] {
                        valid_down = false;
                        break;
                    }

                    down_idx += 1;
                }

                // Check left direction
                let mut left_idx: isize = (j as isize) - 1;
                while left_idx >= 0 {
                    if self.tree_grid[i][left_idx as usize] >= self.tree_grid[i][j] {
                        valid_left = false;
                        break;
                    }

                    left_idx -= 1;
                }

                // Check right direction
                let mut right_idx: usize = j + 1;
                while (right_idx as usize) < self.tree_grid[i].len() {
                    if self.tree_grid[i][right_idx as usize] >= self.tree_grid[i][j] {
                        valid_right = false;
                        break;
                    }

                    right_idx += 1;
                }

                num_visible += (valid_left || valid_right || valid_up || valid_down) as usize;
            }
        }

        num_visible.into()
    }

    fn part2(&mut self) -> Solution {
        let mut best_score = 2 * self.tree_grid.len() + 2 * self.tree_grid[0].len() - 4;

        // How many of these elements in the interior are visible?
        for i in 1..self.tree_grid.len() - 1 {
            for j in 1..self.tree_grid[i].len() - 1 {
                let mut num_up = 0;
                let mut num_down = 0;
                let mut num_right = 0;
                let mut num_left = 0;

                // Check up direction
                let mut up_idx: isize = (i as isize) - 1;
                while (up_idx as isize) >= 0 {
                    if self.tree_grid[up_idx as usize][j] >= self.tree_grid[i][j] {
                        break;
                    }

                    num_up += 1;
                    up_idx -= 1;
                }

                if up_idx >= 0 {
                    num_up += 1;
                }

                // Check down direction
                let mut down_idx: usize = i + 1;
                while down_idx < self.tree_grid[i].len() {
                    if self.tree_grid[down_idx][j] >= self.tree_grid[i][j] {
                        break;
                    }

                    num_down += 1;
                    down_idx += 1;
                }

                if down_idx < self.tree_grid[i].len() {
                    num_down += 1;
                }

                // Check left direction
                let mut left_idx: isize = (j as isize) - 1;
                while left_idx >= 0 {
                    if self.tree_grid[i][left_idx as usize] >= self.tree_grid[i][j] {
                        break;
                    }

                    num_left += 1;
                    left_idx -= 1;
                }

                if left_idx >= 0 {
                    num_left += 1;
                }

                // Check right direction
                let mut right_idx: usize = j + 1;
                while (right_idx as usize) < self.tree_grid[i].len() {
                    if self.tree_grid[i][right_idx as usize] >= self.tree_grid[i][j] {
                        break;
                    }

                    num_right += 1;
                    right_idx += 1;
                }

                if right_idx < self.tree_grid[i].len() {
                    num_right += 1;
                }

                best_score = max(best_score, num_left * num_right * num_up * num_down);
            }
        }

        best_score.into()
    }
}
