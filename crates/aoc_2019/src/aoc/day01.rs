use common::day::{AoCProblem, Solution};

pub struct Day01 {
    modules: Vec<usize>,
}

// https://adventofcode.com/2019/day/1
impl AoCProblem for Day01 {
    fn prepare(input: String) -> Self {
        Self {
            modules: input
                .lines()
                .map(|x| x.parse::<_>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn part1(&mut self) -> Solution {
        self.modules.iter().fold(0, |acc, &x| acc + (x / 3 - 2)).into()
    }

    fn part2(&mut self) -> Solution {
        self.modules.iter().fold(0, |acc, &x| {
            let mut mass = x as i64;
            let mut res = acc as i64;
            loop {
                mass = mass / 3 - 2;
                if mass <= 0 {
                    break;
                }

                res += mass;
            }

            res
        }).into()
    }
}
