use std::collections::{HashMap};
use crate::aoc::aoc_problem::AoCProblem;

pub struct Day14 {
    polymer_template: String,
    polymer_rules: HashMap<[char; 2], char>,
}

impl AoCProblem<usize, usize> for Day14 {
    fn prepare(input: Vec<String>) -> Self {
        let mut polymer_rules: HashMap<[char; 2], char> = HashMap::new();
        for (rule, res) in input.iter().skip(2).map(|x| x.split_once(" -> ").unwrap()) {
            let c = rule.chars().collect::<Vec<_>>();
            polymer_rules.insert([c[0], c[1]], res.chars().nth(0).unwrap());
        }

        Self {
            polymer_template: input[0].to_string(),
            polymer_rules,
        }
    }


    fn part1(&self) -> usize {
        let mut frequency: HashMap<[char; 2], usize> = HashMap::new();
        let polymer = self.polymer_template.chars().collect::<Vec<_>>();
        for w in polymer.windows(2) {
            *frequency.entry([w[0], w[1]]).or_insert(0) += 1;
        }

        let mut count: HashMap<char, usize> = HashMap::new();
        for _ in 0..10 {
            let (freq, ct) = get_frequency_of_pairs(frequency, &self.polymer_rules);
            frequency = freq;
            count = ct;
        }

        *count.get_mut(polymer.iter().last().unwrap()).unwrap() += 1;

        let (_, max_ct) = count.iter().max_by_key(|&(_, count)| count).unwrap();
        let (_, min_ct) = count.iter().min_by_key(|&(_, count)| count).unwrap();

        max_ct - min_ct
    }


    fn part2(&self) -> usize {
        let mut frequency: HashMap<[char; 2], usize> = HashMap::new();
        let polymer = self.polymer_template.chars().collect::<Vec<_>>();
        for w in polymer.windows(2) {
            *frequency.entry([w[0], w[1]]).or_insert(0) += 1;
        }

        let mut count: HashMap<char, usize> = HashMap::new();
        for _ in 0..40 {
            let (freq, ct) = get_frequency_of_pairs(frequency, &self.polymer_rules);
            frequency = freq;
            count = ct;
        }

        *count.get_mut(polymer.iter().last().unwrap()).unwrap() += 1;

        let (_, max_ct) = count.iter().max_by_key(|&(_, count)| count).unwrap();
        let (_, min_ct) = count.iter().min_by_key(|&(_, count)| count).unwrap();

        max_ct - min_ct
    }
}

/// Gets the frequency of each pair that appears in the initial map.
///
/// # Parameters
/// - `initial`: The initial polymer, decomposed as pairs.
/// - `mapping`: The polymer mapping.
///
/// # Returns
/// A tuple containing a hashmap representing the new pairings, and another hashmap consisting of
/// the count of each character (excluding the last character that appeared in the original
/// polymer).
fn get_frequency_of_pairs(
    initial: HashMap<[char; 2], usize>,
    mapping: &HashMap<[char; 2], char>,
) -> (HashMap<[char; 2], usize>, HashMap<char, usize>) {
    let mut new_freq: HashMap<[char; 2], usize> = HashMap::new();
    let mut count: HashMap<char, usize> = HashMap::new();

    for (pair, ct) in initial {
        let mapped_char = mapping.get(&pair).unwrap();
        *new_freq.entry([pair[0], *mapped_char]).or_insert(0) += ct;
        *new_freq.entry([*mapped_char, pair[1]]).or_insert(0) += ct;
        *count.entry(pair[0]).or_insert(0) += ct;
        *count.entry(*mapped_char).or_insert(0) += ct;
    }

    (new_freq, count)
}