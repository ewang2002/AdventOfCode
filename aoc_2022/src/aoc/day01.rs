use crate::aoc::aoc_problem::AoCProblem;

pub struct Day01 {
    num: usize,
}

impl AoCProblem<usize, usize> for Day01 {
    fn prepare(input: Vec<&str>) -> Self {
        let num = input[0].parse().unwrap();
        Self { num }
    }

    fn part1(&mut self) -> usize {
        0
    }

    fn part2(&mut self) -> usize {
        0
    }
}