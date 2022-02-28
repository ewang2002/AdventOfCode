use crate::aoc::aoc_problem::AoCProblem;

pub struct Day01 {
    modules: Vec<usize>,
}

// https://adventofcode.com/2019/day/1
impl AoCProblem<usize, usize> for Day01 {
    fn prepare(input: Vec<String>) -> Self {
        Self {
            modules: input
                .iter()
                .map(|x| x.parse::<_>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn part1(&mut self) -> usize {
        self.modules.iter().fold(0, |acc, &x| acc + (x / 3 - 2))
    }

    fn part2(&mut self) -> usize {
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

            res as usize 
        })
    }
}
