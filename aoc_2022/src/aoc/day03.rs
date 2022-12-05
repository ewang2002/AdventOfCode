use std::collections::HashSet;

use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day03 {
    rucksacks: Vec<Vec<char>>,
}

impl AoCProblem for Day03 {
    fn prepare(input: &str) -> Self {
        Self {
            rucksacks: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut priorities = 0;
        for r in &self.rucksacks {
            let comp_a: HashSet<_> = HashSet::from_iter(r[..r.len() / 2].iter().cloned());
            let comp_b: HashSet<_> = HashSet::from_iter(r[r.len() / 2..].iter().cloned());
            let res: HashSet<&_> = comp_a.intersection(&comp_b).collect();
            if res.len() != 1 {
                panic!("Either no unique elements or multiple unique elements.");
            }

            priorities += calculate_priority(*res.into_iter().next().unwrap());
        }

        priorities.into()
    }

    fn part2(&mut self) -> Solution {
        let mut priorities = 0;
        for group in self.rucksacks.chunks(3) {
            let mut unique_elements: HashSet<_> = HashSet::from_iter(group[0].iter().cloned());
            let all_sets: &[HashSet<_>] = &[
                HashSet::from_iter(group[1].iter().cloned()),
                HashSet::from_iter(group[2].iter().cloned()),
            ];

            // We can also chain the sets using '&', e.g.,
            //          &(&set1 & &set2) & &set3
            for set in all_sets {
                unique_elements.retain(|e| set.contains(e));
            }

            if unique_elements.len() != 1 {
                panic!("Either no unique elements or multiple unique elements.");
            }

            priorities += calculate_priority(unique_elements.into_iter().next().unwrap());
        }

        priorities.into()
    }
}

/// Calculates the priority of the character according to the given prompt.
/// That is,
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
///
/// # Parameters
/// - `c`: the character.
///
/// # Returns
/// The priority of that character.
#[inline(always)]
fn calculate_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 96,
        'A'..='Z' => c as usize - 65 + 27,
        _ => unreachable!(),
    }
}
