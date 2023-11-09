use common::day::{AoCProblem, Solution};
use std::collections::HashSet;

type Board = [[BoardElement; 5]; 5];

pub struct Day04 {
    bingo_boards: Vec<Board>,
    numbers_to_draw: Vec<i32>,
}

// https://adventofcode.com/2021/day/4
impl AoCProblem for Day04 {
    fn prepare(input: String) -> Self {
        let numbers_to_draw = input
            .lines()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut bingo_boards: Vec<Board> = vec![];

        // Skip first two lines so we start at the board directly.
        // Remove the new lines so we can get proper chunks.
        input
            .lines()
            .skip(2)
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .chunks(5)
            .for_each(|board_chunk| {
                let mut board: Board = [[BoardElement {
                    value: -1,
                    selected: false,
                }; 5]; 5];
                for (row_idx, chunk) in board_chunk.iter().enumerate() {
                    let col_vals = chunk
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    for col in 0..5 {
                        board[row_idx][col].value = col_vals[col];
                    }
                }

                bingo_boards.push(board);
            });

        Self {
            numbers_to_draw,
            bingo_boards,
        }
    }

    fn part1(&mut self) -> Solution {
        let mut boards = self.bingo_boards.clone();
        for num in &self.numbers_to_draw {
            for board in &mut boards {
                apply_num_to_board(board, *num);

                if !check_win(board) {
                    continue;
                }

                // This board won!
                return (get_sum_of_unselected(board) * num).into();
            }
        }

        (-1).into()
    }

    fn part2(&mut self) -> Solution {
        let mut boards = self.bingo_boards.clone();
        let mut checked: HashSet<i32> = HashSet::new();

        let mut sums: Vec<i32> = vec![];
        for num in &self.numbers_to_draw {
            for board in &mut boards {
                apply_num_to_board(board, *num);

                if !check_win(board) {
                    continue;
                }

                // Assume that every first row has a unique sum; this will be our identifier.
                let first_row_sum = board.first().unwrap().iter().map(|x| x.value).sum::<i32>();

                if checked.contains(&first_row_sum) {
                    continue;
                }

                checked.insert(first_row_sum);
                sums.push(get_sum_of_unselected(board) * num);
            }
        }

        if sums.is_empty() {
            -1
        } else {
            *sums.last().unwrap()
        }
        .into()
    }
}

#[derive(Debug, Copy, Clone)]
struct BoardElement {
    value: i32,
    selected: bool,
}

/// Selects one or more element(s) that is equal to the target.
///
/// # Parameters
/// - `board`: The board.
/// - `target`: The target value.
fn apply_num_to_board(board: &mut Board, target: i32) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j].value == target {
                board[i][j].selected = true;
            }
        }
    }
}

/// Gets the sum of all unselected values in the board.
///
/// # Parameters
/// - `board`: The board.
///
/// # Returns
/// The sum of all unselected elements.
fn get_sum_of_unselected(board: &Board) -> i32 {
    board
        .iter()
        .flatten()
        .filter(|x| !x.selected)
        .map(|x| x.value)
        .sum::<i32>()
}

/// Checks if the board is in a state such that a row or column has all been selected.
///
/// # Parameters
/// - `board`: The board.
///
/// # Returns
/// Whether the board is considered to be "won" (i.e. there exists a row or column such that all
/// elements are selected).
fn check_win(board: &Board) -> bool {
    // Check rows
    let row_check = board.iter().any(|x| x.iter().all(|y| y.selected));
    if row_check {
        return true;
    }

    // Check columns
    for c in 0..5 {
        let mut is_valid = true;
        for r in 0..5 {
            if !board[r][c].selected {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            return true;
        }
    }

    false
}
