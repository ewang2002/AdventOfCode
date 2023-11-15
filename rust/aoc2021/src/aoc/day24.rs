use common::problem::day::{AoCProblem, Solution};
use std::collections::HashMap;

const MIN_TO_MAX: [i64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const MAX_TO_MIN: [i64; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];

pub struct Day24 {
    instructions: Vec<ALUInstruction>,
}

impl AoCProblem for Day24 {
    fn prepare(input: String) -> Self {
        let mut instructions: Vec<ALUInstruction> = vec![];
        for line in input.lines() {
            let mut split_line = line.split(' ');
            let op = match split_line.next().unwrap() {
                "inp" => Op::Inp,
                "add" => Op::Add,
                "mul" => Op::Mul,
                "div" => Op::Div,
                "mod" => Op::Mod,
                "eql" => Op::Eql,
                _ => panic!("Unknown operator."),
            };

            let first = split_line.next().unwrap().chars().next().unwrap();
            let second = match op {
                Op::Inp => SecondArg::None,
                _ => {
                    let sec = split_line.next().unwrap();
                    if sec == "w" || sec == "x" || sec == "y" || sec == "z" {
                        SecondArg::Var(sec.chars().next().unwrap())
                    } else {
                        SecondArg::Num(sec.parse::<_>().unwrap())
                    }
                }
            };

            instructions.push(ALUInstruction {
                first,
                second,
                operator: op,
            });
        }

        Self { instructions }
    }

    fn part1(&mut self) -> Solution {
        run(
            &self.instructions,
            0,
            &mut HashMap::new(),
            0,
            0,
            0,
            0,
            MAX_TO_MIN,
        )
        .1
        .into()
    }

    fn part2(&mut self) -> Solution {
        run(
            &self.instructions,
            0,
            &mut HashMap::new(),
            0,
            0,
            0,
            0,
            MIN_TO_MAX,
        )
        .1
        .into()
    }

    fn day() -> u32 {
        24
    }

    fn year() -> u32 {
        2021
    }
}

type CacheKey = (usize, i64, i64, i64, i64);
type CacheValue = (bool, i64);
type Cache = HashMap<CacheKey, CacheValue>;

/// Runs the instruction set, looking for the highest or lowest value that gives a `z`-value of 0.
///
/// # Parameters
/// - `instructions`: The instruction set.
/// - `i`: The current index for `instructions`.
/// - `cache`: The cache.
/// - `w`: The `w`-value.
/// - `x`: The `x`-value.
/// - `y`: The `y`-value.
/// - `z`: The `z`-value.
/// - `digits`: The digits to consider. Order matters.
///
/// # Returns
/// Whether `z` is 0, and the corresponding value (or `-1` if no such value exists).
#[allow(clippy::too_many_arguments)]
fn run(
    instructions: &[ALUInstruction],
    i: usize,
    cache: &mut Cache,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    digits: [i64; 9],
) -> (bool, i64) {
    // Find some upperbound for z. Assume that if we go above this value, we won't get 0.
    // Lower power = faster but less accurate (note that using 10^6 gives a >14 digit number)
    // Higher power = slower but more accurate
    if z >= 10_i64.pow(7) {
        return (false, -1);
    }

    let c_res = cache.get(&(i, w, x, y, z));
    if let Some(val) = c_res {
        return *val;
    }

    if i >= instructions.len() {
        return (z == 0, -1);
    }

    let mut v: [i64; 4] = [w, x, y, z];
    let is = &instructions[i];
    match is.operator {
        Op::Inp => {
            for n in digits {
                v[idx(is.first)] = n;
                let (valid, val) = run(instructions, i + 1, cache, v[0], v[1], v[2], v[3], digits);

                let key = (i + 1, v[0], v[1], v[2], v[3]);

                if valid {
                    let res = if val == -1 {
                        (valid, n)
                    } else {
                        (valid, n * 10_i64.pow(val.to_string().len() as u32) + val)
                    };

                    cache.insert(key, res);
                    return cache_and_return(cache, key, res);
                }

                cache.insert(key, (valid, val));
            }

            return cache_and_return(cache, (i + 1, v[0], v[1], v[2], v[3]), (false, -1));
        }
        Op::Add => v[idx(is.first)] += val(&is.second, &v),
        Op::Mul => v[idx(is.first)] *= val(&is.second, &v),
        Op::Div => v[idx(is.first)] /= val(&is.second, &v),
        Op::Mod => v[idx(is.first)] %= val(&is.second, &v),
        Op::Eql => {
            v[idx(is.first)] = if v[idx(is.first)] == val(&is.second, &v) {
                1
            } else {
                0
            }
        }
    };

    let r = run(instructions, i + 1, cache, v[0], v[1], v[2], v[3], digits);
    cache_and_return(cache, (i + 1, v[0], v[1], v[2], v[3]), r)
}

/// Caches and returns the cached value.
///
/// # Parameters
/// - `cache`: The cache.
/// - `key`: The key to cache.
/// - `value`: The value corresponding to the key to cache.
///
/// # Returns
/// The cache value.
fn cache_and_return(cache: &mut Cache, key: CacheKey, value: CacheValue) -> CacheValue {
    cache.insert(key, value);
    value
}

/// Gets the index of the register for some variable.
///
/// # Parameters
/// - `c`: The variable.
///
/// # Returns
/// The index in the `v` array.
fn idx(c: char) -> usize {
    match c {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => unreachable!(),
    }
}

/// Gets the value of the second argument.
///
/// # Parameters
/// - `second_arg`: The second argument.
/// - `vars`: The variable register.
///
/// # Returns
/// The value contained, or referred to, by the second argument.
fn val(second_arg: &SecondArg, vars: &[i64; 4]) -> i64 {
    match second_arg {
        SecondArg::Num(n) => *n,
        SecondArg::Var(n) => vars[idx(*n)],
        SecondArg::None => panic!("second argument is invalid"),
    }
}

enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

enum SecondArg {
    Num(i64),
    Var(char),
    None,
}

struct ALUInstruction {
    operator: Op,
    first: char,
    second: SecondArg,
}
