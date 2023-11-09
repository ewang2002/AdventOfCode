use common::day::{AoCProblem, Solution};

pub struct Day07 {
    horiz_pos: Vec<i32>,
}

// https://adventofcode.com/2021/day/7
impl AoCProblem for Day07 {
    fn prepare(input: String) -> Self {
        return Day07 {
            horiz_pos: input
                .lines()
                .nth(0)
                .unwrap()
                .split(',')
                .map(|x| x.parse::<_>().unwrap())
                .collect::<_>(),
        };
    }

    fn part1(&mut self) -> Solution {
        get_amt_fuel(&self.horiz_pos, |n| n.abs()).into()
    }

    fn part2(&mut self) -> Solution {
        get_amt_fuel(&self.horiz_pos, |n| {
            let float_num = n.abs() as f32;
            // 1 + 2 + ... + n = 0.5 * n * (n + 1)
            (0.5 * float_num * (float_num + 1.0)) as i32
        })
        .into()
    }
}

/// Gets the amount of fuel needed to align every crab to the same horizontal position.
///
/// # Parameters
/// - `horiz_pos`: The slice containing the initial horizontal positions.
/// - `fuel_function`: The function used to calculate the amount of fuel used.
///
/// # Returns
/// The amount of fuel used.
fn get_amt_fuel<F>(horiz_pos: &[i32], fuel_function: F) -> usize
where
    F: Fn(i32) -> i32,
{
    let mut min_fuel_used = usize::MAX;

    let min = *horiz_pos.iter().min().unwrap();
    let max = *horiz_pos.iter().max().unwrap();

    for target in min..=max {
        let mut fuel_used = 0_usize;
        let mut broken_out = false;
        for elem in horiz_pos {
            fuel_used += fuel_function(elem - target).unsigned_abs() as usize;
            if fuel_used > min_fuel_used {
                broken_out = true;
                break;
            }
        }

        if broken_out {
            continue;
        }

        min_fuel_used = fuel_used;
    }

    min_fuel_used
}
