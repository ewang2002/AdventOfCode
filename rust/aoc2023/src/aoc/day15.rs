use common::problem::day::{AoCProblem, Solution};

pub struct Day15 {
    steps: Vec<String>,
}

impl AoCProblem for Day15 {
    fn prepare(input: String) -> Self {
        Self {
            steps: input.split(',').map(|s| s.to_string()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut sum_hash = 0;

        for step in &self.steps {
            sum_hash += hash(step);
        }

        sum_hash.into()
    }

    fn part2(&mut self) -> Solution {
        let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

        for step in &self.steps {
            let idx = step.chars().position(|c| c == '-' || c == '=').unwrap();
            let label = &step[..idx];
            let hash = hash(label);
            if step.contains('-') {
                boxes[hash]
                    .iter()
                    .position(|(l, _)| l == &label)
                    .map(|idx| boxes[hash].remove(idx));
            } else {
                let (_, raw_val) = step.split_once('=').unwrap();
                let val = raw_val.parse::<usize>().unwrap();
                boxes[hash]
                    .iter()
                    // Find the position of the label, if it exists
                    .position(|(l, _)| l == &label)
                    // If it does exist, use the index to set the value
                    .map(|idx| boxes[hash][idx].1 = val)
                    // Otherwise, push a new entry
                    .unwrap_or_else(|| boxes[hash].push((label, val)));
            }
        }

        let mut focusing_power = 0;
        for (box_idx, b) in boxes.iter().enumerate() {
            focusing_power += b
                .iter()
                .enumerate()
                .map(|(slot_idx, (_, val))| (box_idx + 1) * (slot_idx + 1) * *val)
                .sum::<usize>();
        }

        focusing_power.into()
    }

    fn day() -> u32 {
        15
    }

    fn year() -> u32 {
        2023
    }
}

/// Runs the HASH algorithm on the given string.
///
/// # Parameters
/// - `s`: The string to hash.
///
/// # Returns
/// The hash of the given string.
fn hash(s: &str) -> usize {
    let mut hash: usize = 0;
    for c in s.as_bytes() {
        hash += *c as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}
