use crate::aoc::aoc_problem::AoCProblem;
use crate::helpers::vec_arr::transpose_vec;
use std::collections::{HashMap, HashSet};

pub struct Day03 {
    input: Vec<String>,
}

// https://adventofcode.com/2021/day/3
impl AoCProblem<u32, u32> for Day03 {
    fn prepare(input: Vec<String>) -> Self {
        Self { input }
    }

    fn part1(&self) -> u32 {
        let binary_str_vec = transpose_vec(
            &self
                .input
                .iter()
                .map(|x| x.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let mut gamma_rate: u32 = 0;
        let mut epsilon_rate: u32 = 0;

        for vec in binary_str_vec {
            let mut num_ones = 0;
            let mut num_zeros = 0;
            vec.iter().for_each(|x| {
                if *x == '0' {
                    num_zeros += 1;
                } else {
                    num_ones += 1;
                }
            });

            gamma_rate = (gamma_rate << 1) | if num_ones > num_zeros { 1 } else { 0 };
            epsilon_rate = (epsilon_rate << 1) | if num_ones > num_zeros { 0 } else { 1 };
        }

        gamma_rate * epsilon_rate
    }

    fn part2(&self) -> u32 {
        let oxygen_generator_val =
            get_elem_by_bit_criteria(&self.input, |num_zero, num_one, this_char| {
                (num_one == num_zero && this_char != '1')
                    || (num_one > num_zero && this_char != '1')
                    || (num_zero > num_one && this_char != '0')
            });

        let co2_scrubber_rating =
            get_elem_by_bit_criteria(&self.input, |num_zero, num_one, this_char| {
                (num_one == num_zero && this_char != '0')
                    || (num_one > num_zero && this_char != '0')
                    || (num_zero > num_one && this_char != '1')
            });

        oxygen_generator_val * co2_scrubber_rating
    }
}

/// Gets the binary number that matches the given criteria. Used in part 2.
///
/// # Parameters
/// - `input`: The diagnostic report (puzzle input).
/// - `criteria`: A function that takes in the number of zeros, number of ones, and the character
/// and returns a boolean value determining what numbers to keep and what numbers to remove.
///
/// # Returns
/// The number that is left after processing all binary numbers.
fn get_elem_by_bit_criteria<F>(input: &[String], criteria: F) -> u32
where
    F: Fn(i32, i32, char) -> bool,
{
    let mut map: HashMap<&String, Vec<char>> = HashMap::new();
    for elem in input {
        map.insert(elem, elem.chars().collect::<Vec<_>>());
    }

    let mut i: usize = 0;
    while map.len() > 1 {
        let vec_to_check = map.values().collect::<Vec<_>>();
        let mut num_ones = 0;
        let mut num_zeros = 0;
        get_elems_at_idx(vec_to_check, i).iter().for_each(|x| {
            if *x == '0' {
                num_zeros += 1;
            } else {
                num_ones += 1;
            }
        });

        let mut keys_to_delete: HashSet<&String> = HashSet::new();
        for (key, vecs) in &map {
            if criteria(num_zeros, num_ones, vecs[i]) {
                keys_to_delete.insert(key);
            }
        }

        for key in keys_to_delete {
            map.remove(key);
        }

        i += 1;
    }

    u32::from_str_radix(map.keys().next().unwrap(), 2).unwrap()
}

/// Gets an element from each vector at a specified index.
///
/// # Parameters
/// - `vectors`: The vector of vector of characters.
/// - `idx`: The index to grab the element from.
///
/// # Returns
/// The vector containing the element from each vector at index `idx`.
fn get_elems_at_idx(vectors: Vec<&Vec<char>>, idx: usize) -> Vec<char> {
    vectors.iter().map(|x| x[idx]).collect::<Vec<_>>()
}
