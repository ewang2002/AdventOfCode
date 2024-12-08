use common::problem::day::{AoCProblem, Solution};

pub struct Day07 {
    calibration_equations: Vec<(usize, Vec<usize>)>,
}

impl AoCProblem for Day07 {
    fn prepare(input: String) -> Self {
        Self {
            calibration_equations: input
                .lines()
                .map(|l| l.split_once(": ").unwrap())
                .map(|(raw_test_val, raw_nums)| {
                    (
                        raw_test_val.parse().unwrap(),
                        raw_nums.split(' ').map(|n| n.parse().unwrap()).collect(),
                    )
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        self.calibration_equations
            .iter()
            .filter(|(t, n)| can_calibrate(*t, n, 0, [Op::Plus, Op::Multiply]))
            .map(|(t, _)| *t)
            .sum::<usize>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        self.calibration_equations
            .iter()
            .filter(|(t, n)| can_calibrate(*t, n, 0, [Op::Plus, Op::Multiply, Op::Concat]))
            .map(|(t, _)| *t)
            .sum::<usize>()
            .into()
    }

    fn day() -> u32 {
        7
    }

    fn year() -> u32 {
        2024
    }
}

#[derive(Copy, Clone)]
enum Op {
    Plus,
    Multiply,
    Concat,
}

impl Op {
    /// Performs the operation on the two numbers.
    ///
    /// # Parameters
    /// - `num1`: The first number.
    /// - `num2`: The second number.
    ///
    /// # Returns
    /// The result of the operation on the two numbers.
    fn operate(&self, num1: usize, num2: usize) -> usize {
        match self {
            Op::Plus => num1 + num2,
            Op::Multiply => num1 * num2,
            // This is a fancier & faster  way of doing
            // format!("{num1}{num2}").parse::<usize>().unwrap()
            Op::Concat => num1 * 10_usize.pow(num2.ilog10() + 1) + num2,
        }
    }
}

/// Checks whether all the numbers provided in the input `nums`, using any combination
/// of operators, can be calibrated to get `test_val`.
///
/// # Parameters
/// - `test_val`: The value to test for.
/// - `nums`: A list of numbers that, along with `curr_res`, must be used to see if we can
///    achieve `test_val`.
/// - `curr_res`: The current result, as we're calculating towards the `test_val`.
/// - `ops`: The operators to use to calibrate the input numbers.
///
/// # Returns
/// Whether the input number array can be calibrated to achieve `test_val`.
fn can_calibrate<const N: usize>(
    test_val: usize,
    nums: &[usize],
    curr_res: usize,
    ops: [Op; N],
) -> bool {
    if nums.is_empty() {
        test_val == curr_res
    } else if curr_res > test_val {
        false
    } else {
        ops.iter()
            .any(|o| can_calibrate(test_val, &nums[1..], o.operate(curr_res, nums[0]), ops))
    }
}
