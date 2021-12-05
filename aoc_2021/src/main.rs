use std::time::Instant;
use helpers::io;
use aoc::aoc_problem::AoCProblem;
use crate::aoc::day04::Day04;

mod aoc;
mod helpers;

fn main() {
    let input_file = io::file_read_all_lines("input/day04.txt");

    // Prepare to solve
    let solver = Day04::prepare(input_file);

    // Execution begins
    let start = Instant::now();

    // Part 1
    println!("Part 1 Solution: {}", solver.part1());
    let part1_time = start.elapsed();

    // Part 2
    println!("Part 2 Solution: {}", solver.part2());
    let part2_time = start.elapsed();

    // Execution ends
    println!();
    println!("Part 1 Time : \t{} ms.", part1_time.as_millis());
    println!("Part 2 Time : \t{} ms.", part2_time.as_millis());
    println!("Total Time  : \t{} ms.", (part1_time + part2_time).as_millis());
}
