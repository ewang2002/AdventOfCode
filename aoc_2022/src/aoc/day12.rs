use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
};

use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day12 {
    height_map: Vec<Vec<char>>,
}

impl AoCProblem for Day12 {
    fn prepare(input: &str) -> Self {
        Self {
            height_map: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut starting_point = None;
        for (i, row) in self.height_map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 'S' {
                    starting_point = Some((i as isize, j as isize));
                }
            }
        }

        match starting_point {
            Some((x, y)) => bfs(&self.height_map, x, y),
            None => unreachable!("must have starting point"),
        }
        .into()
    }

    fn part2(&mut self) -> Solution {
        let mut final_distance = usize::MAX;
        for (i, row) in self.height_map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 'a' {
                    final_distance = min(
                        bfs(&self.height_map, i as isize, j as isize),
                        final_distance,
                    );
                }
            }
        }

        final_distance.into()
    }
}

const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn bfs(height_map: &[Vec<char>], x_pos: isize, y_pos: isize) -> usize {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    queue.push_back((x_pos, y_pos));
    let mut dist_map: HashMap<(isize, isize), usize> = HashMap::new();
    dist_map.insert((x_pos, y_pos), 0);

    while let Some(pt) = queue.pop_front() {
        let (x, y) = pt;
        let x_u = x as usize;
        let y_u = y as usize;

        let elevation = match height_map[x_u][y_u] {
            'S' => 'a',
            'E' => 'z',
            otherwise => otherwise,
        };

        let curr_dist = *dist_map.get(&pt).unwrap();
        if height_map[x_u][y_u] == 'E' {
            return curr_dist;
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
                'S' => 'a',
                'E' => 'z',
                otherwise => otherwise,
            };

            if proposed_elevation as isize - elevation as isize > 1 {
                continue;
            }

            if dist_map.contains_key(&(new_x, new_y)) {
                continue;
            }

            dist_map.insert((new_x, new_y), curr_dist + 1);
            queue.push_back((new_x, new_y));
        }
    }

    usize::MAX
}
