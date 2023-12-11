use common::problem::run;
use std::env;
mod aoc;

fn main() {
    let args = env::args().skip(1).take(2).collect::<Vec<_>>();
    if args.is_empty() {
        println!("Usage: ./aoc2023 <day> [test]");
        println!("\twhere <day> is an integer in [0, 25].");
        println!("\tand [test] is optionally a positive integer.");
        return;
    }

    let day_to_use = match args[0].parse::<u32>() {
        Ok(o) if o <= 25 => o,
        _ => {
            println!("Usage: ./aoc2023 <day> [test]");
            println!("\twhere <day> is an integer in [0, 25].");
            println!("\tand [test] is optionally a positive integer.");
            return;
        }
    };

    let test_case = if args.len() == 2 {
        args[1].parse::<u32>().ok()
    } else {
        None
    };

    match day_to_use {
        0 => run::<crate::aoc::Day00>(test_case),
        1 => run::<crate::aoc::Day01>(test_case),
        2 => run::<crate::aoc::Day02>(test_case),
        3 => run::<crate::aoc::Day03>(test_case),
        4 => run::<crate::aoc::Day04>(test_case),
        5 => run::<crate::aoc::Day05>(test_case),
        6 => run::<crate::aoc::Day06>(test_case),
        7 => run::<crate::aoc::Day07>(test_case),
        8 => run::<crate::aoc::Day08>(test_case),
        9 => run::<crate::aoc::Day09>(test_case),
        _ => {
            eprintln!("[Error] Day {day_to_use} has not been implemented yet.");
        }
    }
}
