use std::collections::HashSet;

use common::problem::day::{AoCProblem, Solution};

type Point = (usize, usize);

pub struct Day11 {
    galaxy_locations: HashSet<Point>,
    num_rows: usize,
    num_cols: usize,
}

impl AoCProblem for Day11 {
    fn prepare(input: String) -> Self {
        let mut galaxy_locations = HashSet::new();
        let mut num_rows = 0;
        let mut num_cols = 0;
        for (row, line) in input.lines().enumerate() {
            num_rows += 1;
            num_cols = 0;
            for (col, c) in line.chars().enumerate() {
                num_cols += 1;
                if c == '#' {
                    galaxy_locations.insert((row, col));
                }
            }
        }

        Self {
            galaxy_locations,
            num_rows,
            num_cols,
        }
    }

    fn part1(&mut self) -> Solution {
        let locations = expand_universe(&self.galaxy_locations, self.num_rows, self.num_cols, 2);
        // BFS took ~180319ms in release mode
        // Manhattan took ~286ms in release mode
        compute_sum_of_lengths(&locations).into()
    }

    fn part2(&mut self) -> Solution {
        let locations = expand_universe(
            &self.galaxy_locations,
            self.num_rows,
            self.num_cols,
            1_000_000,
        );
        compute_sum_of_lengths(&locations).into()
    }

    fn day() -> u32 {
        11
    }

    fn year() -> u32 {
        2023
    }
}

/// Computes the sum of the lengths of each unique pair of galaxies in the universe.
///
/// # Parameters
/// - `galaxy`: The locations of the galaxies in the universe.
///
/// # Returns
/// The sum of the lengths of each unique pair of galaxies in the universe.
fn compute_sum_of_lengths(galaxy: &HashSet<Point>) -> usize {
    let mut considered_pairs = HashSet::new();
    let mut sum_shortest = 0;
    for pt1 in galaxy {
        for pt2 in galaxy {
            if pt1 == pt2
                || considered_pairs.contains(&(pt1, pt2))
                || considered_pairs.contains(&(pt2, pt1))
            {
                continue;
            }

            considered_pairs.insert((pt1, pt2));
            considered_pairs.insert((pt2, pt1));

            sum_shortest += find_shortest_distance(*pt1, *pt2);
        }
    }

    sum_shortest
}

/// Finds the shortest distance between two points in the universe, computed using the Manhattan distance.
///
/// # Parameters
/// - `from_point`: The point to start from.
/// - `to_point`: The point to end at.
///
/// # Returns
/// The shortest distance between the two points.
fn find_shortest_distance(from_point: Point, to_point: Point) -> usize {
    let (x1, y1) = from_point;
    let (x2, y2) = to_point;
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}

/// Expands the universe by the given number of times.
///
/// # Parameters
/// - `galaxy_locations`: The locations of the galaxies in the universe.
/// - `num_rows`: The number of rows in the universe.
/// - `num_cols`: The number of columns in the universe.
/// - `expand_times`: The number of times to expand the universe.
///
/// # Returns
/// A tuple where the first element is the new locations of the galaxies and the second element is the new number of rows and columns
/// represented as a point.
fn expand_universe(
    galaxy_locations: &HashSet<Point>,
    num_rows: usize,
    num_cols: usize,
    expand_times: usize,
) -> HashSet<Point> {
    let mut new_row_locations = HashSet::new();
    // Step 1: Go through each row and see which row has no galaxy
    let mut row_offset = 0;
    for row in 0..num_rows {
        // If there are no galaxies, we know that this row can be expanded.
        if galaxy_locations.iter().filter(|(r, _)| *r == row).count() == 0 {
            row_offset += expand_times - 1;
        } else {
            // Otherwise, we need to add the row offset to each point in the current row
            // to simulate an expansion.
            for (r, c) in galaxy_locations.iter().filter(|(r, _)| *r == row) {
                new_row_locations.insert((r + row_offset, *c));
            }
        }
    }

    let mut point_locations = HashSet::new();
    let mut new_galaxies = HashSet::new();
    // Step 2: Go through each column and see which column has no galaxy.
    let mut col_offset = 0;
    for col in 0..num_cols {
        // If there are no galaxies, we know that this column can be expanded.
        if new_row_locations.iter().filter(|(_, c)| *c == col).count() == 0 {
            col_offset += expand_times - 1;
        } else {
            // Otherwise, we need to add the column offset to each point in the current column
            // to simulate an expansion.
            for (r, c) in new_row_locations.iter().filter(|(_, c)| *c == col) {
                new_galaxies.insert((*r, c + col_offset));
                point_locations.insert((*r, c + col_offset));
            }
        }
    }

    new_galaxies
}
