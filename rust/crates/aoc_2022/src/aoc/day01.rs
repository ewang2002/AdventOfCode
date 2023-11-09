use common::{
    constants::TWO_NEWLINE,
    day::{AoCProblem, Solution},
};

pub struct Day01 {
    grouped_calories: Vec<usize>,
}

impl AoCProblem for Day01 {
    fn prepare(input: String) -> Self {
        let mut grouped_calories: Vec<usize> = input
            .split(TWO_NEWLINE)
            .map(|g| g.lines().map(|l| l.parse::<usize>().unwrap()).sum())
            .collect();

        grouped_calories.sort_by(|a, b| b.cmp(a));
        Self { grouped_calories }
    }

    fn part1(&mut self) -> Solution {
        self.grouped_calories.iter().max().unwrap().into()
    }

    fn part2(&mut self) -> Solution {
        self.grouped_calories.iter().take(3).sum::<usize>().into()
    }
}
