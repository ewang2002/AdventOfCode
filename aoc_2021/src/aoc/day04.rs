use std::collections::HashSet;
use crate::aoc::aoc_problem::AoCProblem;

type Board = [[BoardElement; 5]; 5];

pub struct Day04 {
    bingo_boards: Vec<Board>,
    numbers_to_draw: Vec<i32>,
}

impl AoCProblem<i32, i32> for Day04 {
    fn prepare(input: Vec<String>) -> Self {
        let numbers_to_draw = input[0].split(",")
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut bingo_boards: Vec<Board> = vec![];

        // Skip first two lines so we start at the board directly.
        // Remove the new lines so we can get proper chunks.
        input.iter()
            .skip(2)
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .chunks(5)
            .for_each(|board_chunk| {
                let mut board: Board = [[BoardElement { value: -1, selected: false }; 5]; 5];
                let mut row_idx: usize = 0;
                for chunk in board_chunk {
                    let col_vals = chunk.split(" ")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    for col in 0..5 {
                        board[row_idx][col].value = col_vals[col];
                    }

                    row_idx += 1;
                }

                bingo_boards.push(board);
            });

        return Day04 { numbers_to_draw, bingo_boards };
    }

    // --- Day 4: Giant Squid ---
    // You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep
    // that you can't see any sunlight. What you can see, however, is a giant squid that has
    // attached itself to the outside of your submarine.
    //
    // Maybe it wants to play bingo?
    //
    // Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are
    // chosen at random, and the chosen number is marked on all boards on which it appears.
    // (Numbers may not appear on all boards.) If all numbers in any row or any column of a board
    // are marked, that board wins. (Diagonals don't count.)
    //
    // The submarine has a bingo subsystem to help passengers (currently, you and the giant squid)
    // pass the time. It automatically generates a random order in which to draw numbers and a
    // random set of boards (your puzzle input). For example:
    //
    //  7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
    //
    //  22 13 17 11  0
    //   8  2 23  4 24
    //  21  9 14 16  7
    //   6 10  3 18  5
    //   1 12 20 15 19
    //
    //   3 15  0  2 22
    //   9 18 13 17  5
    //  19  8  7 25 23
    //  20 11 10 24  4
    //  14 21 16 12  6
    //
    //  14 21 17 24  4
    //  10 16 15  9 19
    //  18  8 23 26 20
    //  22 11 13  6  5
    //   2  0 12  3  7
    //
    // After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the
    // boards are marked as follows (shown here adjacent to each other to save space):
    //
    //  22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
    //   8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
    //  21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
    //   6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
    //   1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
    //
    // After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:
    //
    //  22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
    //   8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
    //  21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
    //   6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
    //   1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
    //
    // Finally, 24 is drawn:
    //
    //  22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
    //   8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
    //  21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
    //   6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
    //   1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
    //
    // At this point, the third board wins because it has at least one complete row or column of
    // marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).
    //
    // The score of the winning board can now be calculated. Start by finding the sum of all
    // unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the
    // number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.
    //
    // To guarantee victory against the giant squid, figure out which board will win first. What
    // will your final score be if you choose that board?
    fn part1(&self) -> i32 {
        let mut boards = self.bingo_boards.clone();
        for num in &self.numbers_to_draw {
            for board in &mut boards {
                apply_num_to_board(board, *num);

                if !check_win(board) {
                    continue;
                }

                // This board won!
                return get_sum_of_unselected(board) * num;
            }
        }

        return -1;
    }

    // --- Part Two ---
    // On the other hand, it might be wise to try a different strategy: let the giant squid win.
    //
    // You aren't sure how many bingo boards a giant squid could play at once, so rather than waste
    // time counting its arms, the safe thing to do is to figure out which board will win last and
    // choose that one. That way, no matter which boards it picks, it will win for sure.
    //
    // In the above example, the second board is the last to win, which happens after 13 is
    // eventually called and its middle column is completely marked. If you were to keep playing
    // until this point, the second board would have a sum of unmarked numbers equal to 148 for a
    // final score of 148 * 13 = 1924.
    //
    // Figure out which board will win last. Once it wins, what would its final score be?
    fn part2(&self) -> i32 {
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
                let first_row_sum = board.first().unwrap().iter()
                    .map(|x| x.value).sum::<i32>();

                if checked.contains(&first_row_sum) {
                    continue;
                }

                checked.insert(first_row_sum);
                sums.push(get_sum_of_unselected(board) * num);
            }
        }

        return if sums.is_empty() { -1 } else { *sums.last().unwrap() };
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
fn apply_num_to_board(board: &mut Board, target: i32) -> () {
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
    board.iter()
        .flat_map(|x| x)
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
    let row_check = board
        .iter()
        .any(|x| x.iter().all(|y| y.selected));
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

    return false;
}