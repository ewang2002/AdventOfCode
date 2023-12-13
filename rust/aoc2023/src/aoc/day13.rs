use common::{
    constants::TWO_NEWLINE,
    problem::day::{AoCProblem, Solution},
};

type Pattern = Vec<Vec<char>>;

pub struct Day13 {
    patterns: Vec<Pattern>,
}

impl AoCProblem for Day13 {
    fn prepare(input: String) -> Self {
        Self {
            patterns: input
                .split(TWO_NEWLINE)
                .map(|raw_pattern| {
                    raw_pattern
                        .lines()
                        .map(|line| line.chars().collect())
                        .collect()
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut sum = 0;
        for pattern in &self.patterns {
            let v = compute_num_vertical_reflections(pattern, None);
            let h = compute_num_horizontal_reflections(pattern, None);
            sum += h * 100 + v;
        }

        sum.into()
    }

    fn part2(&mut self) -> Solution {
        let mut sum = 0;
        for pattern in &mut self.patterns {
            let orig_v = compute_num_vertical_reflections(pattern, None);
            let orig_h = compute_num_horizontal_reflections(pattern, None);

            let mut selected_v = usize::MIN;
            let mut selected_h = usize::MIN;

            'all: for i in 0..pattern.len() {
                for j in 0..pattern[i].len() {
                    let orig_pattern = pattern[i][j];
                    pattern[i][j] = match pattern[i][j] {
                        '#' => '.',
                        '.' => '#',
                        _ => unreachable!(),
                    };
                    let v = compute_num_vertical_reflections(
                        pattern,
                        if orig_v != 0 { Some(orig_v - 1) } else { None },
                    );
                    let h = compute_num_horizontal_reflections(
                        pattern,
                        if orig_h != 0 { Some(orig_h - 1) } else { None },
                    );

                    if v != orig_v && v != usize::MIN {
                        selected_v = v;
                    }

                    if h != orig_h && h != usize::MIN {
                        selected_h = h;
                    }

                    pattern[i][j] = orig_pattern;

                    if selected_v != orig_v && selected_h != orig_h {
                        break 'all;
                    }
                }
            }

            sum += selected_h * 100 + selected_v;
        }

        sum.into()
    }

    fn day() -> u32 {
        13
    }

    fn year() -> u32 {
        2023
    }
}

/// Computes the number of rows that are above a vertical reflection. If no such reflection is
/// found, then `usize::MIN` is returned.
///
/// # Parameters
/// - `pattern`: The pattern to check for vertical reflections.
/// - `ignore_v`: The column to ignore when checking for vertical reflections. This should be zero-
///               indexed.
///
/// # Returns
/// The number of rows that are above a vertical reflection. If no such reflection is found, then
/// `usize::MIN` is returned.
///
/// Alternatively, the value returned is the index (one-indexed) of the column where the line between
/// said column and the following column is a vertical reflection.
fn compute_num_vertical_reflections(pattern: &Pattern, ignore_v: Option<usize>) -> usize {
    for col_idx in 0..pattern[0].len() - 1 {
        if let Some(ignore_v) = ignore_v {
            if col_idx == ignore_v {
                continue;
            }
        }

        let mut is_valid_reflection = true;
        let mut left_col_idx = col_idx;
        let mut right_col_idx = col_idx + 1;

        loop {
            if pattern
                .iter()
                .map(|row| row[left_col_idx])
                .zip(pattern.iter().map(|row| row[right_col_idx]))
                .any(|(l_c, r_c)| l_c != r_c)
            {
                is_valid_reflection = false;
                break;
            }

            if left_col_idx == 0 || right_col_idx == pattern[0].len() - 1 {
                break;
            }

            left_col_idx -= 1;
            right_col_idx += 1;
        }

        if is_valid_reflection {
            // col_idx is the index of the column that is directly before the reflection line.
            // Therefore, col_idx + 1 is the number of the columns that is directly before the
            // reflection line.
            return col_idx + 1;
        }
    }

    usize::MIN
}

/// Computes the number of rows that are above a horizontal reflection. If no such reflection is
/// found, then `usize::MIN` is returned.
///
/// # Parameters
/// - `pattern`: The pattern to check for horizontal reflections.
/// - `ignore_h`: The row to ignore when checking for horizontal reflections. This should be zero-
///               indexed.
///
/// # Returns
/// The number of rows that are above a horizontal reflection. If no such reflection is found, then
/// `usize::MIN` is returned.
///
/// Alternatively, the value returned is the index (one-indexed) of the row where the line between
/// said row and the following row is a horizontal reflection.
fn compute_num_horizontal_reflections(pattern: &Pattern, ignore_h: Option<usize>) -> usize {
    for row_idx in 0..pattern.len() - 1 {
        if let Some(ignore_h) = ignore_h {
            if row_idx == ignore_h {
                continue;
            }
        }

        let mut is_valid_reflection = true;
        let mut upper_row_idx = row_idx;
        let mut lower_row_idx = row_idx + 1;

        loop {
            let upper_row = &pattern[upper_row_idx];
            let lower_row = &pattern[lower_row_idx];

            if upper_row
                .iter()
                .zip(lower_row.iter())
                .any(|(u_c, l_c)| u_c != l_c)
            {
                is_valid_reflection = false;
                break;
            }

            if upper_row_idx == 0 || lower_row_idx == pattern.len() - 1 {
                break;
            }

            upper_row_idx -= 1;
            lower_row_idx += 1;
        }

        if is_valid_reflection {
            // row_idx is the index of the row that is directly before the reflection line.
            // Therefore, row_idx + 1 is the number of the rows that is directly before the
            // reflection line.
            return row_idx + 1;
        }
    }

    usize::MIN
}
