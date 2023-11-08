/// The part of the puzzle (either part 1 or 2).
#[derive(PartialEq, Eq, Debug)]
pub enum AocPart {
    One,
    Two,
}

#[cfg(windows)]
pub const TWO_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
pub const TWO_NEWLINE: &str = "\n\n";
