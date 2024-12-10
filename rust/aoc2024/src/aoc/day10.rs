use std::collections::HashSet;

use common::problem::day::{AoCProblem, Solution};

pub struct Day10 {
    topographic_map: Vec<Vec<u32>>,
}

impl AoCProblem for Day10 {
    fn prepare(input: String) -> Self {
        Self {
            topographic_map: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                        .collect()
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        fn calculate_trail_end_points(
            map: &[Vec<u32>],
            i: usize,
            j: usize,
            seen_ends: &mut HashSet<(usize, usize)>,
        ) {
            if map[i][j] == 9 {
                seen_ends.insert((i, j));
                return;
            }

            // Can we go up?
            if i as isize > 0 && map[i - 1][j] == map[i][j] + 1 {
                calculate_trail_end_points(map, i - 1, j, seen_ends);
            }

            // Can we go left?
            if j as isize > 0 && map[i][j - 1] == map[i][j] + 1 {
                calculate_trail_end_points(map, i, j - 1, seen_ends);
            }

            // Can we go down?
            if i + 1 < map.len() && map[i + 1][j] == map[i][j] + 1 {
                calculate_trail_end_points(map, i + 1, j, seen_ends);
            }

            // Can we go right?
            if j + 1 < map[0].len() && map[i][j + 1] == map[i][j] + 1 {
                calculate_trail_end_points(map, i, j + 1, seen_ends);
            }
        }

        (0..self.topographic_map.len())
            .flat_map(|i| (0..self.topographic_map[i].len()).map(move |j| (i, j)))
            .filter(|&(i, j)| self.topographic_map[i][j] == 0)
            .map(|(i, j)| {
                let mut set = HashSet::new();
                calculate_trail_end_points(&self.topographic_map, i, j, &mut set);
                set.len()
            })
            .sum::<usize>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        fn num_good_trails(map: &[Vec<u32>], i: usize, j: usize) -> usize {
            if map[i][j] == 9 {
                return 1;
            }

            let mut num_good = 0;

            // Can we go up?
            if i as isize > 0 && map[i - 1][j] == map[i][j] + 1 {
                num_good += num_good_trails(map, i - 1, j);
            }

            // Can we go left?
            if j as isize > 0 && map[i][j - 1] == map[i][j] + 1 {
                num_good += num_good_trails(map, i, j - 1);
            }

            // Can we go down?
            if i + 1 < map.len() && map[i + 1][j] == map[i][j] + 1 {
                num_good += num_good_trails(map, i + 1, j);
            }

            // Can we go right?
            if j + 1 < map[0].len() && map[i][j + 1] == map[i][j] + 1 {
                num_good += num_good_trails(map, i, j + 1);
            }

            num_good
        }

        (0..self.topographic_map.len())
            .flat_map(|i| (0..self.topographic_map[i].len()).map(move |j| (i, j)))
            .filter(|&(i, j)| self.topographic_map[i][j] == 0)
            .map(|(i, j)| num_good_trails(&self.topographic_map, i, j))
            .sum::<usize>()
            .into()
    }

    fn day() -> u32 {
        10
    }

    fn year() -> u32 {
        2024
    }
}
