use std::{cmp::max, collections::HashSet};

use common::problem::day::{AoCProblem, Solution};

pub struct Day10 {
    map: Vec<Vec<char>>,
}

// https://adventofcode.com/2019/day/10
impl AoCProblem for Day10 {
    fn prepare(input: String) -> Self {
        Self {
            map: input.lines().map(|x| x.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut max_astroids = 0;

        for (h_idx, row) in self.map.iter().enumerate() {
            for (w_idx, col) in row.iter().enumerate() {
                if *col == '#' {
                    max_astroids = max(
                        max_astroids,
                        count_direct_line_of_sight(&self.map, h_idx, w_idx),
                    );
                }
            }
        }

        max_astroids.into()
    }

    fn part2(&mut self) -> Solution {
        0.into()
    }

    fn day() -> u32 {
        10
    }

    fn year() -> u32 {
        2019
    }
}

const BASE_DIRECTIONS: [[isize; 2]; 4] = [
    [0, 1],  // up
    [1, 0],  // right
    [0, -1], // down
    [-1, 0], // left
];

const REGION_ANGLE_DIRECTIONS: [[isize; 2]; 4] = [
    // Region A
    [1, 1],
    // Region B
    [1, -1],
    // Region C
    [-1, -1],
    // Region D
    [-1, 1],
];

/// Counts the number of '#' seen by the current asteroid at coordinate (h, w),
/// where `h` is the height of the map and `w` is the width of the map.
fn count_direct_line_of_sight(map: &[Vec<char>], h: usize, w: usize) -> usize {
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    // First, check the general base directions
    let mut c = 0;

    for [dh, dw] in BASE_DIRECTIONS {
        let mut t_h = h as isize;
        let mut t_w = w as isize;

        loop {
            t_h += dh;
            t_w += dw;

            if t_h < 0 || t_w < 0 || t_h >= height || t_w >= width {
                break;
            }

            if map[t_h as usize][t_w as usize] == '#' {
                c += 1;
                break;
            }
        }
    }

    // General approach for checking at various angles
    // Let's suppose we have the following graph:
    //                      x
    //                      |
    //                      |
    //               C      |     D
    //                      |
    //         -------------+------------- y
    //                      |
    //               B      |     A
    //                      |
    //                      |
    //
    // Consider region A. The idea is to consider, for each
    // column (i.e., each dw), all possible dh values, and then
    // to use that as the rate of change.
    //
    // Add 1 to dh, then consider all possible dw's from dh = 1.
    // So, for example,
    // - dh = 1, dw = 1: (1, 1), (2, 2), (3, 3), (4, 4), ...
    // - dh = 1, dw = 2: (1, 2), (2, 4), (3, 6), (4, 8), ...
    // - dh = 1, dw = 3: (1, 3), (2, 6), (3, 9), (4, 12), ...
    //  ...
    // - dh = 5, dw = 1: (5, 1), (10, 2), (15, 3), ...
    //  ...
    // We keep repeating this process until our dh reaches a point
    // where we just go out of bounds.
    //
    // For the whole process, we keep repeating this for the four
    // regions.

    // Consider region A.
    for [dhp, dwp] in REGION_ANGLE_DIRECTIONS {
        let mut dh = dhp;
        let mut dw = dwp;
        let mut seen_slopes = HashSet::<(isize, isize)>::new();
        let mut num_no_iters = 0;

        loop {
            let this_slope = simplify_fraction(dh, dw);
            if seen_slopes.contains(&this_slope) {
                dw += dwp;
                continue;
            }

            seen_slopes.insert(this_slope);

            let mut temp_h = h as isize;
            let mut temp_w = w as isize;
            let mut iterated = false;

            loop {
                temp_h += dh;
                temp_w += dw;

                if temp_h < 0 || temp_w < 0 || temp_h >= height || temp_w >= width {
                    break;
                }

                iterated = true;
                if map[temp_h as usize][temp_w as usize] == '#' {
                    c += 1;
                    break;
                }
            }

            // If we iterated at least once, that means that we might still need
            // to process another possible slope.
            if iterated {
                dw += dwp;
                num_no_iters = 0;
                continue;
            }

            // Otherwise, this means we're done with that row.
            // Let's consider the next row. But, first, if
            // we didn't even process the previous row, then
            // we can just break out.
            if num_no_iters >= 2 {
                break;
            }

            num_no_iters += 1;
            dh += dhp;
            dw = dwp;
        }
    }

    c
}

/// Finds the GCD of the absolute values of `n` and `m`.
///
/// # Parameters
/// - `n`: the first number.
/// - `m`: the second number.
///
/// # Returns
/// `gcd(|n|, |m|)`.
fn gcd(n: isize, m: isize) -> isize {
    let mut t_n = n.abs();
    let mut t_m = m.abs();
    while t_m != 0 {
        if t_m < t_n {
            std::mem::swap(&mut t_m, &mut t_n);
        }
        t_m %= t_n;
    }

    t_n
}

/// Simplifies the given fraction, represented as `numerator/denominator`.
///
/// # Parameters
/// - `numerator`: the numerator of this fraction.
/// - `denominator`: the denominator of this fraction.
///
/// # Returns
/// The simplified fraction.
fn simplify_fraction(numerator: isize, denominator: isize) -> (isize, isize) {
    let g = gcd(numerator, denominator);
    (numerator / g, denominator / g)
}
