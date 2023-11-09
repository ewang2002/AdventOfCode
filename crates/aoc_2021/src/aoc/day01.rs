use common::day::{AoCProblem, Solution};

pub struct Day01 {
    nums: Vec<i32>,
}

// https://adventofcode.com/2021/day/1
impl AoCProblem for Day01 {
    fn prepare(input: String) -> Self {
        let nums = input
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Self { nums }
    }

    fn part1(&mut self) -> Solution {
        self.nums
            .windows(2)
            .filter(|x| x[0] < x[1])
            .count()
            .into()
    }

    fn part2(&mut self) -> Solution {
        // Note that we need to see if:
        //      a + b + c < b + c + d
        //      => a < d
        self.nums
            .windows(4)
            .filter(|x| x[0] < x[3])
            .count()
            .into()
    }
}
