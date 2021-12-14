use std::collections::HashMap;
use crate::aoc::aoc_problem::AoCProblem;

pub struct Day14 {
    polymer_template: String,
    polymer_rules: HashMap<String, char>
}

impl AoCProblem<usize, usize> for Day14 {
    fn prepare(input: Vec<String>) -> Self {
        let mut polymer_rules: HashMap<String, char> = HashMap::new();
        for (rule, res) in input.iter().skip(2).map(|x| x.split_once(" -> ").unwrap()) {
            polymer_rules.insert(rule.to_string(), res.chars().nth(0).unwrap());
        }

        Self {
            polymer_template: input[0].to_string(),
            polymer_rules
        }
    }

    fn part1(&self) -> usize {
        let mut polymer = self.polymer_template.chars().collect::<Vec<_>>();
        for _ in 0..10 {
            let mut new_polymer: Vec<char> = vec![];
            for c in polymer.windows(2) {
                new_polymer.push(c[0]);
                new_polymer.push(*self.polymer_rules.get(&format!("{}{}", c[0], c[1])).unwrap());
            }

            new_polymer.push(polymer[polymer.len() - 1]);
            polymer = new_polymer;
        }

        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in polymer {
            *counts.entry(c).or_insert(0) += 1;
        }

        let (_, max_ct) = counts.iter().max_by_key(|&(_, count)| count).unwrap();
        let (_, min_ct) = counts.iter().min_by_key(|&(_, count)| count).unwrap();

        max_ct - min_ct
    }

    fn part2(&self) -> usize {
        0
    }
}