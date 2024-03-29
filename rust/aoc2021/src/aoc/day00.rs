use common::problem::day::{AoCProblem, Solution};

pub struct Day00 {
    num: usize,
}

// Testing
impl AoCProblem for Day00 {
    fn prepare(input: String) -> Self {
        let num = input.lines().nth(0).unwrap().parse().unwrap();
        Day00 { num }
    }

    fn part1(&mut self) -> Solution {
        (self.num + my_helper_function()).into()
    }

    fn part2(&mut self) -> Solution {
        (self.num + 1000 + my_helper_function() * 10).into()
    }

    fn day() -> u32 {
        0
    }

    fn year() -> u32 {
        2021
    }
}

/// A random helper function that does something.
///
/// # Returns
/// - A value.
fn my_helper_function() -> usize {
    10_usize
}
