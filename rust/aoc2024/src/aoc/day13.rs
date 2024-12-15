use common::constants::TWO_NEWLINE;
use common::problem::day::{AoCProblem, Solution};

pub struct Day13 {
    machines: Vec<Machine>,
}

impl AoCProblem for Day13 {
    fn prepare(input: String) -> Self {
        let mut machines = vec![];
        for entry in input.split(TWO_NEWLINE) {
            let mut lines = entry.lines();
            let btn_a_info = lines.next().unwrap();
            let btn_b_info = lines.next().unwrap();
            let prize_info = lines.next().unwrap();
            assert_eq!(0, lines.count());

            let (a_x, a_y) = btn_a_info[10..].split_once(", ").unwrap();
            let btn_a = Button::new(a_x[2..].parse().unwrap(), a_y[2..].parse().unwrap());

            let (b_x, b_y) = btn_b_info[10..].split_once(", ").unwrap();
            let btn_b = Button::new(b_x[2..].parse().unwrap(), b_y[2..].parse().unwrap());

            let (prize_x, prize_y) = prize_info[7..].split_once(", ").unwrap();
            machines.push(Machine::new(
                btn_a,
                btn_b,
                (prize_x[2..].parse().unwrap(), prize_y[2..].parse().unwrap()),
            ));
        }

        Self { machines }
    }

    fn part1(&mut self) -> Solution {
        self.machines
            .iter()
            .map(|m| {
                solve(
                    &[m.button_a.x_change, m.button_b.x_change, m.prize_location.0],
                    &[m.button_a.y_change, m.button_b.y_change, m.prize_location.1],
                )
                .unwrap_or((0, 0))
            })
            .map(|(x, y)| 3 * x + y)
            .sum::<isize>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        self.machines
            .iter()
            .map(|m| {
                solve(
                    &[
                        m.button_a.x_change,
                        m.button_b.x_change,
                        m.prize_location.0 + 10000000000000,
                    ],
                    &[
                        m.button_a.y_change,
                        m.button_b.y_change,
                        m.prize_location.1 + 10000000000000,
                    ],
                )
                .unwrap_or((0, 0))
            })
            .map(|(x, y)| 3 * x + y)
            .sum::<isize>()
            .into()
    }

    fn day() -> u32 {
        13
    }

    fn year() -> u32 {
        2024
    }
}

/// Solves a system of two linear equations in the form `ax + by = c`.
///
/// # Parameters
/// - `first_equ`: A slice representing the first equation. The slice must be of the form
///                `[a, b, c]`.
/// - `second_equ`: A slice representing the second equation. The slice must be of the form
///                 `[a, b, c]`.
///
/// # Returns
/// A tuple representing the solution, if it exists.
fn solve(first_equ: &[isize], second_equ: &[isize]) -> Option<(isize, isize)> {
    let [a1, b1, c1] = first_equ else {
        return None;
    };
    let [a2, b2, c2] = second_equ else {
        return None;
    };

    // a_1 x + b_1 y = c_1
    //      -> y = (c_1 - a_1 x) / b_1
    // a_2 x + b_2 y = c_2
    //      -> y = (c_2 - a_2 x) / b_2
    //
    // (c_1 - a_1 x) / b_1 = (c_2 - a_2 x) / b_2
    //      -> b_1 (c_2 - a_2 x) = b_2 (c_1 - a_1 x)
    //      -> b_1 c_2 - b_1 a_2 x = b_2 c_1 - b_2 a_1 x
    //      -> b_2 a_1 x - b_1 a_2 x = b_2 c_1 - b_1 c_2
    //      -> x(b_2 a_1 - b_1 a_2) = b_2 c_1 - b_1 c_2
    //      -> x = (b_2 c_1 - b_1 c_2) / (b_2 a_1 - b_1 a_2)
    //
    // a_1 x + b_1 y = c_1
    //      -> y = (c_1 - a_1 x) / b_1
    let x_numerator = b2 * c1 - b1 * c2;
    let x_denominator = b2 * a1 - b1 * a2;
    let x = x_numerator as f64 / x_denominator as f64;
    if x.fract() != 0.0 {
        None
    } else {
        let y = (c1 - a1 * x as isize) / b1;
        Some((x as isize, y))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Button {
    x_change: isize,
    y_change: isize,
}

impl Button {
    pub fn new(x_change: isize, y_change: isize) -> Self {
        Self { x_change, y_change }
    }
}

struct Machine {
    button_a: Button,
    button_b: Button,
    prize_location: (isize, isize),
}

impl Machine {
    pub fn new(button_a: Button, button_b: Button, prize_location: (isize, isize)) -> Self {
        Self {
            button_a,
            button_b,
            prize_location,
        }
    }
}
