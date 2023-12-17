use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

use common::problem::day::{AoCProblem, Solution};

pub struct Day16 {
    contraption: Vec<Vec<char>>,
}

impl AoCProblem for Day16 {
    fn prepare(input: String) -> Self {
        Self {
            contraption: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut init_starting_state = vec![];
        // We start at the top left corner (0, 0) and are going right (0, 1)
        match self.contraption[0][0] {
            // it continues in the same direction.
            '.' | '-' => {
                init_starting_state.push((0, 0, 0, 1));
            }
            '/' => {
                init_starting_state.push((0, 0, -1, 0));
            }
            '\\' => {
                init_starting_state.push((0, 0, 1, 0));
            }
            '|' => {
                init_starting_state.push((0, 0, 1, 0));
                init_starting_state.push((0, 0, -1, 0));
            }
            _ => panic!("Invalid starting tile"),
        }

        get_all_energized_tiles(&self.contraption, init_starting_state)
            .len()
            .into()
    }

    fn part2(&mut self) -> Solution {
        let mut most_energized = 0;
        // Go through all the starting tiles at the top
        for col_idx in 0..self.contraption[0].len() {
            let starting_states = get_starting_state(&self.contraption, 0, col_idx);
            most_energized = max(
                most_energized,
                get_all_energized_tiles(&self.contraption, starting_states).len(),
            );
        }

        // Go through all the starting tiles at the bottom
        for col_idx in 0..self.contraption[0].len() {
            let starting_states =
                get_starting_state(&self.contraption, self.contraption.len() - 1, col_idx);
            most_energized = max(
                most_energized,
                get_all_energized_tiles(&self.contraption, starting_states).len(),
            );
        }

        // Go through all the starting tiles at the left
        for row_idx in 0..self.contraption.len() {
            let starting_states = get_starting_state(&self.contraption, row_idx, 0);
            most_energized = max(
                most_energized,
                get_all_energized_tiles(&self.contraption, starting_states).len(),
            );
        }

        // Go through all the starting tiles at the right
        for row_idx in 0..self.contraption.len() {
            let starting_states =
                get_starting_state(&self.contraption, row_idx, self.contraption[0].len() - 1);
            most_energized = max(
                most_energized,
                get_all_energized_tiles(&self.contraption, starting_states).len(),
            );
        }

        most_energized.into()
    }

    fn day() -> u32 {
        16
    }

    fn year() -> u32 {
        2023
    }
}

/// Given a contraption and a starting state, return all the energized tiles.
///
/// # Parameters
/// - `contraption`: The contraption
/// - `starting_states`: The starting states, where each element is of the form
///                      `(row_idx, col_idx, drow, dcol)`, where `(row_idx, col_idx)`
///                       is the starting tile, and `(drow, dcol)` is the direction
///                       we're going in.
///
/// # Returns
/// A set of all the energized tiles.
fn get_all_energized_tiles(
    contraption: &[Vec<char>],
    starting_states: Vec<(isize, isize, isize, isize)>,
) -> HashSet<(isize, isize)> {
    let mut queue: VecDeque<(isize, isize, isize, isize)> = VecDeque::new();
    for state in starting_states {
        queue.push_back(state);
    }
    let mut seen_configurations: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    let mut seen_points: HashSet<(isize, isize)> = HashSet::new();
    while let Some(config @ (row_idx, col_idx, drow, dcol)) = queue.pop_front() {
        // If we've already seen this configuration, then we don't need
        // to repeat it again.
        if seen_configurations.contains(&config) {
            continue;
        }

        seen_configurations.insert(config);
        seen_points.insert((row_idx, col_idx));

        let next_row_idx = row_idx + drow;
        let next_col_idx = col_idx + dcol;

        if next_row_idx < 0
            || next_row_idx >= contraption.len() as isize
            || next_col_idx < 0
            || next_col_idx >= contraption[0].len() as isize
        {
            continue;
        }

        let next_tile = contraption[next_row_idx as usize][next_col_idx as usize];
        match next_tile {
            '.' => {
                queue.push_back((next_row_idx, next_col_idx, drow, dcol));
            }
            '-' => match (drow, dcol) {
                (0, 1) | (0, -1) => {
                    queue.push_back((next_row_idx, next_col_idx, drow, dcol));
                }
                _ => {
                    queue.push_back((next_row_idx, next_col_idx, 0, 1));
                    queue.push_back((next_row_idx, next_col_idx, 0, -1));
                }
            },
            '|' => match (drow, dcol) {
                (1, 0) | (-1, 0) => {
                    queue.push_back((next_row_idx, next_col_idx, drow, dcol));
                }
                _ => {
                    queue.push_back((next_row_idx, next_col_idx, 1, 0));
                    queue.push_back((next_row_idx, next_col_idx, -1, 0));
                }
            },
            '/' => {
                let (new_drow, new_dcol) = (-dcol, -drow);
                queue.push_back((next_row_idx, next_col_idx, new_drow, new_dcol));
            }
            '\\' => {
                let (new_drow, new_dcol) = (dcol, drow);
                queue.push_back((next_row_idx, next_col_idx, new_drow, new_dcol));
            }
            _ => panic!("Invalid tile {}", next_tile),
        }
    }

    seen_points
}

/// Given a starting tile, return all the possible starting states.
///
/// # Parameters
/// - `contraption`: The contraption
/// - `row_idx`: The row index of the starting tile
/// - `col_idx`: The column index of the starting tile
///
/// # Returns
/// A vector of all the possible starting states, where each element is of the form
/// `(row_idx, col_idx, drow, dcol)`, where `(row_idx, col_idx)` is the starting tile,
/// and `(drow, dcol)` is the direction we're going in.
///
/// # Remark
/// This function assumes that the starting tile is an edge tile. If it is not, then
/// this function will panic.
fn get_starting_state(
    contraption: &[Vec<char>],
    row_idx: usize,
    col_idx: usize,
) -> Vec<(isize, isize, isize, isize)> {
    // This solution is very brute-force. One way to improve it is to make a note of which direction
    // a beam goes and then keep a note of all the points that it hits. Then, when we have to try a different
    // starting point, if that beam hits a point that we've already seen, then we can just make use of all the
    // points that beam hits instead of having to recompute it.
    let mut states = vec![];

    match (row_idx, col_idx) {
        // If we have a top-left corner tile, then the initial direction is down or right.
        // Let's just go down.
        (0, 0) => match contraption[row_idx][col_idx] {
            '.' => {
                // If we assume we start going down, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, 1, 0));
            }
            '-' => {
                // If we assume we start going down, we need to split going left and right.
                states.push((row_idx as isize, col_idx as isize, 0, 1));
                states.push((row_idx as isize, col_idx as isize, 0, -1));
            }
            '|' => {
                // If we assume we start going down, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, 1, 0));
            }
            '/' => {
                // If we assume we start going down, then we go left
                states.push((row_idx as isize, col_idx as isize, 0, -1));
            }
            '\\' => {
                // If we assume we start going down, then we go right
                states.push((row_idx as isize, col_idx as isize, 0, 1));
            }
            _ => panic!("Invalid starting tile"),
        },
        // If we have a top-right corner tile, then the initial direction is down or left.
        // Let's just go down.
        (0, col_idx) if col_idx == contraption[0].len() - 1 => {
            match contraption[row_idx][col_idx] {
                '.' => {
                    // If we assume we start going down, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, 1, 0));
                }
                '-' => {
                    // If we assume we start going down, we need to split going left and right.
                    states.push((row_idx as isize, col_idx as isize, 0, 1));
                    states.push((row_idx as isize, col_idx as isize, 0, -1));
                }
                '|' => {
                    // If we assume we start going down, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, 1, 0));
                }
                '/' => {
                    // If we assume we start going down, then we go left
                    states.push((row_idx as isize, col_idx as isize, 0, -1));
                }
                '\\' => {
                    // If we assume we start going down, then we go right
                    states.push((row_idx as isize, col_idx as isize, 0, 1));
                }
                _ => panic!("Invalid starting tile"),
            }
        }
        // If we have a bottom-left corner tile, then the initial direction is up or right.
        // Let's just go up.
        (row_idx, 0) if row_idx == contraption.len() - 1 => match contraption[row_idx][col_idx] {
            '.' => {
                // If we assume we start going up, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, -1, 0));
            }
            '-' => {
                // If we assume we start going up, we need to split going left and right.
                states.push((row_idx as isize, col_idx as isize, 0, 1));
                states.push((row_idx as isize, col_idx as isize, 0, -1));
            }
            '|' => {
                // If we assume we start going up, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, -1, 0));
            }
            '/' => {
                // If we assume we start going up, then we go right
                states.push((row_idx as isize, col_idx as isize, 0, 1));
            }
            '\\' => {
                // If we assume we start going up, then we go left
                states.push((row_idx as isize, col_idx as isize, 0, -1));
            }
            _ => panic!("Invalid starting tile"),
        },
        // If we have a bottom-right corner tile, then the initial direction is up or left.
        // Let's just go up.
        (row_idx, col_idx)
            if row_idx == contraption.len() - 1 && col_idx == contraption[0].len() - 1 =>
        {
            match contraption[row_idx][col_idx] {
                '.' => {
                    // If we assume we start going up, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, -1, 0));
                }
                '-' => {
                    // If we assume we start going up, we need to split going left and right.
                    states.push((row_idx as isize, col_idx as isize, 0, 1));
                    states.push((row_idx as isize, col_idx as isize, 0, -1));
                }
                '|' => {
                    // If we assume we start going up, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, -1, 0));
                }
                '/' => {
                    // If we assume we start going up, then we go right
                    states.push((row_idx as isize, col_idx as isize, 0, 1));
                }
                '\\' => {
                    // If we assume we start going left, then we go up
                    states.push((row_idx as isize, col_idx as isize, -1, 0));
                }
                _ => panic!("Invalid starting tile"),
            }
        }
        // If we are on the top edge, then we can only go down
        (0, col_idx) => match contraption[row_idx][col_idx] {
            '.' => {
                // If we assume we start going down, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, 1, 0));
            }
            '-' => {
                // If we're going down and we see this, then we split to left and right
                states.push((row_idx as isize, col_idx as isize, 0, 1));
                states.push((row_idx as isize, col_idx as isize, 0, -1));
            }
            '|' => {
                // If we assume we start going down, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, 1, 0));
            }
            '/' => {
                // If we assume we start going down, then we go left
                states.push((row_idx as isize, col_idx as isize, 0, -1));
            }
            '\\' => {
                // If we assume we start going down, then we go right
                states.push((row_idx as isize, col_idx as isize, 0, 1));
            }
            _ => panic!("Invalid starting tile"),
        },
        // If we are on the bottom edge, then we can only go up
        (row_idx, col_idx) if row_idx == contraption.len() - 1 => {
            match contraption[row_idx][col_idx] {
                '.' => {
                    // If we assume we start going up, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, -1, 0));
                }
                '-' => {
                    // If we're going up and we see this, then we split to left and right
                    states.push((row_idx as isize, col_idx as isize, 0, 1));
                    states.push((row_idx as isize, col_idx as isize, 0, -1));
                }
                '|' => {
                    // If we assume we start going up, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, -1, 0));
                }
                '/' => {
                    // If we assume we start going up, then we go right
                    states.push((row_idx as isize, col_idx as isize, 0, 1));
                }
                '\\' => {
                    // If we assume we start going up, then we go left
                    states.push((row_idx as isize, col_idx as isize, 0, -1));
                }
                _ => panic!("Invalid starting tile"),
            }
        }
        // If we're on the left edge, then we can only go right
        (row_idx, 0) => match contraption[row_idx][col_idx] {
            '.' => {
                // If we assume we start going right, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, 0, 1));
            }
            '-' => {
                // If we assume we start going right, then we can just continue in the same direction.
                states.push((row_idx as isize, col_idx as isize, 0, 1));
            }
            '|' => {
                // If we're going right and we see this, then we split to up and down
                states.push((row_idx as isize, col_idx as isize, 1, 0));
                states.push((row_idx as isize, col_idx as isize, -1, 0));
            }
            '/' => {
                // If we assume we start going right, then we go up
                states.push((row_idx as isize, col_idx as isize, -1, 0));
            }
            '\\' => {
                // If we assume we start going right, then we go down
                states.push((row_idx as isize, col_idx as isize, 1, 0));
            }
            _ => panic!("Invalid starting tile"),
        },
        // Finally, if we're on the right edge, then we can only go left
        (row_idx, col_idx) if col_idx == contraption[0].len() - 1 => {
            match contraption[row_idx][col_idx] {
                '.' => {
                    // If we assume we start going left, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, 0, -1));
                }
                '-' => {
                    // If we assume we start going left, then we can just continue in the same direction.
                    states.push((row_idx as isize, col_idx as isize, 0, -1));
                }
                '|' => {
                    // If we're going left and we see this, then we split to up and down
                    states.push((row_idx as isize, col_idx as isize, 1, 0));
                    states.push((row_idx as isize, col_idx as isize, -1, 0));
                }
                '/' => {
                    // If we assume we start going left, then we go down
                    states.push((row_idx as isize, col_idx as isize, 1, 0));
                }
                '\\' => {
                    // If we assume we start going left, then we go up
                    states.push((row_idx as isize, col_idx as isize, -1, 0));
                }
                _ => panic!("Invalid starting tile"),
            }
        }
        // Otherwise, this is not a valid starting input
        _ => panic!("Invalid starting input"),
    }

    states
}
