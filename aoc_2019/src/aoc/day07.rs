use crate::aoc::aoc_problem::AoCProblem;
use crate::intcode::{get_digits, parse_intcode, IntCodeComputer};
use std::cmp::max;

pub struct Day07 {
    program: Vec<isize>,
}

// https://adventofcode.com/2019/day/7
impl AoCProblem<isize, isize> for Day07 {
    fn prepare(input: Vec<String>) -> Self {
        Self {
            program: parse_intcode(&input[0]),
        }
    }

    fn part1(&mut self) -> isize {
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
                            let digits = match valid_setting(e + d * 10 + c * 100 + b * 1000 + a * 10000) {
                                Some(d) => d,
                                None => continue
                            };

                            computers[0].input_to_stdin(digits[0]);
                            computers[0].input_to_stdin(0);
                            computers[0].run();

                            for i in 1..5 {
                                computers[i].input_to_stdin(digits[i]);
                                computers[i]
                                    .input_to_stdin(*computers[i - 1].view_stdout().last().expect("no stdout"));
                                computers[i].run();
                            }

                            let res = computers[4].view_stdout().last().expect("no final stdout");
                            highest = max(*res, highest);
                        }
                    }
                }
            }
        }

        highest
    }

    fn part2(&mut self) -> isize {
        0
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