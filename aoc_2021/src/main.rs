use std::time::Instant;
use helpers::io;
use aoc::aoc_problem::AoCProblem;
use crate::aoc::day07::Day07;

mod aoc;
mod helpers;

fn main() {
    let input_file = io::file_read_all_lines("input/day07.txt");

    // Prepare to solve
    let mut start = Instant::now();
    let solver = Day07::prepare(input_file);
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
    println!("Input Parse : \t{} ms.\tor\t{} μs.", in_t.as_millis(), in_t.as_micros());
    println!("Part 1 Time : \t{} ms.\tor\t{} μs.", p1_t.as_millis(), p1_t.as_micros());
    println!("Part 2 Time : \t{} ms.\tor\t{} μs.", p2_t.as_millis(), p2_t.as_micros());
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
