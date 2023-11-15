use common::{
    intcode::{parse_intcode, IntCodeComputer},
    problem::day::{AoCProblem, Solution},
};

pub struct Day09 {
    program: Vec<isize>,
}

// https://adventofcode.com/2019/day/9
impl AoCProblem for Day09 {
    fn prepare(input: String) -> Self {
        Self {
            program: parse_intcode(input.lines().nth(0).unwrap()),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut c = IntCodeComputer::new(&self.program, Some(vec![1]));
        c.run_until_completion();
        c.view_stdout().last().unwrap().into()
    }

    fn part2(&mut self) -> Solution {
        let mut c = IntCodeComputer::new(&self.program, Some(vec![2]));
        c.run_until_completion();
        c.view_stdout().last().unwrap().into()
    }

    fn day() -> u32 {
        9
    }
}
