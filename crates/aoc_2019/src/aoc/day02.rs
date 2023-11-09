use common::day::{AoCProblem, Solution};

use crate::intcode::{parse_intcode, IntCodeComputer};

pub struct Day02 {
    opcodes: Vec<isize>,
}

// https://adventofcode.com/2019/day/2
impl AoCProblem for Day02 {
    fn prepare(input: String) -> Self {
        Self {
            opcodes: parse_intcode(input.lines().nth(0).unwrap()),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut opcodes = self.opcodes.clone();
        opcodes[1] = 12;
        opcodes[2] = 2;
        let mut computer = IntCodeComputer::new(&opcodes, None);
        computer.run_until_completion();
        computer[0].into()
    }

    fn part2(&mut self) -> Solution {
        let mut computer = IntCodeComputer::new(&self.opcodes, None);
        for noun in 0..100 {
            for verb in 0..100 {
                computer[1] = noun;
                computer[2] = verb;

                computer.run_until_completion();
                if computer[0] == 19690720 {
                    return (100 * noun + verb).into();
                }

                computer.reset();
            }
        }

        panic!("No solution found.");
    }
}
