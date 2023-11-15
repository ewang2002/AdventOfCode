use common::problem::day::{AoCProblem, Solution};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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
impl AoCProblem for Day08 {
    fn prepare(input: String) -> Self {
        return Day08 {
            pattern_res: input
                .lines()
                .map(|x| {
                    let s = x.split(" | ").collect::<Vec<_>>();
                    return SegmentDisplayEntry {
                        signal_pattern: s[0].split(' ').map(|z| z.to_string()).collect::<_>(),
                        output_value: s[1].split(' ').map(|z| z.to_string()).collect::<_>(),
                    };
                })
                .collect::<_>(),
        };
    }

    fn part1(&mut self) -> Solution {
        return self
            .pattern_res
            .iter()
            .flat_map(|x| &x.output_value)
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count()
            .into();
    }

    fn part2(&mut self) -> Solution {
        let get_val = |c: char| -> i32 {
            match c {
                'a' => A,
                'b' => B,
                'c' => C,
                'd' => D,
                'e' => E,
                'f' => F,
                'g' => G,
                _ => panic!("Unknown character {}", c),
            }
        };

        let mut final_sum = 0;
        for entry in &self.pattern_res {
            // 1, 4, 7, 8 each have a unique number of segments
            // Use them to build the baseline map.

            // temp_segments: key -> [0, 9], value = [...]
            let mut temp_segments: HashMap<i32, HashSet<char>> = HashMap::new();

            let mut base = entry
                .signal_pattern
                .iter()
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .collect::<Vec<_>>();
            base.sort_by(|x, y| {
                if x.len() < y.len() {
                    return Ordering::Less;
                }

                if x.len() > y.len() {
                    return Ordering::Greater;
                }

                Ordering::Equal
            });

            temp_segments.insert(1, base[0].chars().collect::<_>());
            temp_segments.insert(7, base[1].chars().collect::<_>());
            temp_segments.insert(4, base[2].chars().collect::<_>());
            temp_segments.insert(8, base[3].chars().collect::<_>());

            // Borrow checker really making me do this
            let one_set: HashSet<char> = base[0].chars().collect();
            let seven_set: HashSet<char> = base[1].chars().collect();
            let four_set: HashSet<char> = base[2].chars().collect();
            let eight_set: HashSet<char> = base[3].chars().collect();

            // The idea is that, given an initial number (1, 4, 7, 8) and an unknown number, we can
            // compare the number of overlapping segments and use that to figure out which numbers
            // we're working with.
            //
            // For example, if we wrote out the segments for 1, 4, 7, 8, we know that the number
            // 2 would overlap 1's segments 1 time; 2 would overlap 4's segments 2 times; 2 would
            // overlap 7's segments 2 times; and 2 would overlap 8's segments 5 times.
            //
            // By repeatedly applying this to every unknown number, we get the following table:
            //
            //  ∩ | 1   4   7   8
            //  ------------------
            //  2 | 1   2   2   5
            //  3 | 2   3   3   5
            //  5 | 1   3   2   5
            //  6 | 1   3   2   6
            //  9 | 2   4   3   6
            //  0 | 2   3   3   6
            //
            // For example, if we take the row with value 5, this table says that:
            // - 5's segments "intersect" with 1's segments 1 time.
            // - 5's segments "intersect" with 4's segments 3 time.
            // - 5's segments "intersect" with 7's segments 2 time.
            // - 5's segments "intersect" with 8's segments 5 time.
            //
            // By intersection, consider this 5:
            //
            //  dddd
            // e
            // e
            //  ffff
            //      b
            //      b
            //  cccc
            //
            // And consider this 4:
            //
            // e    a
            // e    a
            //  ffff
            //      b
            //      b
            //
            // If we were to take the "intersection" of the segments represented by these two
            // numbers, we would get:
            //
            // e
            // e
            //  ffff
            //      b
            //      b
            //
            // Which means that there are 3 segments in the intersection (or, another way of
            // putting it is there are 3 segments between the two numbers that overlap).

            for sig in &entry.signal_pattern {
                let set: HashSet<_> = sig.chars().collect();
                let tuple = (
                    set.intersection(&one_set).count(),
                    set.intersection(&four_set).count(),
                    set.intersection(&seven_set).count(),
                    set.intersection(&eight_set).count(),
                );

                match tuple {
                    (1, 2, 2, 5) => temp_segments.insert(2, set),
                    (2, 3, 3, 5) => temp_segments.insert(3, set),
                    (1, 3, 2, 5) => temp_segments.insert(5, set),
                    (1, 3, 2, 6) => temp_segments.insert(6, set),
                    (2, 4, 3, 6) => temp_segments.insert(9, set),
                    (2, 3, 3, 6) => temp_segments.insert(0, set),
                    // Ignore initial values
                    (_, _, _, 2) | (_, _, _, 4) | (_, _, _, 3) | (_, _, _, 7) => continue,
                    _ => panic!("Error: {:?}", tuple),
                };
            }

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

                let parsed_num = segment_map.get(&seg_id).expect("Invalid value found");
                temp_sum += *parsed_num * mult_by;
                mult_by /= 10;
            }

            final_sum += temp_sum;
        }

        final_sum.into()
    }

    fn day() -> u32 {
        8
    }
}

struct SegmentDisplayEntry {
    signal_pattern: Vec<String>,
    output_value: Vec<String>,
}
