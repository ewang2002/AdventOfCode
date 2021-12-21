use std::cmp::{min};
use crate::aoc::aoc_problem::AoCProblem;

pub struct Day21 {
    p1_starting: usize,
    p2_starting: usize,
}

// https://adventofcode.com/2021/day/21
impl AoCProblem<usize, usize> for Day21 {
    fn prepare(input: Vec<String>) -> Self {
        assert_eq!(2, input.len());
        Self {
            p1_starting: input[0].split_once(": ").unwrap().1.parse().unwrap(),
            p2_starting: input[1].split_once(": ").unwrap().1.parse().unwrap(),
        }
    }

    fn part1(&self) -> usize {
        let mut player_position: [usize; 2] = [self.p1_starting, self.p2_starting];
        let mut player_scores: [usize; 2] = [0, 0];

        let mut p: usize = 0;
        let mut die_val = 1;
        let mut rolled = 0;
        loop {
            let mut n = 0;
            for _ in 0..3 {
                n += die_val;
                die_val = (die_val % 10) + 1;
            }

            rolled += 3;

            player_position[p] = ((player_position[p] - 1 + n) % 10) + 1;
            player_scores[p] += player_position[p];
            if player_scores[p] >= 1000 {
                break;
            }

            p = (p + 1) % 2;
        }

        min(player_scores[0], player_scores[1]) * rolled
    }

    fn part2(&self) -> usize {
        0
    }
}
