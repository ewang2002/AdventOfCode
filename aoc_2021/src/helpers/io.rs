use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads all lines from a file. Returns a string.
///
/// # Parameters
/// * `filename` - The file name.
///
/// # Returns
/// * A vector of strings. Each string represents a line in the file.
pub fn file_read_all_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No such file found.");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Line could not be parsed."))
        .collect()
}
