use common::problem::day::{AoCProblem, Solution};

pub struct Day03 {
    memory: Vec<char>,
}

impl AoCProblem for Day03 {
    fn prepare(input: String) -> Self {
        Self {
            memory: input.chars().collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut i = 0;
        let mut res = 0;
        while i < self.memory.len() {
            if self.memory[i] != 'm' {
                i += 1;
                continue;
            }

            i += 1;
            if self.memory[i] != 'u' {
                i += 1;
                continue;
            }

            i += 1;
            if self.memory[i] != 'l' {
                i += 1;
                continue;
            }

            i += 1;
            if self.memory[i] != '(' {
                i += 1;
                continue;
            }

            // Move past (
            i += 1;
            let mut left_num: isize = 0;
            while i < self.memory.len() {
                if self.memory[i] == ',' {
                    break;
                }

                if !self.memory[i].is_ascii_digit() {
                    left_num = isize::MIN;
                    break;
                }

                left_num = left_num * 10 + self.memory[i].to_digit(10).unwrap() as isize;
                i += 1;
            }

            if left_num == isize::MIN || i >= self.memory.len() {
                continue;
            }

            // Move past comma
            i += 1;
            let mut right_num: isize = 0;
            while i < self.memory.len() {
                if self.memory[i] == ')' {
                    break;
                }

                if !self.memory[i].is_ascii_digit() {
                    right_num = isize::MIN;
                    break;
                }

                right_num = right_num * 10 + self.memory[i].to_digit(10).unwrap() as isize;
                i += 1;
            }

            if right_num == isize::MIN || i >= self.memory.len() {
                continue;
            }

            // Move past )
            i += 1;
            res += left_num * right_num;
        }

        res.into()
    }

    fn part2(&mut self) -> Solution {
        let mut can_multiply = true;
        let mut res = 0;
        let mut i = 0;
        loop {
            match get_next_token(&self.memory, &mut i) {
                Instr::Mul(l, r) if can_multiply => res += l * r,
                Instr::Do => can_multiply = true,
                Instr::Dont => can_multiply = false,
                Instr::None => break,
                _ => continue,
            };
        }

        res.into()
    }

    fn day() -> u32 {
        3
    }

    fn year() -> u32 {
        2024
    }
}

enum Instr {
    Do,
    Dont,
    Mul(isize, isize),
    None,
}

/// Gets the next token (i.e., instruction) to process.
/// 
/// # Parameters
/// - `memory`: The active memory to operate on.
/// - `i`: The index to start parsing at.
/// 
/// # Returns
/// The next possible token (Mul, Do, or Don't), or None if no more tokens can be found.
fn get_next_token(memory: &[char], i: &mut usize) -> Instr {
    let mut buffer = String::new();
    while *i < memory.len() {
        // We want to look for the next token
        match memory[*i] {
            'd' if buffer.is_empty() => {
                buffer.push('d');
            }
            'o' if buffer.as_str() == "d" => {
                buffer.push('o');
            }
            'n' if buffer.as_str() == "do" => {
                buffer.push('n');
            }
            '\'' if buffer.as_str() == "don" => {
                buffer.push('\'');
            }
            't' if buffer.as_str() == "don'" => {
                buffer.push('t');
            }
            '(' if buffer.as_str() == "do" => {
                *i += 1;
                if memory[*i] == ')' {
                    return Instr::Do;
                }

                buffer.clear();
            }
            '(' if buffer.as_str() == "don't" => {
                *i += 1;
                if memory[*i] == ')' {
                    return Instr::Dont;
                }

                buffer.clear();
            }
            'm' if buffer.is_empty() => {
                buffer.push('m');
            }
            'u' if buffer.as_str() == "m" => {
                buffer.push('u');
            }
            'l' if buffer.as_str() == "mu" => {
                buffer.clear();

                // Skip the l
                *i += 1;

                // Do we have ( ?
                if memory[*i] != '(' {
                    *i += 1;
                    continue;
                }

                *i += 1;
                // Parse number until we hit comma
                let mut left_num: isize = 0;
                while *i < memory.len() && memory[*i] != ',' {
                    if !memory[*i].is_ascii_digit() {
                        left_num = isize::MIN;
                        break;
                    }

                    left_num = left_num * 10 + memory[*i].to_digit(10).unwrap() as isize;
                    *i += 1;
                }

                if left_num == isize::MIN || *i >= memory.len() {
                    continue;
                }

                // Move past ,
                *i += 1;
                let mut right_num: isize = 0;
                while *i < memory.len() && memory[*i] != ')' {
                    if !memory[*i].is_ascii_digit() {
                        right_num = isize::MIN;
                        break;
                    }

                    right_num = right_num * 10 + memory[*i].to_digit(10).unwrap() as isize;
                    *i += 1;
                }

                if right_num == isize::MIN || *i >= memory.len() {
                    continue;
                }

                // Move past )
                *i += 1;
                return Instr::Mul(left_num, right_num);
            }
            _ => {
                buffer.clear();
            }
        }

        *i += 1;
    }

    Instr::None
}
