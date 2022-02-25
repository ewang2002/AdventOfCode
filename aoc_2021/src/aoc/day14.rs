use crate::aoc::aoc_problem::AoCProblem;
use std::collections::HashMap;

// The plan that I chose to use is as follows:
// - Consider the following polymer:
//          NCNBCHB
// - [W] First, we note that the last character of this polymer is 'B'. No matter how many times we
// want to strengthen the polymer, we will have the same last character. This can be generalized to
// any polymer.
//
// - [X] We can break this polymer up into pairs, like so:
//          ['NC', 'CN', 'NB', 'BC', 'CH', 'HB']
//
// - This can be mapped into a map like so:
//          NC => 1
//          CN => 1
//          NB => 1
//          BC => 1
//          CH => 1
//          HB => 1
// Where the key is the pair and the value is the number of occurrences of that pair.
//
// - Assuming that these are put in random order (since hashing doesn't guarantee order). Then, we
// can map the pairs to the corresponding letter based on the polymer instruction mapping:
//          NC -> B
//          CN -> C
//          NB -> B
//          BC -> B
//          CH -> B
//          HB -> C
//
// - From this, we can create a new map with the new polymers, like so:
//          PAIRING             C1 C2
//          -------------------------
//          NC -> B             NB BC
//          CN -> C             CC CN
//          NB -> B             NB BB
//          BC -> B             BB BC
//          CH -> B             CB BH
//          HB -> C             HC CB
//
// - To count the number of actual characters in this polymer, we note that if we simply count the
// number of characters in each of C1 and C2, we would be over-counting since the first character
// of C2 is the same as the last character of C1 and the last character of C2 is the same as the
// first character in the next row of C1, e.g. for NB and BC, notice how 'B' is shared between the
// two pairs and how 'C' is shared between BC and CC (next row).
//
// So, the solution is to have two maps.
// 1) Our first map would track the number of pairs we have in all of C1 and C2 above, just like
// what we did in [X]. We need this if we want to strengthen the polymer again. In other words, for
// the above C1 and C2, we would have the following mapping:
//      {NB: 2, BC: 2, CC: 1, CN: 1, BB: 2, CB: 2, BH: 1, HC: 1}
// This mapping represents the next polymer, should we have to strengthen the polymer again.
//
// 2) Our second map would track the number of characters that we have in C1 only. But, this would
// omit the last character in the polymer. However, since we know that every polymer and its
// derivations will share the last character (see [W]), we can simply add one to the last
// character in the mapping. In other words, we would only consider the following pairs:
//      [NB, CC, NB, BB, CB, HC, B]
//                               ^ (Last character in polymer)
// Which gives us the following number of each character in the next iteration of the polymer:
//      {N: 2, B: 6, C: 4, H: 1}
// We can use this to our advantage, as seen in the implementation below.

type Pair = [char; 2];

pub struct Day14 {
    polymer_template_pairing: HashMap<Pair, usize>,
    polymer_rules: HashMap<Pair, char>,
    last_polymer_char: char,
}

// https://adventofcode.com/2021/day/14
impl AoCProblem<usize, usize> for Day14 {
    fn prepare(input: Vec<String>) -> Self {
        let mut polymer_rules: HashMap<Pair, char> = HashMap::new();
        for (rule, res) in input.iter().skip(2).map(|x| x.split_once(" -> ").unwrap()) {
            let c = rule.chars().collect::<Vec<_>>();
            polymer_rules.insert([c[0], c[1]], res.chars().next().unwrap());
        }

        let mut polymer_template_pairing: HashMap<Pair, usize> = HashMap::new();
        let polymer = input[0].chars().collect::<Vec<_>>();
        for w in polymer.windows(2) {
            *polymer_template_pairing.entry([w[0], w[1]]).or_insert(0) += 1;
        }

        Self {
            polymer_template_pairing,
            polymer_rules,
            last_polymer_char: *polymer.iter().last().unwrap(),
        }
    }

    fn part1(&self) -> usize {
        get_max_min_quantity_diff(self, 10)
    }

    fn part2(&self) -> usize {
        get_max_min_quantity_diff(self, 40)
    }
}

/// Gets the difference between the most and least occurring characters in the polymer after
/// strengthening the polymer `amt` number of times.
///
/// # Parameters
/// - `p_struct`: The Day14 problem structure.
/// - `amt`: The number of times to strengthen the polymer, i.e. run the frequency function.
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
