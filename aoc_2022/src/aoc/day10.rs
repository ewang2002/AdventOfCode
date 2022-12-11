use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day10 {
    program: Vec<Instruction>,
}

impl AoCProblem for Day10 {
    fn prepare(input: &str) -> Self {
        Self {
            program: input.lines().map(|l| l.into()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        const CYCLES_TO_CONSIDER: [isize; 6] = [20, 60, 100, 140, 180, 220];
        let mut idx_cycle = 0;
        let mut x = 1;
        let mut amt_cycles = 0;
        let mut strength_amt = 0;
        for instruction in &self.program {
            if idx_cycle >= CYCLES_TO_CONSIDER.len() {
                break;
            }

            amt_cycles += 1;
            if amt_cycles == CYCLES_TO_CONSIDER[idx_cycle] {
                strength_amt += amt_cycles * x;
                idx_cycle += 1;
            }

            if let Instruction::Addx(num) = instruction {
                amt_cycles += 1;
                if amt_cycles == CYCLES_TO_CONSIDER[idx_cycle] {
                    strength_amt += amt_cycles * x;
                    idx_cycle += 1;
                }

                x += num;
            }
        }

        strength_amt.into()
    }

    fn part2(&mut self) -> Solution {
        let get_idx = |cycles: isize| {
            (
                ((cycles - 1) / 40) as usize,
                ((cycles - 1) % 40) as usize,
                (cycles - 1) % 40,
            )
        };

        // Draw a single pixel into the CRT during each cycle.
        let mut crt: [[char; 40]; 6] = [[' '; 40]; 6];
        let mut x = 1;
        let mut amt_cycles = 0;
        for instruction in &self.program {
            amt_cycles += 1;
            let (row, col, icol) = get_idx(amt_cycles);
            if row == 6 {
                break;
            }

            crt[row][col] = if icol == x - 1 || icol == x || icol == x + 1 {
                '#'
            } else {
                ' '
            };

            if let Instruction::Addx(num) = instruction {
                amt_cycles += 1;
                let (row, col, icol) = get_idx(amt_cycles);
                crt[row][col] = if icol == x - 1 || icol == x || icol == x + 1 {
                    '#'
                } else {
                    ' '
                };

                x += num;
            }
        }

        format!(
            "\n{}",
            crt.into_iter()
                .map(|x| String::from_iter(x))
                .collect::<Vec<_>>()
                .join("\n")
        )
        .into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s.starts_with("addx") {
            Instruction::Addx(s.split_once(' ').unwrap().1.parse().unwrap())
        } else if s.starts_with("noop") {
            Instruction::Noop
        } else {
            unreachable!()
        }
    }
}
