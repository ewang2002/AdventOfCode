use crate::aoc::aoc_problem::AoCProblem;
use crate::intcode::{parse_intcode, IntCodeComputer};

pub struct Day09 {
    program: Vec<isize>,
}

// https://adventofcode.com/2019/day/7
impl AoCProblem<isize, isize> for Day09 {
    fn prepare(input: Vec<String>) -> Self {
        Self {
            program: parse_intcode(&input[0]),
        }
    }

    fn part1(&mut self) -> isize {
        let mut c = IntCodeComputer::new(&self.program, Some(vec![1]));
        c.run_until_completion();
        *c.view_stdout().last().unwrap()
    }

    fn part2(&mut self) -> isize {
        let mut c = IntCodeComputer::new(&self.program, Some(vec![2]));
        c.run_until_completion();
        *c.view_stdout().last().unwrap()
    }
}
