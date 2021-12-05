use std::collections::{HashMap, HashSet};
use crate::aoc::aoc_problem::AoCProblem;
use crate::helpers::vec_arr::transpose_vec;

pub struct Day03 {
    input: Vec<String>,
}

impl AoCProblem<u32, u32> for Day03 {
    fn prepare(input: Vec<String>) -> Self {
        return Day03 { input };
    }

    // --- Day 3: Binary Diagnostic ---
    // The submarine has been making some odd creaking noises, so you ask it to produce a
    // diagnostic report just in case.
    //
    // The diagnostic report (your puzzle input) consists of a list of binary numbers which, when
    // decoded properly, can tell you many useful things about the conditions of the submarine. The
    // first parameter to check is the power consumption.
    //
    // You need to use the binary numbers in the diagnostic report to generate two new binary
    // numbers (called the gamma rate and the epsilon rate). The power consumption can then be
    // found by multiplying the gamma rate by the epsilon rate.
    //
    // Each bit in the gamma rate can be determined by finding the most common bit in the
    // corresponding position of all numbers in the diagnostic report. For example, given the
    // following diagnostic report:
    //
    //  00100
    //  11110
    //  10110
    //  10111
    //  10101
    //  01111
    //  00111
    //  11100
    //  10000
    //  11001
    //  00010
    //  01010
    //
    // Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since
    // the most common bit is 1, the first bit of the gamma rate is 1.
    //
    // The most common second bit of the numbers in the diagnostic report is 0, so the second bit
    // of the gamma rate is 0.
    //
    // The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively,
    // and so the final three bits of the gamma rate are 110.
    //
    // So, the gamma rate is the binary number 10110, or 22 in decimal.
    //
    // The epsilon rate is calculated in a similar way; rather than use the most common bit, the
    // least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal.
    // Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
    //
    // Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon
    // rate, then multiply them together. What is the power consumption of the submarine? (Be sure
    // to represent your answer in decimal, not binary.)
    fn part1(&self) -> u32 {
        let binary_str_vec = transpose_vec(&self.input.iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>());

        let mut gamma_rate: u32 = 0;
        let mut epsilon_rate: u32 = 0;

        for vec in binary_str_vec {
            let mut num_ones = 0;
            let mut num_zeros = 0;
            vec.iter()
                .for_each(|x| if *x == '0' { num_zeros += 1; } else { num_ones += 1; });

            gamma_rate = (gamma_rate << 1) | if num_ones > num_zeros { 1 } else { 0 };
            epsilon_rate = (epsilon_rate << 1) | if num_ones > num_zeros { 0 } else { 1 };
        }

        return gamma_rate * epsilon_rate;
    }

    // --- Part Two ---
    // Next, you should verify the life support rating, which can be determined by multiplying the
    // oxygen generator rating by the CO2 scrubber rating.
    //
    // Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in
    // your diagnostic report - finding them is the tricky part. Both values are located using a
    // similar process that involves filtering out values until only one remains. Before searching
    // for either rating value, start with the full list of binary numbers from your diagnostic
    // report and consider just the first bit of those numbers. Then:
    //
    // - Keep only numbers selected by the bit criteria for the type of rating value for which you
    // are searching. Discard numbers which do not match the bit criteria.
    // - If you only have one number left, stop; this is the rating value for which you are
    // searching.
    // - Otherwise, repeat the process, considering the next bit to the right.
    //
    // The bit criteria depends on which type of rating value you want to find:
    // - To find oxygen generator rating, determine the most common value (0 or 1) in the current
    // bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally
    // common, keep values with a 1 in the position being considered.
    // - To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit
    // position, and keep only numbers with that bit in that position. If 0 and 1 are equally
    // common, keep values with a 0 in the position being considered.
    //
    // For example, to determine the oxygen generator rating value using the same example
    // diagnostic report from above:
    //
    // - Start with all 12 numbers and consider only the first bit of each number. There are more
    // 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position:
    // 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
    // - Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than
    // 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101,
    // and 10000.
    // - In the third position, three of the four numbers have a 1, so keep those three: 10110,
    // 10111, and 10101.
    // - In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and
    // 10111.
    // - In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to
    // find the oxygen generator rating, keep the number with a 1 in that position: 10111.
    // - As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in
    // decimal.
    //
    // Then, to determine the CO2 scrubber rating value from the same example above:
    // - Start again with all 12 numbers and consider only the first bit of each number. There are
    // fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position:
    // 00100, 01111, 00111, 00010, and 01010.
    // - Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than
    // 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
    // - In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to
    // find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
    // - As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in
    // decimal.
    // - Finally, to find the life support rating, multiply the oxygen generator rating (23) by the
    // CO2 scrubber rating (10) to get 230.
    //
    // Use the binary numbers in your diagnostic report to calculate the oxygen generator rating
    // and CO2 scrubber rating, then multiply them together. What is the life support rating of
    // the submarine? (Be sure to represent your answer in decimal, not binary.)
    fn part2(&self) -> u32 {
        let oxygen_generator_val = get_elem_by_bit_criteria(
            &self.input,
            |num_zero, num_one, this_char| {
                return (num_one == num_zero && this_char != '1')
                    || (num_one > num_zero && this_char != '1')
                    || (num_zero > num_one && this_char != '0');
            },
        );

        let co2_scrubber_rating = get_elem_by_bit_criteria(
            &self.input,
            |num_zero, num_one, this_char| {
                return (num_one == num_zero && this_char != '0')
                    || (num_one > num_zero && this_char != '0')
                    || (num_zero > num_one && this_char != '1');
            },
        );

        return oxygen_generator_val * co2_scrubber_rating;
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
fn get_elem_by_bit_criteria<F>(input: &Vec<String>, criteria: F) -> u32
    where F: Fn(i32, i32, char) -> bool {
    let mut map: HashMap<&String, Vec<char>> = HashMap::new();
    for elem in input {
        map.insert(elem, elem.chars().collect::<Vec<_>>());
    }

    let mut i: usize = 0;
    while map.len() > 1 {
        let vec_to_check = map.values().collect::<Vec<_>>();
        let mut num_ones = 0;
        let mut num_zeros = 0;
        get_elems_at_idx(vec_to_check, i)
            .iter()
            .for_each(|x| if *x == '0' { num_zeros += 1; } else { num_ones += 1; });

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

    return u32::from_str_radix(map.keys().nth(0).unwrap(), 2).unwrap();
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
    return vectors.iter().map(|x| x[idx]).collect::<Vec<_>>();
}