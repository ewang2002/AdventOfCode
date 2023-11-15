use std::{
    cmp::{max, min},
    collections::HashSet,
};

use common::problem::day::{AoCProblem, Solution};

pub struct Day14 {
    vertical_slices: Vec<Vec<(usize, usize)>>,
    initial_explored: HashSet<(usize, usize)>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
}

impl AoCProblem for Day14 {
    fn prepare(input: String) -> Self {
        let vertical_slices: Vec<Vec<(usize, usize)>> = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|raw_pt| raw_pt.split_once(',').unwrap())
                    .map(|(raw_x, raw_y)| (raw_x.parse().unwrap(), raw_y.parse().unwrap()))
                    .collect()
            })
            .collect();

        let mut explored: HashSet<(usize, usize)> = HashSet::new();

        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut min_y = usize::MIN;
        for path in &vertical_slices {
            for points in path.windows(2) {
                let (start_x, start_y) = points[0];
                let (end_x, end_y) = points[1];

                min_x = min(min(start_x, end_x), min_x);
                max_x = max(max(start_x, end_x), max_x);
                // We're taking the max here because a lower position in the
                // matrix means a higher y value.
                min_y = max(end_y, min_y);

                if start_x == end_x {
                    let init_y = min(start_y, end_y);
                    let final_y = max(start_y, end_y);
                    for y in init_y..=final_y {
                        explored.insert((start_x, y));
                    }
                } else if start_y == end_y {
                    let init_x = min(start_x, end_x);
                    let final_x = max(start_x, end_x);
                    for x in init_x..=final_x {
                        explored.insert((x, start_y));
                    }
                } else {
                    unreachable!("we can assume this won't be the case");
                }
            }
        }
        Self {
            vertical_slices,
            initial_explored: explored,
            min_x,
            min_y,
            max_x,
        }
    }

    fn part1(&mut self) -> Solution {
        let mut explored = self.initial_explored.clone();
        // Approach: we'll have a set of all explored points. We'll put all
        // rocks in our set of explored points. Then, we can just run through
        // each rock until we find out how many rocks it takes for a rock to
        // go into the void.
        // Keep looping. While looping, we can keep dropping sand at 500, 0.
        let mut ct = 0;
        'main: loop {
            let mut highest_x = 500;
            let mut highest_y = 0;
            loop {
                if highest_x == self.min_x || highest_x == self.max_x {
                    break 'main;
                }

                // Check neighbors of this point
                // 1. Can we go down?
                if !explored.contains(&(highest_x, highest_y + 1)) {
                    highest_y += 1;
                    continue;
                }

                // 2. Can we go left & down?
                if !explored.contains(&(highest_x - 1, highest_y + 1)) {
                    highest_x -= 1;
                    highest_y += 1;
                    continue;
                }

                // 3. Can we go right & down?
                if !explored.contains(&(highest_x + 1, highest_y + 1)) {
                    highest_x += 1;
                    highest_y += 1;
                    continue;
                }

                // Otherwise, this grain of sand must be at rest.
                explored.insert((highest_x, highest_y));
                ct += 1;
                break;
            }
        }

        ct.into()
    }

    fn part2(&mut self) -> Solution {
        let mut explored = self.initial_explored.clone();
        let min_y = self.min_y + 2;
        let mut has_reached_starting_pt = false;
        let mut ct = 0;
        'main: loop {
            let mut highest_x = 500;
            let mut highest_y = 0;
            loop {
                // Check neighbors of this point
                // 1. Can we go down?
                if !explored.contains(&(highest_x, highest_y + 1)) && min_y != highest_y + 1 {
                    highest_y += 1;
                    continue;
                }

                // 2. Can we go left & down?
                if !explored.contains(&(highest_x - 1, highest_y + 1)) && min_y != highest_y + 1 {
                    highest_x -= 1;
                    highest_y += 1;
                    continue;
                }

                // 3. Can we go right & down?
                if !explored.contains(&(highest_x + 1, highest_y + 1)) && min_y != highest_y + 1 {
                    highest_x += 1;
                    highest_y += 1;
                    continue;
                }

                if highest_x == 500 && highest_y == 0 {
                    if has_reached_starting_pt {
                        break 'main;
                    }

                    has_reached_starting_pt = true;
                }
                // Otherwise, this grain of sand must be at rest.
                explored.insert((highest_x, highest_y));
                ct += 1;
                break;
            }
        }

        ct.into()
    }

    fn day() -> u32 {
        14
    }
}
