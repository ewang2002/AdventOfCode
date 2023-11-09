use common::day::{AoCProblem, Solution};
use std::collections::HashSet;

type Point = (i32, i32);

pub struct Day20 {
    algorithm: Vec<bool>,
    image: HashSet<Point>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

// https://adventofcode.com/2021/day/20
impl AoCProblem for Day20 {
    fn prepare(input: String) -> Self {
        let algorithm = input
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .map(|x| x == '#')
            .collect::<Vec<_>>();
        assert_eq!(512, algorithm.len());

        let mut image: HashSet<Point> = HashSet::new();
        let mut row = 0;
        for line in input.lines().skip(2) {
            let mut col = 0;
            for c in line.chars() {
                if c != '#' {
                    col += 1;
                    continue;
                }

                image.insert((row, col));
                col += 1;
            }

            row += 1;
        }

        Self {
            algorithm,
            image,
            min_x: 0,
            min_y: 0,
            max_x: row,
            max_y: input.lines().nth(2).unwrap().len() as i32,
        }
    }

    fn part1(&mut self) -> Solution {
        enhance_image(
            &self.image,
            &self.algorithm,
            (self.min_x, self.min_y),
            (self.max_x, self.max_y),
            2,
        )
        .into()
    }

    fn part2(&mut self) -> Solution {
        enhance_image(
            &self.image,
            &self.algorithm,
            (self.min_x, self.min_y),
            (self.max_x, self.max_y),
            50,
        )
        .into()
    }
}

/// Enhances an image, returning the number of bright pixels.
///
/// # Parameters
/// - `image`: The set containing all bright points.
/// - `algorithm`: The algorithm to apply to the image.
/// - `min`: The top-left point of this image.
/// - `max`: The bottom-right point of this image.
/// - `num_apply`: The number of times to apply this operation.
///
/// # Returns
/// The number of bright pixels.
fn enhance_image(
    image: &HashSet<Point>,
    algorithm: &[bool],
    min: Point,
    max: Point,
    num_apply: usize,
) -> usize {
    let mut image = image.clone();
    let mut min_x = min.0;
    let mut min_y = min.1;
    let mut max_x = max.0;
    let mut max_y = max.1;

    let mut binary_str = String::new();
    for iterations in 0..num_apply {
        let mut temp_image: HashSet<Point> = HashSet::new();
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for (n_x, n_y) in get_surrounding_points((x, y)) {
                    if image.contains(&(n_x, n_y)) {
                        binary_str.push('1');
                        continue;
                    }

                    if algorithm[0]
                        && iterations & 1 == 1
                        && (n_x < min_x || n_y < min_y || n_x > max_x || n_y > max_y)
                    {
                        binary_str.push('1');
                        continue;
                    }

                    binary_str.push('0');
                }

                let b = usize::from_str_radix(&binary_str, 2).unwrap();
                if algorithm[b] {
                    temp_image.insert((x, y));
                }

                binary_str.clear();
            }
        }

        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;
        image = temp_image;
    }

    image.len()
}

/// Gets all surrounding points of a point.
///
/// # Parameter
/// - `p`: The point.
///
/// # Returns
/// The surrounding points.
#[inline(always)]
fn get_surrounding_points(p: Point) -> Vec<Point> {
    let (x, y) = p;
    let mut v = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            v.push((x + dx, y + dy));
        }
    }

    v
}
