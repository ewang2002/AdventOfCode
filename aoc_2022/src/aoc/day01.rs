use crate::aoc::aoc_problem::AoCProblem;

pub struct Day01 {
    calories_raw: Vec<String>,
}

impl AoCProblem<usize, usize> for Day01 {
    fn prepare(input: &str) -> Self {
        Self {
            calories_raw: input.split("\r\n\r\n").map(|x| x.to_owned()).collect(),
        }
    }

    fn part1(&mut self) -> usize {
        self.calories_raw
            .iter()
            .map(|c| c.lines().map(|x| x.parse::<usize>().unwrap()).sum())
            .max()
            .unwrap()
    }

    fn part2(&mut self) -> usize {
        let mut d = self
            .calories_raw
            .iter()
            .map(|c| c.lines().map(|x| x.parse::<usize>().unwrap()).sum())
            .collect::<Vec<usize>>();
        d.sort_by(|a, b| b.partial_cmp(a).unwrap());
        d.into_iter().take(3).sum()
    }
}
