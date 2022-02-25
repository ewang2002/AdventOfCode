use crate::aoc::aoc_problem::AoCProblem;

pub struct Day01 {
    nums: Vec<i32>,
}

// https://adventofcode.com/2021/day/1
impl AoCProblem<usize, usize> for Day01 {
    fn prepare(input: Vec<String>) -> Self {
        let nums = input
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Self { nums }
    }

    fn part1(&self) -> usize {
        self.nums
            .windows(2)
            .into_iter()
            .filter(|x| x[0] < x[1])
            .count()
    }

    fn part2(&self) -> usize {
        // Note that we need to see if:
        //      a + b + c < b + c + d
        //      => a < d
        self.nums
            .windows(4)
            .into_iter()
            .filter(|x| x[0] < x[3])
            .count()
    }
}
