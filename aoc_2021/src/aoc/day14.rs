use std::collections::{HashMap};
use crate::aoc::aoc_problem::AoCProblem;

type Pair = [char; 2];

pub struct Day14 {
    polymer_template_pairing: HashMap<Pair, usize>,
    polymer_rules: HashMap<Pair, char>,
    last_polymer_char: char
}

impl AoCProblem<usize, usize> for Day14 {
    fn prepare(input: Vec<String>) -> Self {
        let mut polymer_rules: HashMap<Pair, char> = HashMap::new();
        for (rule, res) in input.iter().skip(2).map(|x| x.split_once(" -> ").unwrap()) {
            let c = rule.chars().collect::<Vec<_>>();
            polymer_rules.insert([c[0], c[1]], res.chars().nth(0).unwrap());
        }

        let mut polymer_template_pairing: HashMap<Pair, usize> = HashMap::new();
        let polymer = input[0].chars().collect::<Vec<_>>();
        for w in polymer.windows(2) {
            *polymer_template_pairing.entry([w[0], w[1]]).or_insert(0) += 1;
        }

        Self {
            polymer_template_pairing,
            polymer_rules,
            last_polymer_char: *polymer.iter().last().unwrap()
        }
    }


    fn part1(&self) -> usize {
        get_max_min_quantity_diff(self, 10)
    }


    fn part2(&self) -> usize {
        get_max_min_quantity_diff(self, 40)
    }
}

/// Gets the difference between the most and least occurring polymers.
///
/// # Parameters
/// - `p_struct`: The Day14 problem structure.
/// - `amt`: The number of times to run the frequency function.
///
/// # Returns
/// The difference between the most and least occurring polymers.
#[inline(always)]
fn get_max_min_quantity_diff(p_struct: &Day14, amt: usize) -> usize {
    let mut frequency: HashMap<Pair, usize> = p_struct.polymer_template_pairing.clone();
    let mut count: HashMap<char, usize> = HashMap::new();
    for _ in 0..amt {
        let (freq, ct) = get_frequency_of_pairs(frequency, &p_struct.polymer_rules);
        frequency = freq;
        count = ct;
    }

    *count.get_mut(&p_struct.last_polymer_char).unwrap() += 1;

    let (_, max_ct) = count.iter().max_by_key(|&(_, count)| count).unwrap();
    let (_, min_ct) = count.iter().min_by_key(|&(_, count)| count).unwrap();

    max_ct - min_ct
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
    initial: HashMap<Pair, usize>,
    mapping: &HashMap<Pair, char>,
) -> (HashMap<Pair, usize>, HashMap<char, usize>) {
    let mut new_freq: HashMap<Pair, usize> = HashMap::new();
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