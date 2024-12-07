use common::problem::day::{AoCProblem, Solution};
use std::collections::HashSet;

const GUARD: char = '^';
const SPACE: char = '.';

pub struct Day06 {
    map: Vec<Vec<char>>,
    starting_point: (isize, isize),
    seen_points: HashSet<(isize, isize)>,
}

impl AoCProblem for Day06 {
    fn prepare(input: String) -> Self {
        let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let starting_point = map
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().any(|col| *col == GUARD))
            .map(|(i, row)| {
                (
                    i as isize,
                    (0..row.len()).find(|cidx| row[*cidx] == GUARD).unwrap() as isize,
                )
            })
            .nth(0)
            .unwrap();

        map[starting_point.0 as usize][starting_point.1 as usize] = SPACE;

        Self {
            map,
            starting_point,
            seen_points: HashSet::new(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut di = -1;
        let mut dj = 0;

        let (mut i, mut j) = self.starting_point;
        loop {
            self.seen_points.insert((i, j));
            if is_out_of_bounds(i + di, j + dj, &self.map) {
                break;
            }

            if self.map[(i + di) as usize][(j + dj) as usize] == SPACE {
                i += di;
                j += dj;
            } else {
                (di, dj) = get_next_turn((di, dj));
            }
        }

        self.seen_points.len().into()
    }

    fn part2(&mut self) -> Solution {
        // This is a very brute force solution.
        //
        // This idea uses the assumption that if we were to add a single obstacle, it would
        // only be effective if we put the obstacle at a location within the patrol path
        // (part 1). In other words, if we put the obstacle somewhere that is NOT in the
        // patrol path, then that obstacle is useless.
        //
        // All this function does is checks if adding the obstacle to the specified location will
        // cause the guard to infinitely loop.
        let will_infinite_loop = |obstacle_loc: (isize, isize)| -> bool {
            // Each element is of the form ((x, y), (dx, dy)), where (x, y) is the point and
            // (dx, dy) is the direction we were heading when we hit this point.
            let mut seen_points: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
            let mut di = -1;
            let mut dj = 0;

            let (mut i, mut j) = self.starting_point;
            loop {
                if !seen_points.insert(((i, j), (di, dj))) {
                    return true;
                }

                if is_out_of_bounds(i + di, j + dj, &self.map) {
                    return false;
                }

                if self.map[(i + di) as usize][(j + dj) as usize] == SPACE
                    && (i + di, j + dj) != obstacle_loc
                {
                    i += di;
                    j += dj;
                } else {
                    (di, dj) = get_next_turn((di, dj));
                }
            }
        };

        self.seen_points
            .iter()
            .filter(|pt| will_infinite_loop(**pt))
            .count()
            .into()
    }

    fn day() -> u32 {
        6
    }

    fn year() -> u32 {
        2024
    }
}

/// Simply checks if (i, j) is out of bounds with respect to the given map.
///
/// # Parameters
/// - `i`: The row index.
/// - `j`: The column index.
/// - `map`: The map (2D array).
///
/// # Returns
/// Whether (i, j) is out of bounds.
fn is_out_of_bounds(i: isize, j: isize, map: &[Vec<char>]) -> bool {
    i < 0 || i >= map.len() as isize || j < 0 || j >= map[0].len() as isize
}

/// Gets the directional tuple representing a right turn from the current direction.
///
/// # Parameters
/// - `curr_dir`: The current directional tuple.
///
/// # Returns
/// The next directional tuple.
fn get_next_turn(curr_dir: (isize, isize)) -> (isize, isize) {
    // Need to turn right
    match curr_dir {
        // Facing up -> face right
        (-1, 0) => (0, 1),
        // Facing right -> face down
        (0, 1) => (1, 0),
        // Facing down -> face left
        (1, 0) => (0, -1),
        // Facing left -> face up
        (0, -1) => (-1, 0),
        _ => unreachable!(),
    }
}
