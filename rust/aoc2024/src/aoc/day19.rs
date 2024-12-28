use common::constants::TWO_NEWLINE;
use common::problem::day::{AoCProblem, Solution};
use std::collections::{HashMap, HashSet};

pub struct Day19 {
    towel_patterns: HashSet<String>,
    desired_designs: Vec<String>,
}

impl AoCProblem for Day19 {
    fn prepare(input: String) -> Self {
        let (raw_towel, raw_designs) = input.split_once(TWO_NEWLINE).unwrap();

        Self {
            towel_patterns: raw_towel.split(", ").map(String::from).collect(),
            desired_designs: raw_designs.lines().map(String::from).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        self.desired_designs
            .iter()
            .filter(|d| count_arrangements(&self.towel_patterns, d.as_str()) > 0)
            .count()
            .into()
    }

    fn part2(&mut self) -> Solution {
        self.desired_designs
            .iter()
            .map(|d| count_arrangements(&self.towel_patterns, d.as_str()))
            .sum::<usize>()
            .into()
    }

    fn day() -> u32 {
        19
    }

    fn year() -> u32 {
        2024
    }
}

/// Counts the number of possible towel designs that can be created from `desired_design` using
/// the designs defined in `towel_patterns`.
///
/// # Parameters
/// - `towel_patterns`: A set of all available towels.
/// - `desired_design`: A string representing a towel where you'd like to see how many possible
///                     towel designs can be created.
///
/// # Returns
/// The number of possible towel designs that can be created.
fn count_arrangements(towel_patterns: &HashSet<String>, desired_design: &str) -> usize {
    fn count<'a>(
        towel_patterns: &HashSet<String>,
        remaining: &'a str,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if remaining.is_empty() {
            1
        } else if let Some(c) = cache.get(remaining) {
            *c
        } else {
            let mut ct = 0;
            for t in towel_patterns {
                if remaining.starts_with(t) {
                    ct += count(towel_patterns, &remaining[t.len()..], cache);
                }
            }

            cache.insert(remaining, ct);
            ct
        }
    }

    count(towel_patterns, desired_design, &mut HashMap::new())
}
