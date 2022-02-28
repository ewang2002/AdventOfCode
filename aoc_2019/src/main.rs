use crate::aoc::AoCProblem;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod aoc;

/// A macro that can be used to automatically parse the input file and prepare the corresponding
/// problem structure for execution. To use:
/// ```
/// let mut solver = prepare_day!(DayXX);
/// ```
/// Where `XX` is the day that you want to execute. For example, to execute:
/// - The solutions for Day 3, use `03`: `prepare_day!(Day03);`
/// - The solutions for Day 10, use `10`: `prepare_day!(Day10);`
macro_rules! prepare_day {
    ($day: ident) => {{
        use crate::aoc::*;
        let file = File::open(format!("input/{}.txt", stringify!($day).to_lowercase()).as_str())
            .expect("No such file found.");
        let input_file = BufReader::new(file)
            .lines()
            .map(|l| l.expect("Line could not be parsed."))
            .collect();
        $day::prepare(input_file)
    }};
}

fn main() {
    // Prepare to solve
    let mut start = Instant::now();

    // Change this to the correct day!
    let mut solver = prepare_day!(Day01);
    let in_t = start.elapsed();

    // Execution begins
    // Part 1
    start = Instant::now();
    println!("Part 1 Solution: {}", solver.part1());
    let p1_t = start.elapsed();

    // Part 2
    start = Instant::now();
    println!("Part 2 Solution: {}", solver.part2());
    let p2_t = start.elapsed();

    // Execution ends
    println!();
    println!(
        "Input Parse : \t{} ms.\tor\t{} μs.",
        in_t.as_millis(),
        in_t.as_micros()
    );
    println!(
        "Part 1 Time : \t{} ms.\tor\t{} μs.",
        p1_t.as_millis(),
        p1_t.as_micros()
    );
    println!(
        "Part 2 Time : \t{} ms.\tor\t{} μs.",
        p2_t.as_millis(),
        p2_t.as_micros()
    );
    println!();
    println!(
        "P1 + P2     : \t{} ms.\tor\t{} μs.",
        (p1_t + p2_t).as_millis(),
        (p1_t + p2_t).as_micros()
    );
    println!(
        "P + P1 + P2 : \t{} ms.\tor\t{} μs.",
        (in_t + p1_t + p2_t).as_millis(),
        (in_t + p1_t + p2_t).as_micros()
    );
}
