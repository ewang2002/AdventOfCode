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
        let mut points = vec![Point { x: 0, y: 0 }; 2];

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
        let mut points = vec![Point { x: 0, y: 0 }; 10];
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

#[derive(Clone)]
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
#[inline(always)]
fn are_points_touching(p1: &Point, p2: &Point) -> bool {
    let Point { x: x1, y: y1 } = p1;
    let Point { x: x2, y: y2 } = p2;
    // If any component of the two points (so the x or y component)
    // have a difference that is 2 or greater, then that means that
    // component isn't touching and thus the points themselves are
    // not touching. For example, points (0, 0) and (1, 1) are
    // touching because both components are less than 2:
    //
    //                  |
    //                  |P
    //            ------P-----
    //                  |
    //                  |
    //
    // But points (0, 0) and (1, 2) are not touching because the y
    // component is 2 or more.
    //
    //                  |
    //                  | P
    //            ------P-----
    //                  |
    //                  |
    //
    (*x1 - *x2).abs() <= 1 && (*y1 - *y2).abs() <= 1
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
        let head_x = points[i].x;
        let head_y = points[i].y;
        let tail_x = points[i + 1].x;
        let tail_y = points[i + 1].y;

        if are_points_touching(&points[i], &points[i + 1]) {
            continue;
        }

        // Here, we assume the points are not touching. Recall that the
        // signum method returns
        // - 1 if the number is positive
        // - 0 if the number is zero
        // - -1 if the number is negative
        //
        // There are two cases that we need to account for when we need to
        // get the points to touch.
        //
        // CASE 1: "If the head is ever two steps directly up, down, left,
        // or right from the tail, the tail must also move one step in that
        // direction so it remains close enough." In this case, we just need
        // to add ONE to the corresponding component so that both the head and
        // tail are touching. We note that, WLOG, if the x-component of the
        // head and tail are the same, the difference of that component will be
        // 0 so 0.signum() will give us 0 (meaning no change when we add to the
        // component). Otherwise, we'll get 1 or -1, which is the correct offset
        // to add to one of the tail's components.
        //
        // CASE 2: "if the head and tail aren't touching and aren't in the
        // same row or column, the tail always moves one step diagonally to
        // keep up." Here, we just need to add the correct offset to each component
        // of the tail so the tail and head touches. In this case, we note that both
        // components of both points will differ, so signum() will always return a
        // non-zero integer.
        //
        // In essence, the point I'm making is that both cases above can be handled
        // by the two statements below.
        points[i + 1].x += (head_x - tail_x).signum();
        points[i + 1].y += (head_y - tail_y).signum();
    }
}

// https://github.com/ewang2002/AdventOfCode/blob/ee27036e97f226a712a70d590eee92bbad5f9b90/aoc_2022/src/aoc/day09.rs
// for the original implementation.
