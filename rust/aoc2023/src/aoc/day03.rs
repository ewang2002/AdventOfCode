use common::problem::day::{AoCProblem, Solution};

pub struct Day03 {
    parts: Vec<Vec<char>>,
}

impl AoCProblem for Day03 {
    fn prepare(input: String) -> Self {
        Self {
            parts: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut sum = 0;
        for (row_idx, line) in self.parts.iter().enumerate() {
            let mut i = 0;
            while i < line.len() {
                let this_char = line[i];
                if this_char == '.' {
                    i += 1;
                    continue;
                }

                if this_char.is_ascii_digit() {
                    let mut this_num = 0;
                    // Get the row index representing the row directly above the number we found (if possible)
                    let top_idx = if row_idx == 0 { row_idx } else { row_idx - 1 };

                    // Get the row index representing the row directly below the number we found (if possible)
                    let bottom_idx = if row_idx + 1 < self.parts.len() {
                        row_idx + 1
                    } else {
                        row_idx
                    };

                    // Get the column index representing the column that is directly to the left of the number (if possible)
                    let left_idx = if i == 0 { i } else { i - 1 };

                    // Extract the digits that are next to the digit we just found
                    while i < line.len() && line[i].is_ascii_digit() {
                        this_num = this_num * 10 + line[i].to_digit(10).unwrap();
                        i += 1;
                    }

                    // Get the column index representing the column that is directly to the right of the number (if possible)
                    let right_idx = if i < line.len() { i } else { i - 1 };

                    // Check left bound (top to bottom)
                    //         X......
                    //         X12345.
                    //         X......
                    let mut is_next_to_symbol = false;
                    for y in top_idx..=bottom_idx {
                        if y >= self.parts.len() {
                            break;
                        }

                        if !self.parts[y][left_idx].is_ascii_digit()
                            && self.parts[y][left_idx] != '.'
                        {
                            is_next_to_symbol = true;
                            break;
                        }
                    }

                    // Check right bound (top to bottom)
                    //         ......X
                    //         .12345X
                    //         ......X
                    //
                    // Note: if we already found that the number is next to a symbol, then we don't
                    // need to check this.
                    if !is_next_to_symbol {
                        for y in top_idx..=bottom_idx {
                            if y >= self.parts.len() {
                                break;
                            }

                            if !self.parts[y][right_idx].is_ascii_digit()
                                && self.parts[y][right_idx] != '.'
                            {
                                is_next_to_symbol = true;
                                break;
                            }
                        }
                    }

                    // Check top bound (left to right)
                    //         XXXXXXX
                    //         .12345.
                    //         .......
                    if !is_next_to_symbol {
                        for x in left_idx..=right_idx {
                            if x >= self.parts[top_idx].len() {
                                break;
                            }

                            if !self.parts[top_idx][x].is_ascii_digit()
                                && self.parts[top_idx][x] != '.'
                            {
                                is_next_to_symbol = true;
                                break;
                            }
                        }
                    }

                    // Check bottom bound (left to right)
                    //         .......
                    //         .12345.
                    //         XXXXXXX
                    if !is_next_to_symbol {
                        for x in left_idx..=right_idx {
                            if x >= self.parts[top_idx].len() {
                                break;
                            }

                            if !self.parts[bottom_idx][x].is_ascii_digit()
                                && self.parts[bottom_idx][x] != '.'
                            {
                                is_next_to_symbol = true;
                                break;
                            }
                        }
                    }

                    // If the number is next to a symbol, then we can add it up
                    if is_next_to_symbol {
                        sum += this_num;
                    }
                }

                i += 1;
            }
        }

        sum.into()
    }

    fn part2(&mut self) -> Solution {
        // First, find all indices of the star (*)
        let mut valid_indices = vec![];
        for (row_idx, row) in self.parts.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col == '*' {
                    valid_indices.push((row_idx, col_idx));
                }
            }
        }

        fn vec_to_digits(digits: &[char]) -> usize {
            let mut final_num = 0;
            for digit in digits {
                final_num = final_num * 10 + digit.to_digit(10).unwrap() as usize;
            }
            final_num
        }

