use common::problem::day::{AoCProblem, Solution};

pub struct Day09 {
    report: Vec<Vec<isize>>,
}

impl AoCProblem for Day09 {
    fn prepare(input: String) -> Self {
        Self {
            report: input
                .lines()
                .map(|line| line.split(' ').map(|item| item.parse().unwrap()).collect())
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut sum_values = 0;
        for history in &self.report {
            // Maintains all numbers that are directly on the right side of the "triangle"
            let mut right_side: Vec<isize> = vec![];
            // Pre-load the current history with all entries from this history entry
            let mut curr_history: Vec<isize> = history.to_vec();

            loop {
                right_side.push(curr_history[curr_history.len() - 1]);
                let mut diffs: Vec<isize> = vec![];

                // Compute the differences of each pair of elements (this will also be
                // the values of the next computed row down)
                for chunk in curr_history.windows(2) {
                    diffs.push(chunk[1] - chunk[0]);
                }

                if diffs.iter().all(|d| *d == 0) {
                    break;
                }

                // Essentially update the curr_history vector with the values for the
                // next computed row down
                curr_history.clear();
                curr_history.append(&mut diffs);
            }

            right_side.reverse();

            let mut value = 0;
            for elem in right_side {
                value += elem;
            }

            sum_values += value;
        }

        sum_values.into()
    }

    fn part2(&mut self) -> Solution {
        // This code is pretty much the same exact thing as above, just slightly different to account
        // for the fact that we want the value on the left side.
        let mut sum_values = 0;
        for history in &self.report {
            let mut left_side: Vec<isize> = vec![];
            let mut curr_history: Vec<isize> = history.to_vec();

            loop {
                left_side.push(curr_history[0]);
                let mut diffs: Vec<isize> = vec![];
                for chunk in curr_history.windows(2) {
                    diffs.push(chunk[1] - chunk[0]);
                }

                if diffs.iter().all(|d| *d == 0) {
                    break;
                }

                curr_history.clear();
                curr_history.append(&mut diffs);
            }

            left_side.reverse();

            let mut value = 0;
            for elem in left_side {
                value = elem - value;
            }

            sum_values += value;
        }

        sum_values.into()
    }

    fn day() -> u32 {
        9
    }

    fn year() -> u32 {
        2023
    }
}
