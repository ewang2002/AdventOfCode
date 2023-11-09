use crate::intcode::{parse_intcode, IntCodeComputer};
use common::day::{AoCProblem, Solution};

pub struct Day05 {
    opcodes: Vec<isize>,
}

// https://adventofcode.com/2019/day/5
impl AoCProblem for Day05 {
    fn prepare(input: String) -> Self {
        Self {
            opcodes: parse_intcode(input.lines().nth(0).unwrap()),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut computer = IntCodeComputer::new(&self.opcodes, Some(vec![1]));
        computer.run_until_completion();
        let len_of_out = computer.view_stdout().len();
        assert!(computer.view_stdout()[..len_of_out - 1]
            .iter()
            .all(|x| *x == 0));
        computer.view_stdout().last().unwrap().into()
    }

    fn part2(&mut self) -> Solution {
        let mut computer = IntCodeComputer::new(&self.opcodes, Some(vec![5]));
        computer.run_until_completion();
        assert_eq!(computer.view_stdout().len(), 1);
        computer.view_stdout().last().unwrap().into()
    }
}