        // Processes a single row, looking for all numbers directly at the specified column.
        // For example, if there is a * at coordinate (x, y), then `row_idx_consider` would be
        // x + dx where dx \in {1, -1}, i.e., the row that is directly above or below x.
        //
        // Likewise, `curr_col_idx` would be y, the current column to consider.
        //
        // For example purposes, I'll assume dx = +1 in the comments inside this function.
        let process_row = |row_idx_consider: usize, curr_col_idx: usize| -> Vec<usize> {
            // Is there a digit to the left of the current column for the row to consider?
            //
            // e.g., if the * is at coordinates (x, y), then is there a digit at (x + dx, y - 1)?
            //
            //       ...*...
            //       123
            let left_digit = curr_col_idx != 0
                && self.parts[row_idx_consider][curr_col_idx - 1].is_ascii_digit();

            // Is there a digit to the right of the current column for the row to consider?
            //
            // e.g., if the * is at coordinates (x, y), then is there a digit at (x + dx, y + 1)?
            //
            //       ...*...
            //           123
            let right_digit = curr_col_idx + 1 < self.parts[row_idx_consider].len()
                && self.parts[row_idx_consider][curr_col_idx + 1].is_ascii_digit();

            // Is there a digit directly above/below the current column (where the * is)?
            //
            // e.g., if the * is at coordinates (x, y), then is there a digit at (x + dx, y)?
            //
            //       ...*...
            //          5
            let direct_digit = self.parts[row_idx_consider][curr_col_idx].is_ascii_digit();

            let mut all_numbers = vec![];

            // If we have no digits whatsoever, then we're done.
            if !left_digit && !right_digit && !direct_digit {
                return all_numbers;
            }

            let mut num_to_build = vec![];

            // If we have a left digit to consider, then get all digits associated with that digit.
            if left_digit {
                let mut i = curr_col_idx - 1;
                while self.parts[row_idx_consider][i].is_ascii_digit() {
                    num_to_build.push(self.parts[row_idx_consider][i]);
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }

                num_to_build.reverse();
            }

            // If we have a digit directly above/below the *, then combine that digit
            // with the digits from the left digit
            if direct_digit {
                num_to_build.push(self.parts[row_idx_consider][curr_col_idx]);
            } else {
                // If there isn't a digit above/below the *, then we can save the number.
                if !num_to_build.is_empty() {
                    all_numbers.push(vec_to_digits(&num_to_build));
                }

                num_to_build.clear();
            }

            // If we have a right digit to consider, then combine that digit (and any digits)
            // directly after the right digit w/ the digits from the left and direct digits
            // as needed.
            if right_digit {
                let mut i = curr_col_idx + 1;
                while i < self.parts[row_idx_consider].len()
                    && self.parts[row_idx_consider][i].is_ascii_digit()
                {
                    num_to_build.push(self.parts[row_idx_consider][i]);
                    i += 1;
                }
            }

            // If there was a right digit (and possibly direct and left digits), then build them
            if !num_to_build.is_empty() {
                all_numbers.push(vec_to_digits(&num_to_build));
            }

            all_numbers
        };

        let mut sum = 0;
        // Now that we have all the indices, we can look around each one to see if we can find a number.
        for (row_idx, col_idx) in valid_indices {
            let mut all_numbers = vec![];

            // Easy case: is there a digit directly to the left of the *?
            if col_idx != 0 && self.parts[row_idx][col_idx - 1].is_ascii_digit() {
                let mut digits = vec![];
                let mut i = col_idx - 1;
                while self.parts[row_idx][i].is_ascii_digit() {
                    digits.push(self.parts[row_idx][i]);
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }

                digits.reverse();
                all_numbers.push(vec_to_digits(&digits));
            }

            // Easy case: is there a digit directly to the right of the *?
            if col_idx + 1 < self.parts[row_idx].len()
                && self.parts[row_idx][col_idx + 1].is_ascii_digit()
            {
                let mut digits = vec![];
                let mut i = col_idx + 1;
                while i < self.parts[row_idx].len() && self.parts[row_idx][i].is_ascii_digit() {
                    digits.push(self.parts[row_idx][i]);
                    i += 1;
                }

                all_numbers.push(vec_to_digits(&digits));
            }

            // Hard case: is there a digit anywhere above the * (i.e., at the current row - 1)?
            if row_idx != 0 {
                for n in process_row(row_idx - 1, col_idx) {
                    all_numbers.push(n);
                }
            }

            // Hard case: is there a digit anywhere below the * (i.e., at the current row + 1)?
            if row_idx + 1 < self.parts.len() {
                for n in process_row(row_idx + 1, col_idx) {
                    all_numbers.push(n);
                }
            }

            // If we find exactly two numbers, then this is a valid gear
            if all_numbers.len() == 2 {
                sum += all_numbers.into_iter().product::<usize>();
            }
        }

        sum.into()
    }

    fn day() -> u32 {
        3
    }

    fn year() -> u32 {
        2023
    }
}
