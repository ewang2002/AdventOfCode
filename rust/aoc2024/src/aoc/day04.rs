use common::problem::day::{AoCProblem, Solution};

pub struct Day04 {
    word_search: Vec<Vec<char>>,
}

impl AoCProblem for Day04 {
    fn prepare(input: String) -> Self {
        Self {
            word_search: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut ct = 0;
        for i in 0..self.word_search.len() {
            for (j, letter) in self.word_search[i].iter().enumerate() {
                if *letter != 'X' {
                    continue;
                }

                ct += count_xmas_words(&self.word_search, i as isize, j as isize);
            }
        }

        ct.into()
    }

    fn part2(&mut self) -> Solution {
        let mut ct = 0;
        for i in 0..self.word_search.len() {
            for (j, letter) in self.word_search[i].iter().enumerate() {
                if *letter != 'A' {
                    continue;
                }

                ct += if is_xmas_structure(&self.word_search, i as isize, j as isize) {
                    1
                } else {
                    0
                };
            }
        }

        ct.into()
    }

    fn day() -> u32 {
        4
    }

    fn year() -> u32 {
        2024
    }
}

const DI_DJ: [[[isize; 3]; 2]; 8] = [
    // Vertical up
    [[-1, -2, -3], [0, 0, 0]],
    // Vertical down
    [[1, 2, 3], [0, 0, 0]],
    // Horizontal left
    [[0, 0, 0], [-1, -2, -3]],
    // Horizontal right
    [[0, 0, 0], [1, 2, 3]],
    // Diagonal up left
    [[-1, -2, -3], [-1, -2, -3]],
    // Diagonal up right
    [[-1, -2, -3], [1, 2, 3]],
    // Diagonal down left
    [[1, 2, 3], [-1, -2, -3]],
    // Diagonal down right
    [[1, 2, 3], [1, 2, 3]],
];

const MAS: [char; 3] = ['M', 'A', 'S'];

/// Counts the number of times the word "XMAS" appears at `(i, j)`.
///
/// # Parameters
/// - `search`: The word search grid.
/// - `i`: The row index corresponding to the location of "X"
/// - `j`: The column index corresponding to the location of "X"
///
/// # Returns
/// The number of times the word "XMAS" appears at `(i, j)`. If `(i, j)`
/// is not "X", then 0 is returned.
fn count_xmas_words(search: &[Vec<char>], i: isize, j: isize) -> usize {
    if search[i as usize][j as usize] != 'X' {
        return 0;
    }

    let mut count = 0;
    for [di_diff, dj_diff] in DI_DJ {
        let mut is_ok = true;
        for (curr_letter_idx, (di, dj)) in di_diff.into_iter().zip(dj_diff).enumerate() {
            let new_i = i + di;
            let new_j = j + dj;
            if new_i < 0
                || new_i >= search.len() as isize
                || new_j < 0
                || new_j >= search[0].len() as isize
            {
                is_ok = false;
                break;
            }

            is_ok &= search[new_i as usize][new_j as usize] == MAS[curr_letter_idx];
        }

        if is_ok {
            count += 1;
        }
    }

    count
}

/// Determines whether the structure at `(i, j)` is a valid XMAS structure.
///
/// # Parameters
/// - `search`: The word search grid.
/// - `i`: The row index corresponding to the location of "A"
/// - `j`: The column index corresponding to the location of "A"
///
/// # Returns
/// `true` if the structure at `(i, j)` is a valid XMAS structure, `false` otherwise.
fn is_xmas_structure(search: &[Vec<char>], i: isize, j: isize) -> bool {
    if search[i as usize][j as usize] != 'A' {
        return false;
    }

    // Make sure we aren't out of bounds
    if i - 1 < 0 || i + 1 >= search.len() as isize || j - 1 < 0 || j + 1 >= search[0].len() as isize
    {
        return false;
    }

    // Get top left letter
    let top_left = search[(i - 1) as usize][(j - 1) as usize];
    // Get top right letter
    let top_right = search[(i - 1) as usize][(j + 1) as usize];
    // Get bottom left letter
    let bottom_left = search[(i + 1) as usize][(j - 1) as usize];
    // Get bottom right letter
    let bottom_right = search[(i + 1) as usize][(j + 1) as usize];

    matches!(
        (top_left, bottom_right, top_right, bottom_left),
        ('M', 'S', 'M', 'S') | ('M', 'S', 'S', 'M') | ('S', 'M', 'M', 'S') | ('S', 'M', 'S', 'M')
    )
}
