use crate::aoc::aoc_problem::AoCProblem;
use crate::intcode::{IntCodeComputer, parse_intcode};

pub struct Day05 {
    opcodes: Vec<isize>,
}

// https://adventofcode.com/2019/day/5
impl AoCProblem<isize, isize> for Day05 {
    fn prepare(input: Vec<String>) -> Self {
        Self {
            opcodes: parse_intcode(&input[0])
        }
    }

    fn part1(&mut self) -> isize {
        let mut computer = IntCodeComputer::new(&self.opcodes, Some(vec![1]));
        computer.run();
        let len_of_out = computer.view_stdout().len();
        assert!(computer.view_stdout()[..len_of_out - 1].iter().all(|x| *x == 0));
        *computer.view_stdout().last().unwrap()
    }

    fn part2(&mut self) -> isize {
        let mut computer = IntCodeComputer::new(&self.opcodes, Some(vec![5]));
        computer.run();
        assert_eq!(computer.view_stdout().len(), 1);
        *computer.view_stdout().last().unwrap()
    }
}
