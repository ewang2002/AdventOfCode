use crate::aoc::aoc_problem::AoCProblem;

pub struct Day01 {
    grouped_calories: Vec<usize>,
}

impl AoCProblem<usize, usize> for Day01 {
    fn prepare(input: &str) -> Self {
        let mut grouped_calories: Vec<usize> = input
            .split("\r\n\r\n")
            .map(|g| g.lines().map(|l| l.parse::<usize>().unwrap()).sum())
            .collect();

        grouped_calories.sort_by(|a, b| b.cmp(a));
        Self { grouped_calories }
    }

    fn part1(&mut self) -> usize {
        *self.grouped_calories.iter().max().unwrap()
    }

    fn part2(&mut self) -> usize {
        self.grouped_calories.iter().take(3).sum()
    }
}
