use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::aoc::aoc_problem::AoCProblem;

const A: i32 = 1 << 0;
const B: i32 = 1 << 1;
const C: i32 = 1 << 2;
const D: i32 = 1 << 3;
const E: i32 = 1 << 4;
const F: i32 = 1 << 5;
const G: i32 = 1 << 6;

pub struct Day08 {
    pattern_res: Vec<SegmentDisplayEntry>,
}

// https://adventofcode.com/2021/day/8
impl AoCProblem<usize, i32> for Day08 {
    fn prepare(input: Vec<String>) -> Self {
        return Day08 {
            pattern_res: input.iter().map(|x| {
                let s = x.split(" | ").collect::<Vec<_>>();
                return SegmentDisplayEntry {
                    signal_pattern: s[0].split(" ").map(|z| z.to_string()).collect::<_>(),
                    output_value: s[1].split(" ").map(|z| z.to_string()).collect::<_>(),
                };
            }).collect::<_>()
        };
    }

    fn part1(&self) -> usize {
        return self.pattern_res.iter().flat_map(|x| &x.output_value)
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
    }

    fn part2(&self) -> i32 {
        let get_val = |c: char| -> i32 {
            match c {
                'a' => A,
                'b' => B,
                'c' => C,
                'd' => D,
                'e' => E,
                'f' => F,
                'g' => G,
                _ => panic!("Unknown character {}", c)
            }
        };

        let pos_finder = |v: &Vec<&String>, against: &HashSet<char>, target_len: usize| -> usize {
            v.iter()
                .position(|x| x.chars()
                    .collect::<HashSet<_>>()
                    .intersection(against)
                    .count() == target_len
                ).unwrap()
        };

        let mut final_sum = 0;
        for entry in &self.pattern_res {
            // 1, 4, 7, 8 each have a unique number of segments
            // Use them to build the baseline map.

            // temp_segments: key -> [0, 9], value = [...]
            let mut temp_segments: HashMap<i32, HashSet<char>> = HashMap::new();

            let mut base = entry.signal_pattern.iter()
                .filter(|x| match x.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false
                }).collect::<Vec<_>>();
            base.sort_by(|x, y| {
                if x.len() < y.len() {
                    return Ordering::Less;
                }

                if x.len() > y.len() {
                    return Ordering::Greater;
                }

                return Ordering::Equal;
            });

            temp_segments.insert(1, base[0].chars().collect::<_>());
            temp_segments.insert(7, base[1].chars().collect::<_>());
            temp_segments.insert(4, base[2].chars().collect::<_>());
            temp_segments.insert(8, base[3].chars().collect::<_>());

            // Now that we've added the baseline maps, we need to construct the other numbers
            // Start with either 6, 9, 0
            let mut second_longest = entry.signal_pattern.iter()
                .filter(|x| x.len() == 6)
                .collect::<Vec<_>>();

            // The intersection of chars representing [9] and 4 (which we know) will give us a char
            // vector of length 4. Use this to find what 9 is.
            let nine_idx = pos_finder(
                &second_longest,
                temp_segments.get(&4).unwrap(),
                4
            );

            temp_segments.insert(9, second_longest[nine_idx].chars().collect::<_>());
            second_longest.remove(nine_idx);

            // The intersection of chars representing [0] and 1 (which we know) will give us a char
            // vector of length 2. Use this to find what 0 is
            let zero_idx = pos_finder(
                &second_longest,
                temp_segments.get(&1).unwrap(),
                2
            );

            temp_segments.insert(0, second_longest[zero_idx].chars().collect::<_>());
            second_longest.remove(zero_idx);

            // The last element must be 6, then.
            temp_segments.insert(6, second_longest[0].chars().collect::<_>());

            // Last thing we need to do is find 5, 2, 3.
            let mut shortest = entry.signal_pattern.iter()
                .filter(|x| x.len() == 5)
                .collect::<Vec<_>>();

            // The intersection of chars representing [3] and 1 (which we know) will give us a char
            // vector of length 2. Use this to find what 3 is.
            let three_idx = pos_finder(
                &shortest,
                temp_segments.get(&1).unwrap(),
                2
            );
            temp_segments.insert(3, shortest[three_idx].chars().collect::<_>());
            shortest.remove(three_idx);

            // The intersection of chars representing [5] and 4 (which we know) will give us a char
            // vector of length 3. Use this to find what 5 is.
            let five_idx = pos_finder(
                &shortest,
                temp_segments.get(&4).unwrap(),
                3
            );
            temp_segments.insert(5, shortest[five_idx].chars().collect::<_>());
            shortest.remove(five_idx);

            // Which means the last number to be found is 2
            temp_segments.insert(2, shortest[0].chars().collect::<_>());

            // Map each number's character vec ID to the number
            let mut segment_map: HashMap<i32, i32> = HashMap::new();
            for (k, v) in temp_segments {
                let mut seg_id: i32 = 0;
                for c in v {
                    seg_id |= get_val(c);
                }

                segment_map.insert(seg_id, k);
            }

            // Finally, find the numbers
            let mut temp_sum = 0;
            let mut mult_by = 1000;
            for output in &entry.output_value {
                let mut seg_id = 0;
                for c in output.chars() {
                    seg_id |= get_val(c);
                }

                let parsed_num = segment_map
                    .get(&seg_id)
                    .expect("Invalid value found");
                temp_sum += *parsed_num * mult_by;
                mult_by /= 10;
            }

            final_sum += temp_sum;
        }

        return final_sum;
    }
}

struct SegmentDisplayEntry {
    signal_pattern: Vec<String>,
    output_value: Vec<String>,
}