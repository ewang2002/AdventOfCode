use common::problem::day::{AoCProblem, Solution};

pub struct Day00 {
    nums: Vec<usize>
}

impl AoCProblem for Day00 {
    fn prepare(input: String) -> Self {
        Self {
            nums: input.lines()
                .map(|x| x.parse().unwrap())
                .collect()
        }
    }

    fn part1(&mut self) -> Solution {
        self.nums.iter().sum::<usize>().into()
    }

    fn part2(&mut self) -> Solution {
        (self.nums.iter().min().unwrap() * self.nums.iter().max().unwrap()).into()
    }

    fn day() -> u32 {
        0
    }

    fn year() -> u32 {
        2024
    }
}
