use std::collections::{HashMap, HashSet};

use common::problem::day::{AoCProblem, Solution};

pub struct Day04 {
    // numbers[i] -> ith game
    // numbers[i][0] -> list of winning numbers
    // numbers[i][1] -> your numbers
    numbers: Vec<[HashSet<usize>; 2]>,
}

impl AoCProblem for Day04 {
    fn prepare(input: String) -> Self {
        let mut numbers = vec![];
        for line in input.lines() {
            let (_, num_list) = line.split_once(": ").unwrap();
            let (winning_numbers, your_numbers) = num_list.split_once(" | ").unwrap();
            numbers.push([
                winning_numbers
                    .split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
                your_numbers
                    .split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
            ])
        }

        Self { numbers }
    }

    fn part1(&mut self) -> Solution {
        let mut points = 0;
        for [winning_nums, your_nums] in &self.numbers {
            let num_related = winning_nums.intersection(your_nums).count() as u32;
            if num_related != 0 {
                points += 2_usize.pow(num_related - 1);
            }
        }

        points.into()
    }

    fn part2(&mut self) -> Solution {
        let mut num_cards: HashMap<usize, usize> = HashMap::new();
        // Set 1 for each initial card since these are the originals.
        for c in 0..self.numbers.iter().len() {
            num_cards.insert(c, 1);
        }

        for (card_idx, [winning_nums, your_nums]) in self.numbers.iter().enumerate() {
            let num_related = winning_nums.intersection(your_nums).count();
            let this_card_amt = *num_cards.entry(card_idx).or_default();
            for i in 0..num_related {
                *num_cards.entry(card_idx + i + 1).or_default() += this_card_amt;
            }
        }

        num_cards.values().copied().sum::<usize>().into()
    }

    fn day() -> u32 {
        4
    }

    fn year() -> u32 {
        2023
    }
}
