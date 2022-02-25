use crate::aoc::aoc_problem::AoCProblem;

pub struct Day00 {
    num: usize,
}

// Testing
impl AoCProblem<usize, usize> for Day00 {
    fn prepare(input: Vec<String>) -> Self {
        let num = input[0].parse().unwrap();
        Day00 { num }
    }

    fn part1(&self) -> usize {
        self.num + my_helper_function()
    }

    fn part2(&self) -> usize {
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
