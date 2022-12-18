use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
};

use crate::aoc::aoc_problem::{AoCProblem, Solution};

type Triplet = (isize, isize, isize);
const DIFF: [Triplet; 6] = [
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
];

pub struct Day18 {
    droplet_coordinates: Vec<Triplet>,
}

impl AoCProblem for Day18 {
    fn prepare(input: &str) -> Self {
        Self {
            droplet_coordinates: input
                .lines()
                .map(|l| {
                    let mut iterator = l.split(',').map(|n| n.parse::<isize>().unwrap());
                    (
                        iterator.next().unwrap(),
                        iterator.next().unwrap(),
                        iterator.next().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut mutual_coordinates: HashSet<Triplet> = HashSet::new();
        let mut total_surface_area = 0;
        for pt @ (x, y, z) in &self.droplet_coordinates {
            total_surface_area += 6 - 2 * DIFF
                .iter()
                .filter(|(dx, dy, dz)| mutual_coordinates.contains(&(x + dx, y + dy, z + dz)))
                .count() as isize;
            mutual_coordinates.insert(*pt);
        }

        total_surface_area.into()
    }

    fn part2(&mut self) -> Solution {
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;
        let mut y_min = isize::MAX;
        let mut y_max = isize::MIN;
        let mut z_min = isize::MAX;
        let mut z_max = isize::MIN;

        for (x, y, z) in &self.droplet_coordinates {
            x_min = min(x_min, *x);
            x_max = max(x_max, *x);
            y_min = min(y_min, *y);
            y_max = max(y_max, *y);
            z_min = min(z_min, *z);
            z_max = max(z_max, *z);
        }

        x_min -= 1;
        y_min -= 1;
        z_min -= 1;

        x_max += 1;
        y_max += 1;
        z_max += 1;

        // Approach: use the "flood fill" technique to figure out which "cubes" are
        // exposed. Once we know which cubes are exposed, all we need to do is go through
        // all cubes in our input and see which ones have "exposed" neighbors. For example:
        //
        // Suppose we have the following representation of the problem:
        //         |
        //        CCC
        //       C | CC
        //      C  |  C
        //  ----C--+---C---
        //       C | CC
        //        CCC
        //         |
        //
        // where C represents one of the cubes in our input. Let '#' be the result of
        // running the flood fill algorithm on this input.
        //  ###############
        //  ######CCC######
        //  #####C | CC####
        //  ####C  |  C####
        //  ####C--+---C###
        //  #####C | CC####
        //  ######CCC######
        //  ###############
        //
        // Then, all we need to do is, for each cube C, count the number of neighbors that is
        // just '#'. This gives us the exterior surface area.
        //
        // For this problem, we just need to do it in 3D instead of 2D. The concept remains the
        // same.

        let mut queue: VecDeque<Triplet> = VecDeque::new();
        queue.push_back((x_min, y_min, z_min));

        let mut explored: HashSet<Triplet> = HashSet::new();
        let mut reachable_points: HashSet<Triplet> = HashSet::new();
        while let Some(pt @ (x, y, z)) = queue.pop_front() {
            if explored.contains(&pt)
                || x < x_min
                || y < y_min
                || z < z_min
                || x > x_max
                || y > y_max
                || z > z_max
            {
                continue;
            }

            explored.insert(pt);

            if self.droplet_coordinates.contains(&pt) {
                continue;
            }

            reachable_points.insert(pt);
            for (dx, dy, dz) in DIFF {
                queue.push_back((x + dx, y + dy, z + dz));
            }
        }

        let mut total_surface_area = 0;
        for (x, y, z) in &self.droplet_coordinates {
            for (dx, dy, dz) in DIFF {
                if reachable_points.contains(&(x + dx, y + dy, z + dz)) {
                    total_surface_area += 1;
                }
            }
        }

        total_surface_area.into()
    }
}
