use crate::{aoc::aoc_problem::AoCProblem, intcode::IntCodeComputer};

pub struct Day02 {
    opcodes: Vec<usize>,
}

// https://adventofcode.com/2019/day/2
impl AoCProblem<usize, usize> for Day02 {
    fn prepare(input: Vec<String>) -> Self {
        Self {
            opcodes: input[0]
                .split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn part1(&mut self) -> usize {
        let mut opcodes = self.opcodes.clone();
        opcodes[1] = 12;
        opcodes[2] = 2;
        let mut computer = IntCodeComputer::new(&opcodes);
        computer.run();
        computer[0]
    }

    fn part2(&mut self) -> usize {
        let mut computer = IntCodeComputer::new(&self.opcodes);
        for noun in 0..100 {
            for verb in 0..100 {
                computer[1] = noun;
                computer[2] = verb;

                computer.run();
                if computer[0] == 19690720 {
                    return 100 * noun + verb;
                }

                computer.reset();
            }
        }

        panic!("No solution found.");
    }
}
