use crate::aoc::aoc_problem::AoCProblem;

pub struct Day00 {
    num: usize,
}

// Testing
impl AoCProblem<usize, usize> for Day00 {
    fn prepare(input: &str) -> Self {
        let num = input.parse().unwrap();
        Self { num }
    }

    fn part1(&mut self) -> usize {
        self.num + my_helper_function()
    }

    fn part2(&mut self) -> usize {
        self.num + 1000 + my_helper_function() * 10
    }
}

/// A random helper function that does something.
///
/// # Returns
/// - A value.
fn my_helper_function() -> usize {
    10_usize
}
