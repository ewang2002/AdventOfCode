#![allow(dead_code)]

/// Transposes a vector of vectors of a copyable type.
///
/// # Parameters
/// - `v`: The vector of vector of type T, which is copyable.
///
/// # Returns
/// The transposed vector.
pub fn transpose_vec<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Copy {
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

/// Transposes a matrix of a copyable type.
///
/// # Parameters
/// - `v`: The vector of vector of type T, which is both copyable and has a default value.
///
/// # Returns
/// The transposed matrix.
pub fn transpose_mat<T, const R: usize, const C: usize>(v: &[[T; C]; R]) -> [[T; R]; C]
    where T: Copy + Default {
    let mut new_arr: [[T; R]; C] = [[T::default(); R]; C];
    for r in 0..R {
        for c in 0..C {
            new_arr[c][r] = v[r][c];
        }
    }

    return new_arr;
}