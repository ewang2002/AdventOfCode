use common::day::{AoCProblem, Solution};

pub struct Day06 {
    fish_timers: Vec<usize>,
}

// https://adventofcode.com/2021/day/6
impl AoCProblem for Day06 {
    fn prepare(input: String) -> Self {
        return Day06 {
            fish_timers: input
                .lines()
                .nth(0)
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        };
    }

    fn part1(&mut self) -> Solution {
        calculate_fish_amt(&self.fish_timers, 80).into()
    }

    fn part2(&mut self) -> Solution {
        calculate_fish_amt(&self.fish_timers, 256).into()
    }
}

/// Calculates the number of fish after a certain number of day.
///
/// # Parameters
/// - `initial_count`: A slice containing the initial number of lanternfish, where each element
/// represents the amount of time left before reproducing.
/// - `target_day`: The target day to calculate.
///
/// # Returns
/// The number of fish after `target_day` days passes.
fn calculate_fish_amt(initial_count: &[usize], target_day: i32) -> usize {
    // Index 0 = fish with timer 0
    // Index 1 = fish with timer 1
    // ...
    // Index 8 = fish with timer 8
    let mut arr: [usize; 9] = [0; 9];

    for init_fish in initial_count {
        arr[*init_fish] += 1;
    }

    let mut num_days_passed = 0;
    while num_days_passed < target_day {
        let num_fish_to_add = arr[0];
        for i in 0_usize..8_usize {
            arr[i] = arr[i + 1];
        }

        arr[6] += num_fish_to_add;
        arr[8] = num_fish_to_add;
        num_days_passed += 1;
    }

    return arr.iter().sum::<usize>();
}
