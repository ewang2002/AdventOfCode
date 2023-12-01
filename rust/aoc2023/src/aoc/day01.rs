use common::problem::day::{AoCProblem, Solution};

const NUM_MAPPING: [(&str, u32); 10] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
];

pub struct Day01 {
    calibration_doc: Vec<String>,
}

impl AoCProblem for Day01 {
    fn prepare(input: String) -> Self {
        Self {
            calibration_doc: input.lines().map(|l| l.to_string()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        self.calibration_doc
            .iter()
            .map(|data| {
                let lhs = data
                    .chars()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();

                let rhs = data
                    .chars()
                    .rfind(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();

                lhs * 10 + rhs
            })
            .sum::<u32>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        let mut sum = 0;
        let mut lhs = String::with_capacity(200);
        let mut rhs = String::with_capacity(200);
        for line in &self.calibration_doc {
            let mut digits = 0;
            for c in line.chars() {
                if c.is_ascii_digit() {
                    digits = c.to_digit(10).unwrap() * 10;
                    break;
                } else {
                    lhs.push(c);
                    if let Some(n) = extract_num_from_str(&lhs) {
                        digits = n * 10;
                        break;
                    }
                }
            }

            for c in line.chars().rev() {
                if c.is_ascii_digit() {
                    digits += c.to_digit(10).unwrap();
                    break;
                } else {
                    rhs.insert(0, c);
                    if let Some(n) = extract_num_from_str(&rhs) {
                        digits += n;
                        break;
                    }
                }
            }

            sum += digits;
            lhs.clear();
            rhs.clear();
        }

        sum.into()
    }

    fn day() -> u32 {
        1
    }

    fn year() -> u32 {
        2023
    }
}

/// Attempts to extract a number (spelled out) from the given string.
///
/// # Parameters
/// - `s`: The string.
///
/// # Returns
/// Either the number, if it exists, or None if no number exists.
fn extract_num_from_str(s: &str) -> Option<u32> {
    NUM_MAPPING
        .into_iter()
        .find(|(spelling, _)| s.contains(spelling))
        .map(|(_, num)| num)
}
