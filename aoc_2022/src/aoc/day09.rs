use std::collections::HashSet;

use crate::aoc::aoc_problem::{AoCProblem, Solution};

type Point = (isize, isize);

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
        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert((0, 0));
        let mut head_x: isize = 0;
        let mut head_y: isize = 0;
        let mut tail_x: isize = 0;
        let mut tail_y: isize = 0;

        for (dir, amt) in &self.motions {
            let mut to_process = *amt;
            while to_process > 0 {
                move_tail_unit_len(&mut head_x, &mut head_y, &mut tail_x, &mut tail_y, dir);
                visited.insert((tail_x, tail_y));
                to_process -= 1;
            }
        }

        visited.len().into()
    }

    fn part2(&mut self) -> Solution {
        0.into()
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

fn move_tail_unit_len(
    head_x: &mut isize,
    head_y: &mut isize,
    tail_x: &mut isize,
    tail_y: &mut isize,
    dir: impl AsRef<Direction>,
) {
    let dir = dir.as_ref();
    match dir {
        Direction::Right => *head_x += 1,
        Direction::Left => *head_x -= 1,
        Direction::Up => *head_y += 1,
        Direction::Down => *head_y -= 1,
    }

    let diff_x = (*head_x - *tail_x).abs();
    let diff_y = (*head_y - *tail_y).abs();
    if diff_x > 1 {
        if *head_x > *tail_x {
            *tail_x += 1;
        } else {
            *tail_x -= 1;
        }

        if *head_y != *tail_y {
            match dir {
                Direction::Right | Direction::Left => {
                    *tail_y += if *head_y > *tail_y { 1 } else { -1 }
                }
                Direction::Down | Direction::Up => {
                    *tail_x += if *head_x > *tail_x { 1 } else { -1 }
                }
            }
        }
    }

    if diff_y > 1 {
        if *head_y > *tail_y {
            *tail_y += 1;
        } else {
            *tail_y -= 1;
        }

        if *head_x != *tail_x {
            match dir {
                Direction::Right | Direction::Left => {
                    *tail_y += if *head_y > *tail_y { 1 } else { -1 }
                }
                Direction::Down | Direction::Up => {
                    *tail_x += if *head_x > *tail_x { 1 } else { -1 }
                }
            }
        }
    }
}
