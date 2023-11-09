use common::day::{AoCProblem, Solution};
use std::cmp::max;

// Remark: At a later time, I'll try to implement this using a binary tree (which would have been
// ideal for a problem like this, except it was late and I was -- and still am -- lazy).

const LEFT_BRACE: i32 = -1;
const RIGHT_BRACE: i32 = -2;
const COMMA: i32 = -3;

pub struct Day18 {
    homework: Vec<String>,
}

// https://adventofcode.com/2021/day/18
impl AoCProblem for Day18 {
    fn prepare(input: String) -> Self {
        Self {
            homework: input.lines().map(|x| x.to_string()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let hw: Vec<&str> = self.homework.iter().map(|x| x.as_str()).collect();
        solve_homework_problem(&hw).into()
    }

    fn part2(&mut self) -> Solution {
        let hw: Vec<&str> = self.homework.iter().map(|x| x.as_str()).collect();
        let mut max_mag: i32 = i32::MIN;
        for i in 0..hw.len() {
            for j in 0..hw.len() {
                if i == j {
                    continue;
                }

                max_mag = max(
                    max(
                        solve_homework_problem(&[hw[i], hw[j]]),
                        solve_homework_problem(&[hw[j], hw[i]]),
                    ),
                    max_mag,
                );
            }
        }

        max_mag.into()
    }
}

/// Parses a snail number line.
///
/// # Parameters
/// - `line`: The line to parse.
///
/// # Returns
/// A vector that represents the snail number.
fn parse_line(line: &str) -> Vec<i32> {
    line.chars()
        .map(|x| match x {
            '[' => LEFT_BRACE,
            ']' => RIGHT_BRACE,
            ',' => COMMA,
            _ => x.to_digit(10).unwrap() as i32,
        })
        .collect()
}

/// Adds two snail numbers together.
///
/// # Parameters
/// - `a`: The first snail number.
/// - `b`: The second snail number.
///
/// # Returns
/// A vector containing the new snail number.
fn add(a: &[i32], b: &[i32]) -> Vec<i32> {
    // a + b = [(all of a), (all of b)]
    let mut v: Vec<i32> = vec![LEFT_BRACE];
    v.extend(a);
    v.push(COMMA);
    v.extend(b);
    v.push(RIGHT_BRACE);
    v
}

/// Attempts to explode a snail number.
///
/// # Parameters
/// - `n`: The snail number.
///
/// # Returns
/// `true` if something changed; `false` otherwise.
fn explode(n: &mut Vec<i32>) -> bool {
    let mut b = 0;
    for i in 0..n.len() {
        if n[i] == LEFT_BRACE {
            b += 1;
            continue;
        }

        if n[i] == RIGHT_BRACE {
            b -= 1;
            continue;
        }

        // We use 5 because we also count the outer-most braces.
        if b < 5 {
            continue;
        }

        // If this is a valid pair...
        if n[i] == COMMA && n[i - 1] >= 0 && n[i + 1] >= 0 {
            let before_num = n[i - 1];
            let after_num = n[i + 1];
            // Look for the nearest valid left number to add to
            for j in (0..(i - 2)).rev() {
                if n[j] < 0 {
                    continue;
                }

                n[j] += before_num;
                break;
            }

            // Look for the nearest valid right number to add to
            for val in n.iter_mut().skip(i + 2) {
                if *val < 0 {
                    continue;
                }

                *val += after_num;
                break;
            }

            // Index i looks like:
            //      ...[z,[x,y]],k...
            //              ^ i
            // So, we remove everything from (i - 2), inclusive, to (i + 3), exclusive and then
            // put a singular 0 in its place:
            //      ...[z,0],k...
            //            ^ i
            n.splice((i - 2)..(i + 3), [0]);
            return true;
        }
    }

    false
}

/// Attempts to split a snail number.
///
/// # Parameters
/// - `n`: The snail number.
///
/// # Returns
/// `true` if something changed; `false` otherwise.
fn split(n: &mut Vec<i32>) -> bool {
    for i in 0..n.len() {
        match n[i] {
            LEFT_BRACE | RIGHT_BRACE | COMMA => continue,
            _ if n[i] < 10 => continue,
            _ => {
                let left = n[i] / 2;
                let right = n[i] - left;
                n.splice(i..(i + 1), [LEFT_BRACE, left, COMMA, right, RIGHT_BRACE]);
                return true;
            }
        };
    }

    false
}

/// Gets the string representation of this snail number. Useful for debugging.
///
/// # Parameters
/// - `n`: The snail number.
///
/// # Returns
/// The string representation of this snail number.
fn get_string_representation(n: &[i32]) -> String {
    let mut s: String = String::new();
    for c in n {
        match *c {
            LEFT_BRACE => s.push('['),
            RIGHT_BRACE => s.push(']'),
            COMMA => s.push(','),
            _ => s.push_str(&c.to_string()),
        };
    }

    s
}

/// Flattens the snail number, essentially "simplifying" it.
///
/// # Parameters
/// - `n`: The snail number.
///
/// # Returns
/// `true` if the snail number was flattened once and `false` otherwise.
fn flatten(n: &mut Vec<i32>) -> bool {
    for i in 0..n.len() {
        if n[i] == COMMA && n[i - 1] >= 0 && n[i + 1] >= 0 {
            let left = n[i - 1];
            let right = n[i + 1];
            // Remove from index (i - 2) 5 times. Since index i looks like:
            //          [x,y]
            //            ^
            //            i
            // We can remove everything from (i - 2), inclusive, to (i + 3), exclusive and then
            // put the result of the operation in its place; in this case, 3 * x + 2 * y.
            n.splice((i - 2)..(i + 3), [3 * left + 2 * right]);
            return true;
        }
    }

    false
}

/// Solves a homework problem.
///
/// # Parameters
/// - `problems`: The addition problems to solve.
///
/// # Returns
/// The result (magnitude).
fn solve_homework_problem(problems: &[&str]) -> i32 {
    let mut start = parse_line(problems[0]);
    process_line(&mut start);

    for p in problems.iter().skip(1) {
        start = add(&start, &parse_line(p));
        process_line(&mut start);
    }

    while flatten(&mut start) {}
    start[0]
}

/// Continuously processes a line until it can no longer be processed. This will continuously run
/// the `split` and `explode` methods.
///
/// # Parameters
/// - `n`: The snail number to process.
fn process_line(n: &mut Vec<i32>) {
    loop {
        if explode(n) {
            continue;
        }

        if split(n) {
            continue;
        }

        break;
    }
}
