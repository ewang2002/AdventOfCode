#![allow(dead_code)]

/// Transposes a vector of vectors of a copyable type.
///
/// # Parameters
/// - `v`: The vector of vector of type T, which is copyable.
///
/// # Returns
/// The transposed vector.
pub fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Copy {
    if v.is_empty() {
        return vec![];
    }

    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    return (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| *n.next().unwrap())
                .collect::<Vec<T>>()
        }).collect();
}