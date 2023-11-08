use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day10 {
    program: Vec<Instruction>,
}

impl AoCProblem for Day10 {
    fn prepare(input: String) -> Self {
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
        // Gets the index to be updated in the CRT. Note that the cycle
        // position can be thought of as a one-based index, but when
        // updating the CRT we need a zero-based index, hence the -1
        // offset.
        let get_idx = |cycles: usize| ((cycles - 1) / 40, (cycles - 1) % 40);

        // Checks whether the given index is in the sprite range.
        let is_in_sprite_range = |idx: usize, x: isize| {
            idx == (x - 1) as usize || idx == x as usize || idx == (x + 1) as usize
        };

        // Draw a single pixel into the CRT during each cycle.
        let mut crt: [[char; 40]; 6] = [[' '; 40]; 6];
        let mut x = 1;
        let mut amt_cycles: usize = 0;
        for instruction in &self.program {
            amt_cycles += 1;
            let (row, col) = get_idx(amt_cycles);
            if row == 6 {
                break;
            }

            crt[row][col] = if is_in_sprite_range(col, x) { '#' } else { ' ' };
            if let Instruction::Addx(num) = instruction {
                amt_cycles += 1;
                let (row, col) = get_idx(amt_cycles);
                crt[row][col] = if is_in_sprite_range(col, x) { '#' } else { ' ' };
                x += num;
            }
        }

        format!(
            "\n{}",
            crt.into_iter()
                .map(String::from_iter)
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
