use common::day::{AoCProblem, Solution};
use crate::intcode::{get_digits, parse_intcode, IntCodeComputer};
use std::cmp::max;

pub struct Day07 {
    program: Vec<isize>,
}

// https://adventofcode.com/2019/day/7
impl AoCProblem for Day07 {
    fn prepare(input: String) -> Self {
        Self {
            program: parse_intcode(input.lines().nth(0).unwrap()),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut computers: [IntCodeComputer; 5] = [
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
        ];

        let mut highest: isize = 0;

        // Is it brute-force time?
        for a in 0..=4 {
            for b in 0..=4 {
                for c in 0..=4 {
                    for d in 0..=4 {
                        for e in 0..=4 {
                            computers.iter_mut().for_each(|x| x.reset());
                            let digits =
                                match valid_setting(e + d * 10 + c * 100 + b * 1000 + a * 10000) {
                                    Some(d) => d,
                                    None => continue,
                                };

                            computers[0].input_to_stdin(digits[0]);
                            computers[0].input_to_stdin(0);
                            computers[0].run_until_completion();

                            for i in 1..5 {
                                computers[i].input_to_stdin(digits[i]);
                                computers[i].input_to_stdin(
                                    *computers[i - 1].view_stdout().last().expect("no stdout"),
                                );
                                computers[i].run_until_completion();
                            }

                            let res = computers[4].view_stdout().last().expect("no final stdout");
                            highest = max(*res, highest);
                        }
                    }
                }
            }
        }

        highest.into()
    }

    fn part2(&mut self) -> Solution {
        let mut computers: [IntCodeComputer; 5] = [
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
            IntCodeComputer::new(&self.program, None),
        ];

        let mut highest: isize = 0;

        // Is it brute-force time?
        for a in 5..=9 {
            for b in 5..=9 {
                for c in 5..=9 {
                    for d in 5..=9 {
                        for e in 5..=9 {
                            computers.iter_mut().for_each(|x| x.reset());
                            let digits =
                                match valid_setting(e + d * 10 + c * 100 + b * 1000 + a * 10000) {
                                    Some(d) => d,
                                    None => continue,
                                };

                            // Load initial digits to all computers
                            for (i, c) in computers.iter_mut().enumerate() {
                                c.input_to_stdin(digits[i]);
                            }

                            // Initially, run the very first computer
                            computers[0].input_to_stdin(0);
                            computers[0].run_until_output();

                            let mut output = *computers[0]
                                .view_stdout()
                                .last()
                                .expect("no stdout on init");
                            let mut curr_idx = 1;

                            loop {
                                if computers[4].has_halted() {
                                    break;
                                }

                                computers[curr_idx].input_to_stdin(output);
                                computers[curr_idx].run_until_output();

                                output = match computers[curr_idx].view_stdout().last() {
                                    Some(o) => *o,
                                    // This shouldn't hit
                                    None => break,
                                };

                                curr_idx += 1;
                                curr_idx %= computers.len();
                            }

                            let res = computers[4].view_stdout().last().expect("no final stdout");
                            highest = max(*res, highest);
                        }
                    }
                }
            }
        }

        highest.into()
    }
}

/// Checks if the given number is a valid phase setting (the number has no duplicates).
///
/// # Parameters
/// - `num`: The number to check.
///
/// # Returns
/// A vector of digits if the number is valid, or `None` if it is not.
fn valid_setting(num: isize) -> Option<Vec<isize>> {
    let mut digits = get_digits(num);
    digits.reverse();
    while digits.len() < 5 {
        digits.push(0);
    }
    digits.reverse();

    let mut mask = 0;
    // Make sure there are unique digits
    for digit in &digits {
        if mask & (1 << *digit) != 0 {
            return None;
        }

        mask |= 1 << *digit;
    }

    Some(digits)
}
