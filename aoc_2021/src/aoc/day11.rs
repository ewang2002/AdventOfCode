use std::collections::{BinaryHeap, HashSet};
use crate::aoc::aoc_problem::AoCProblem;

pub struct Day11 {
    energy_levels: Vec<Vec<i32>>,
}

// https://adventofcode.com/2021/day/11
impl AoCProblem<u32, u32> for Day11 {
    fn prepare(input: Vec<String>) -> Self {
        return Day11 {
            energy_levels: input.iter()
                .map(|x| x.split("")
                    .filter(|y| !y.is_empty())
                    .map(|y| y.parse::<_>().unwrap()).collect())
                .collect::<_>()
        };
    }

    fn part1(&self) -> u32 {
        let mut energy_levels = self.energy_levels.clone();

        let mut flashes: u32 = 0;
        for _ in 1..100 {
            let mut flashed_pts: HashSet<(usize, usize)> = HashSet::new();
            for row in 0..energy_levels.len() {
                for col in 0..energy_levels[0].len() {
                    flashes += iterate_energy_level(&mut energy_levels, row, col, &mut flashed_pts);
                }
            }
        }

        return flashes;
    }

    fn part2(&self) -> u32 {
        let mut energy_levels = self.energy_levels.clone();

        for step in 1..1000 {
            let mut flashed_pts: HashSet<(usize, usize)> = HashSet::new();
            for row in 0..energy_levels.len() {
                for col in 0..energy_levels[0].len() {
                    iterate_energy_level(&mut energy_levels, row, col, &mut flashed_pts);
                }
            }

            if flashed_pts.len() == energy_levels.len() * energy_levels[0].len() {
                return step as u32;
            }
        }

        panic!(":(");
    }
}

fn iterate_energy_level(energy_levels: &mut Vec<Vec<i32>>, row: usize, col: usize,
                        flashed_pts: &mut HashSet<(usize, usize)>) -> u32 {
    let this_pt = (row, col);
    if flashed_pts.contains(&this_pt) {
        return 0;
    }

    energy_levels[row][col] += 1;
    if energy_levels[row][col] <= 9 {
        return 0;
    }

    energy_levels[row][col] = 0;
    flashed_pts.insert(this_pt);
    let mut flashes: u32 = 1;

    // Check up
    if row > 0 {
        flashes += iterate_energy_level(energy_levels, row - 1, col, flashed_pts);
    }

    // Check down
    if row < energy_levels.len() - 1 {
        flashes += iterate_energy_level(energy_levels, row + 1, col, flashed_pts);
    }

    // Check left
    if col > 0 {
        flashes += iterate_energy_level(energy_levels, row, col - 1, flashed_pts);
    }

    // Check right
    if col < energy_levels[0].len() - 1 {
        flashes += iterate_energy_level(energy_levels, row, col + 1, flashed_pts);
    }

    // Check up right
    if row > 0 && col < energy_levels[0].len() - 1 {
        flashes += iterate_energy_level(energy_levels, row - 1, col + 1, flashed_pts);
    }

    if row < energy_levels.len() - 1 && col < energy_levels[0].len() - 1 {
        flashes += iterate_energy_level(energy_levels, row + 1, col + 1, flashed_pts);
    }

    if row > 0 && col > 0 {
        flashes += iterate_energy_level(energy_levels, row - 1, col - 1, flashed_pts);
    }

    if row < energy_levels.len() - 1 && col > 0 {
        flashes += iterate_energy_level(energy_levels, row + 1, col - 1, flashed_pts);
    }

    return flashes;
}