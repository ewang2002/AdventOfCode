#[cfg(windows)]
pub const TWO_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
pub const TWO_NEWLINE: &str = "\n\n";
