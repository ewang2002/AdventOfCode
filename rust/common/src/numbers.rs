/// Computes the GCD of two integers.
///
/// # Parameters
/// - `a`: The first integer.
/// - `b`: The second integer.
///
/// # Returns
/// The GCD of the two integers.
pub fn gcd<T>(n: T, m: T) -> usize
where
    T: Into<usize>,
{
    let mut n = n.into();
    let mut m = m.into();
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }

    n
}

/// Computes the LCM of two integers.
///
/// # Parameters
/// - `a`: The first integer.
/// - `b`: The second integer.
///
/// # Returns
/// The LCM of the two integers.
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
