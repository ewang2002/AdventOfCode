use crate::aoc::aoc_problem::AoCProblem;

pub struct Day00 {
    num: usize
}

impl AoCProblem<usize, usize> for Day00 {
    fn prepare(input: Vec<String>) -> Self {
        let num = input[0].parse().unwrap();
        return Day00 { num };
    }

    // --- Day 0: Test Round ---
    //
    // This is a test round to make sure everything works as intended.
    //
    // Suppose you are given a number x (your puzzle input). What is x + 10?
    fn part1(&self) -> usize {
        return self.num + my_helper_function();
    }

    // --- Part Two ---
    //
    // Suppose I wanted to find x + 1000 + 10 * 10. What is it?
    fn part2(&self) -> usize {
        return self.num + 1000 + my_helper_function() * 10;
    }
}

/// A random helper function that does something.
///
/// # Returns
/// - A value.
fn my_helper_function() -> usize {
    return 10_usize;
}