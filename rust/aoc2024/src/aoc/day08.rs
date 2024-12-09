use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use common::problem::day::{AoCProblem, Solution};

pub struct Day08 {
    antenna_map: Vec<Vec<char>>,
    antenna_locations: HashMap<char, Vec<(isize, isize)>>,
}

impl AoCProblem for Day08 {
    fn prepare(input: String) -> Self {
        let antenna_map: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();

        let mut antenna_locations = HashMap::new();
        for (i, row) in antenna_map.iter().enumerate() {
            for (j, antenna) in row.iter().enumerate() {
                if *antenna == '.' {
                    continue;
                }

                antenna_locations
                    .entry(*antenna)
                    .or_insert_with(Vec::new)
                    .push((i as isize, j as isize));
            }
        }

        Self {
            antenna_map,
            antenna_locations,
        }
    }

    fn part1(&mut self) -> Solution {
        let mut antinode_locations = HashSet::new();
        for locations in self.antenna_locations.values() {
            for (idx, first_pt) in locations.iter().enumerate() {
                for second_pt in locations.iter().skip(idx + 1) {
                    compute_proposed_antinode_locations(*first_pt, *second_pt, 1)
                        .into_iter()
                        .filter(|(i, j)| self.is_in_bounds(*i, *j))
                        .for_each(|pt| {
                            antinode_locations.insert(pt);
                        });
                }
            }
        }

        antinode_locations.len().into()
    }

    fn part2(&mut self) -> Solution {
        let mut antinode_locations = HashSet::new();
        for locations in self.antenna_locations.values() {
            for (idx, first_pt) in locations.iter().enumerate() {
                for second_pt in locations.iter().skip(idx + 1) {
                    // "This means that some of the new antinodes will occur at the position
                    // of each antenna (unless that antenna is the only one of its frequency)."
                    //
                    // So, we also need to include the original antenna locations since their
                    // locations will also be locations of antinodes
                    antinode_locations.insert(*first_pt);
                    antinode_locations.insert(*second_pt);

                    compute_proposed_antinode_locations(
                        *first_pt,
                        *second_pt,
                        self.antenna_map.len(),
                    )
                    .into_iter()
                    .filter(|(i, j)| self.is_in_bounds(*i, *j))
                    .for_each(|pt| {
                        antinode_locations.insert(pt);
                    });
                }
            }
        }

        antinode_locations.len().into()
    }

    fn day() -> u32 {
        8
    }

    fn year() -> u32 {
        2024
    }
}

impl Day08 {
    /// Checks if the given indices are in bounds.
    ///
    /// # Parameters
    /// - `i`: The `i`th index.
    /// - `j`: The `j`th index.
    ///
    /// # Returns
    /// If `(i, j)` is in bounds.
    fn is_in_bounds(&self, i: isize, j: isize) -> bool {
        i >= 0
            && i < self.antenna_map.len() as isize
            && j >= 0
            && j < self.antenna_map[0].len() as isize
    }
}

/// Computes the possible locations of the antinodes.
///
/// # Parameters
/// - `first_pt`: The location of the first antenna.
/// - `second_pt`: The location of the second antenna.
/// - `maX_antinodes`: The maximum number of antinodes to look for (in both directions).
///
/// # Returns
/// A list of points where each point represents the location of a possible antinode.
/// This will not include the original input points.
fn compute_proposed_antinode_locations(
    first_pt: (isize, isize),
    second_pt: (isize, isize),
    max_antinodes: usize,
) -> Vec<(isize, isize)> {
    let ((ui, uj), (li, lj)) = if first_pt.0 > second_pt.0 {
        (second_pt, first_pt)
    } else {
        (first_pt, second_pt)
    };

    let delta_i = (li - ui).abs();
    let delta_j = (lj - uj).abs();
    let mut proposed_antinode_locations = vec![];
    if ui - li == 0 {
        // If the slope is undefined, then just add delta_j as needed
        for x in 1..=max_antinodes {
            proposed_antinode_locations.push((li, min(lj, uj) - delta_j * x as isize));
            proposed_antinode_locations.push((li, max(lj, uj) + delta_j * x as isize));
        }
    } else if uj - lj == 0 {
        // If the slope is flat, then just add delta_i
        for x in 1..=max_antinodes {
            proposed_antinode_locations.push((min(li, ui) - delta_i * x as isize, uj));
            proposed_antinode_locations.push((max(li, ui) + delta_i * x as isize, uj));
        }
    } else {
        let slope = (uj - lj) as f64 / (ui - li) as f64;
        let j = |i: isize| -> f64 { slope * (i - li) as f64 + lj as f64 };

        for x in 1..=max_antinodes {
            let upper_antinode_i = ui - delta_i * x as isize;
            let lower_antinode_i = li + delta_i * x as isize;
            let upper_antinode_j = j(upper_antinode_i) as isize;
            let lower_antinode_j = j(lower_antinode_i) as isize;
            proposed_antinode_locations.push((upper_antinode_i, upper_antinode_j));
            proposed_antinode_locations.push((lower_antinode_i, lower_antinode_j));
        }
    }

    proposed_antinode_locations
}
