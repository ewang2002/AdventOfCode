use std::{collections::HashSet, fmt::Debug};

use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day09 {
    motions: Vec<(Direction, usize)>,
}

impl AoCProblem for Day09 {
    fn prepare(input: &str) -> Self {
        Self {
            motions: input
                .lines()
                .map(|l| {
                    let (dir, amt) = l.split_once(' ').unwrap();
                    (dir.into(), amt.parse().unwrap())
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut points = vec![Point { x: 0, y: 0 }, Point { x: 0, y: 0 }];

        for (dir, amt) in &self.motions {
            let mut to_process = *amt;
            while to_process > 0 {
                move_tail_unit_len(&mut points, dir);
                visited.insert((points[1].x, points[1].y));
                to_process -= 1;
            }
        }

        visited.len().into()
    }

    fn part2(&mut self) -> Solution {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut points = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
        ];

        for (dir, amt) in &self.motions {
            let mut to_process = *amt;
            while to_process > 0 {
                move_tail_unit_len(&mut points, dir);
                visited.insert((points[9].x, points[9].y));
                to_process -= 1;
            }
        }

        visited.len().into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "L" => Self::Left,
            "D" => Self::Down,
            "U" => Self::Up,
            _ => unreachable!("{} should not be part of input", s),
        }
    }
}

impl AsRef<Direction> for Direction {
    fn as_ref(&self) -> &Direction {
        self
    }
}

struct Point {
    x: isize,
    y: isize,
}

/// Checks if two points are "touching." Two points are touching if they
/// are next to each other in some way (either vertically or horizontally
/// or diagonally).
///
/// # Parameters
/// - `p1`: A reference to the first point.
/// - `p2`: A reference to the second point.
///
/// # Returns
/// A `bool` value indicating if the points are touching each other.
fn are_points_touching(p1: &Point, p2: &Point) -> bool {
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if *x1 + dx == *x2 && *y1 + dy == *y2 {
                return true;
            }
        }
    }

    false
}

/// Moves the head one unit in the specified direction and then moves the remaining
/// points so that all points are still connected.
///
/// # Parameters
/// - `points`: The points. The first element should be the head and the last element
///             should be the tail.
/// - `dir`: The direction that the head should move towards.
fn move_tail_unit_len(points: &mut [Point], dir: impl AsRef<Direction>) {
    let dir = dir.as_ref();
    match dir {
        Direction::Right => points[0].x += 1,
        Direction::Left => points[0].x -= 1,
        Direction::Up => points[0].y += 1,
        Direction::Down => points[0].y -= 1,
    }

    for i in 0..points.len() - 1 {
        let window = &mut points[i..i + 2];

        let head_x = window[0].x;
        let head_y = window[0].y;
        let mut tail_x = window[1].x;
        let mut tail_y = window[1].y;

        if are_points_touching(&window[0], &window[1]) {
            continue;
        }

        // Are they in the same "level" (row- or column-wise)?
        if head_x == tail_x {
            tail_y += if head_y > tail_y { 1 } else { -1 };
        } else if head_y == tail_y {
            tail_x += if head_x > tail_x { 1 } else { -1 };
        } else {
            // Otherwise, we need to "push" the tail so it's
            // on the same level as the head.
            'm: for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 || dy == 0 {
                        continue;
                    }

                    if are_points_touching(
                        &window[0],
                        &Point {
                            x: tail_x + dx,
                            y: tail_y + dy,
                        },
                    ) {
                        tail_x += dx;
                        tail_y += dy;
                        break 'm;
                    }
                }
            }
        }

        window[1].x = tail_x;
        window[1].y = tail_y;
    }
}
