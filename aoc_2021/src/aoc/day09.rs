use crate::aoc::aoc_problem::{AoCProblem, Solution};
use std::collections::{BinaryHeap, HashSet};

pub struct Day09 {
    height_map: Vec<Vec<u32>>,
}

// https://adventofcode.com/2021/day/9
impl AoCProblem for Day09 {
    fn prepare(input: String) -> Self {
        let mut height_map: Vec<Vec<u32>> = vec![];
        for line in input.lines() {
            height_map.push(
                line.split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect(),
            );
        }

        Day09 { height_map }
    }

    fn part1(&mut self) -> Solution {
        return compute_low_points(&self.height_map)
            .iter()
            .fold(0, |acc, val| acc + val.0 + 1)
            .into();
    }

    fn part2(&mut self) -> Solution {
        let low_points = compute_low_points(&self.height_map);
        let mut explored: HashSet<(usize, usize)> = HashSet::new();

        // By default, this is a max heap
        let mut poss_basin_sizes: BinaryHeap<u32> = BinaryHeap::new();
        for (_, (row, col)) in low_points {
            let mut basin_size: u32 = 0;
            explore(&self.height_map, row, col, &mut explored, &mut basin_size);
            poss_basin_sizes.push(basin_size);
        }

        let s1 = poss_basin_sizes.pop().unwrap();
        let s2 = poss_basin_sizes.pop().unwrap();
        let s3 = poss_basin_sizes.pop().unwrap();
        (s1 * s2 * s3).into()
    }
}

/// Recursively explores a basin, getting the size of it.
///
/// # Parameters
/// - `height_map`: The height map vector.
/// - `row`: The current row.
/// - `col`: The current column.
/// - `explored`: The explored points.
/// - `basin_values`: The size of this basin.
///
/// # Returns
/// The size of this basin.
fn explore(
    height_map: &[Vec<u32>],
    row: usize,
    col: usize,
    explored: &mut HashSet<(usize, usize)>,
    basin_size: &mut u32,
) {
    let curr_pt = (row, col);
    if explored.contains(&curr_pt) {
        return;
    }

    explored.insert(curr_pt);
    if height_map[row][col] == 9 {
        return;
    }

    *basin_size += 1;
    if row < height_map.len() - 1 {
        explore(height_map, row + 1, col, explored, basin_size);
    }

    if row > 0 {
        explore(height_map, row - 1, col, explored, basin_size);
    }

    if col < height_map[row].len() - 1 {
        explore(height_map, row, col + 1, explored, basin_size);
    }

    if col > 0 {
        explore(height_map, row, col - 1, explored, basin_size);
    }
}

/// Computes the lowest points in this height map, as specified by the writeup.
///
/// # Parameters
/// - `height_map`: The height map.
///
/// # Returns
/// A vector containing the low points followed by the coordinates that these points were found in.
fn compute_low_points(height_map: &[Vec<u32>]) -> Vec<(u32, (usize, usize))> {
    let max_rows = height_map.len();
    let max_cols = height_map[0].len();
    let mut low_points: Vec<(u32, (usize, usize))> = vec![];

    for row in 0..max_rows {
        for col in 0..max_cols {
            let target = height_map[row][col];

            // Check up
            if row > 0 && target >= height_map[row - 1][col] {
                continue;
            }

            // Check down
            if row < max_rows - 1 && target >= height_map[row + 1][col] {
                continue;
            }

            // Check left
            if col > 0 && target >= height_map[row][col - 1] {
                continue;
            }

            // Check right
            if col < max_cols - 1 && target >= height_map[row][col + 1] {
                continue;
            }

            low_points.push((target, (row, col)));
        }
    }

    low_points
}
