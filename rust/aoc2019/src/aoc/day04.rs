use std::collections::HashMap;

use common::problem::day::{AoCProblem, Solution};

pub struct Day04 {
    min: usize,
    max: usize,
}

// https://adventofcode.com/2019/day/4
impl AoCProblem for Day04 {
    fn prepare(input: String) -> Self {
        let (min, max) = input.lines().nth(0).unwrap().split_once('-').unwrap();
        Self {
            min: min.parse::<usize>().unwrap(),
            max: max.parse::<usize>().unwrap(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut valid_passwords = 0;

        'main: for mut num in self.min..=self.max {
            let mut digits = vec![];
            while num > 0 {
                digits.insert(0, num % 10);
                num /= 10;
            }

            // Two adjacent digits are the same
            if digits.windows(2).all(|x| x[0] != x[1]) {
                continue;
            }

            // Going from left to right, the digits never decrease
            for window in digits.windows(2) {
                if window[0] <= window[1] {
                    continue;
                }

                continue 'main;
            }

            valid_passwords += 1;
        }

        valid_passwords.into()
    }

    fn part2(&mut self) -> Solution {
        let mut valid_passwords = 0;

        'main: for mut num in self.min..=self.max {
            let mut digits = vec![];
            while num > 0 {
                digits.insert(0, num % 10);
                num /= 10;
            }

            // Two adjacent digits are the same
            let mut map = HashMap::new();
            digits.windows(2).for_each(|w| {
                if w[0] != w[1] {
                    return;
                }

                *map.entry(w[0]).or_insert(0) += 1;
            });

            if map.iter().all(|(_, v)| *v != 1) {
                continue;
            }

            // Going from left to right, the digits never decrease
            for window in digits.windows(2) {
                if window[0] <= window[1] {
                    continue;
                }

                continue 'main;
            }

            valid_passwords += 1;
        }

        valid_passwords.into()
    }

    fn day() -> u32 {
        4
    }

    fn year() -> u32 {
        2019
    }
}
