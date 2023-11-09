use std::collections::HashSet;

use common::day::{AoCProblem, Solution};

pub struct Day06 {
    data_buffer: Vec<char>,
}

impl AoCProblem for Day06 {
    fn prepare(input: String) -> Self {
        Self {
            data_buffer: input.chars().collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        find_pos_of_marker(&self.data_buffer, 4).into()
    }

    fn part2(&mut self) -> Solution {
        find_pos_of_marker(&self.data_buffer, 14).into()
    }
}

/// Finds the position of the first start-of-packet marker in the datastream.
///
/// # Parameters
/// - `datastream`: The datastream.
/// - `num_unique`: The number of unique elements needed to reach this marker.
///
/// # Returns
/// The position of the first start-of-packet marker.
///
/// # Constraints
/// - It's assumed that the `datastream` has a marker.
fn find_pos_of_marker(datastream: &[char], num_unique: usize) -> usize {
    let mut set = HashSet::new();
    for (i, elems) in datastream.windows(num_unique).enumerate() {
        set.clear();
        for e in elems {
            set.insert(e);
        }

        if set.len() != num_unique {
            continue;
        }

        return i + num_unique;
    }

    unreachable!("should not be reachable from here");
}
