use std::{fs, path::Path, time::Instant};

use crate::*;

use crate::aoc::{self, AoCProblem};

/// Runs the specified day.
///
/// # Parameters
/// - `day`: The day to run. This should be in the range [0, 25].
///
/// # Returns
/// A result representing whether the execution was successful or not.
pub fn run(day: u32) -> RunResult {
    // Look for input file.
    let input_file = Path::new("input").join(format!("day{:02}.txt", day));
    if !input_file.exists() {
        return RunResult::InputFileNotFound(input_file);
    }

    let mut start = Instant::now();
    let input_str = match fs::read_to_string(&input_file) {
        Ok(o) => o,
        Err(_) => return RunResult::InputFileNotValid(input_file),
    };

    let content = input_str.lines().collect::<Vec<_>>();

    let mut aoc_problem: Box<dyn AoCProblem<_, _>> = match day {
        0 => Box::new(aoc::Day00::prepare(content)),
        1 => Box::new(aoc::Day01::prepare(content)),
        _ => return RunResult::ProblemNotFound(day),
    };

    let input_time = start.elapsed();

    // Part 1
    start = Instant::now();
    aoc_problem.part1();
    let p1_t = start.elapsed();

    // Part 2
    start = Instant::now();
    aoc_problem.part2();
    let p2_t = start.elapsed();

    // Execution ends, display time statistics.
    println!();
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
