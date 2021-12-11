use std::collections::{BinaryHeap, HashSet};
use crate::aoc::aoc_problem::AoCProblem;

const COORD_DIFF: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
];

pub struct Day11 {
    energy_levels: Vec<Vec<i32>>
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
        for _ in 1..=100 {
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

    for (dx, dy) in COORD_DIFF {
        let c_row = (row as i32) + dx;
        let c_col = (col as i32) + dy;

        if c_row < 0
            || c_col < 0
            || c_row >= energy_levels.len() as i32
            || c_col >= energy_levels[0].len() as i32 {
            continue;
        }

        flashes += iterate_energy_level(
            energy_levels,
            c_row as usize,
            c_col as usize,
            flashed_pts
        );
    }


    return flashes;
}