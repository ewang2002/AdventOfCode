use std::{fs, path::Path, time::Instant};

use common::day::AoCProblem;

use crate::*;


/// Runs your solution to specified day.
///
/// # Parameters
/// - `day`: The day to run. This should be in the range [0, 25].
/// - `test_case`: The test case to run, if any. If `None`, then the
/// solution file is executed.
///
/// # Returns
/// A result representing whether the execution was successful or not.
pub fn run(day: u32, test_case: Option<u32>) -> RunResult {
    // Look for input file.
    let input_file = Path::new("input").join(if let Some(t) = test_case {
        format!("day{:02}_test{}.txt", day, t)
    } else {
        format!("day{:02}.txt", day)
    });

    if !input_file.exists() {
        return RunResult::InputFileNotFound(input_file);
    }

    let mut start = Instant::now();
    let input_str = match fs::read_to_string(&input_file) {
        Ok(o) => o,
        Err(_) => return RunResult::InputFileNotValid(input_file),
    };

    let mut solver: Box<dyn AoCProblem> = match day {
        1 => Box::new(aoc::Day01::prepare(input_str)),
        2 => Box::new(aoc::Day02::prepare(input_str)),
        3 => Box::new(aoc::Day03::prepare(input_str)),
        4 => Box::new(aoc::Day04::prepare(input_str)),
        5 => Box::new(aoc::Day05::prepare(input_str)),
        6 => Box::new(aoc::Day06::prepare(input_str)),
        7 => Box::new(aoc::Day07::prepare(input_str)),
        8 => Box::new(aoc::Day08::prepare(input_str)),
        9 => Box::new(aoc::Day09::prepare(input_str)),
        10 => Box::new(aoc::Day10::prepare(input_str)),
        11 => Box::new(aoc::Day11::prepare(input_str)),
        12 => Box::new(aoc::Day12::prepare(input_str)),
        13 => Box::new(aoc::Day13::prepare(input_str)),
        14 => Box::new(aoc::Day14::prepare(input_str)),
        15 => Box::new(aoc::Day15::prepare(input_str)),
        16 => Box::new(aoc::Day16::prepare(input_str)),
        17 => Box::new(aoc::Day17::prepare(input_str)),
        18 => Box::new(aoc::Day18::prepare(input_str)),
        20 => Box::new(aoc::Day20::prepare(input_str)),
        21 => Box::new(aoc::Day21::prepare(input_str)),
        22 => Box::new(aoc::Day22::prepare(input_str)),
        24 => Box::new(aoc::Day24::prepare(input_str)),
        25 => Box::new(aoc::Day25::prepare(input_str)),
        _ => return RunResult::ProblemNotFound(day),
    };

    let input_time = start.elapsed();

    // Part 1
    start = Instant::now();
    println!("Part 1 Solution: {}", solver.part1());
    let p1_t = start.elapsed();

    // Part 2
    start = Instant::now();
    println!("Part 2 Solution: {}", solver.part2());
    let p2_t = start.elapsed();

    // Execution ends, display time statistics.
    println!();
    match test_case {
        Some(t) => println!("[!] Running Code for Test Case {}.", t),
        None => println!("[.] Running Code for Solution."),
    }
    println!("Input Parse : \t{} ms.", input_time.as_millis());
    println!("Part 1 Time : \t{} ms.", p1_t.as_millis());
    println!("Part 2 Time : \t{} ms.", p2_t.as_millis());
    println!();
    println!("P1 + P2     : \t{} ms.", (p1_t + p2_t).as_millis(),);
    println!(
        "P + P1 + P2 : \t{} ms.",
        (input_time + p1_t + p2_t).as_millis(),
    );

    RunResult::Success
}
