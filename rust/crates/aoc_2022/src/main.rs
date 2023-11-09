use run::run;
use std::{env, path::PathBuf};

mod aoc;
mod run;

/// Main function.
fn main() {
    let args = env::args().skip(1).take(2).collect::<Vec<_>>();
    if args.is_empty() {
        println!("Usage: ./aoc_2022 <day> [test]");
        println!("\twhere <day> is an integer in [0, 25].");
        println!("\tand [test] is optionally a positive integer.");
        return;
    }

    let day_to_use = match args[0].parse::<u32>() {
        Ok(o) if o <= 25 => o,
        _ => {
            println!("Usage: ./aoc_2022 <day> [test]");
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

    match run(day_to_use, test_case) {
        RunResult::InputFileNotFound(f) => {
            eprintln!("[Error] The input file, {:?}, was not found.", f);
        }
        RunResult::InputFileNotValid(f) => {
            eprintln!("[Error] The input file, {:?}, could not be read.", f);
        }
        RunResult::ProblemNotFound(d) => {
            eprintln!("[Error] Day {} has not been implemented yet.", d);
        }
        _ => {}
    }
}

pub enum RunResult {
    /// The result was successful.
    Success,

    /// The input file was not found.
    InputFileNotFound(PathBuf),

    /// The input file could not be read.
    InputFileNotValid(PathBuf),

    /// The problem was not found.
    ProblemNotFound(u32),
}
