use crate::intcode::parse_intcode;
use crate::{aoc::aoc_problem::AoCProblem, intcode::IntCodeComputer};

pub struct Day02 {
    opcodes: Vec<isize>,
}

// https://adventofcode.com/2019/day/2
impl AoCProblem<isize, isize> for Day02 {
    fn prepare(input: Vec<String>) -> Self {
        Self {
            opcodes: parse_intcode(&input[0]),
        }
    }

    fn part1(&mut self) -> isize {
        let mut opcodes = self.opcodes.clone();
        opcodes[1] = 12;
        opcodes[2] = 2;
        let mut computer = IntCodeComputer::new(&opcodes, None);
        computer.run_until_completion();
        computer[0]
    }

    fn part2(&mut self) -> isize {
        let mut computer = IntCodeComputer::new(&self.opcodes, None);
        for noun in 0..100 {
            for verb in 0..100 {
                computer[1] = noun;
                computer[2] = verb;

                computer.run_until_completion();
                if computer[0] == 19690720 {
                    return 100 * noun + verb;
                }

                computer.reset();
            }
        }

        panic!("No solution found.");
    }
}
