use std::{cmp::Ordering, collections::BinaryHeap};

use common::problem::day::{AoCProblem, Solution};

type Point = (usize, usize);

pub struct Day14 {
    rounded_rocks: Vec<Point>,
    cube_rocks: Vec<Point>,
    height: usize,
}

impl AoCProblem for Day14 {
    fn prepare(input: String) -> Self {
        let mut rounded_rocks: Vec<Point> = vec![];
        let mut cube_rocks: Vec<Point> = vec![];
        let height = input.lines().count();

        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                match c {
                    '#' => cube_rocks.push((row_idx, col_idx)),
                    'O' => rounded_rocks.push((row_idx, col_idx)),
                    _ => {}
                }
            }
        }

        Self {
            rounded_rocks,
            cube_rocks,
            height,
        }
    }

    fn part1(&mut self) -> Solution {
        // For each round rock, determine how far up the rock can go
        // without encountering a cube rock.
        let mut total_load = 0;
        for (row, _) in tilt_north(&self.rounded_rocks, &self.cube_rocks) {
            total_load += self.height - row;
        }

        total_load.into()
    }

    fn part2(&mut self) -> Solution {
        // Implementation idea: Keep track of all states (rounded rocks) we've seen so far.
        // Eventually, we'll reach a state we've seen before.

        // 100467 - too low
        0.into()
    }

    fn day() -> u32 {
        14
    }

    fn year() -> u32 {
        2023
    }
}

#[allow(dead_code)]
fn tilt_east(rounded_rocks: &[Point], cube_rocks: &[Point], height: usize) -> Vec<Point> {
    #[derive(Debug, PartialEq, Eq)]
    struct EastOrderPt(Point);

    impl PartialOrd for EastOrderPt {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for EastOrderPt {
        fn cmp(&self, other: &Self) -> Ordering {
            let (row1, col1) = self.0;
            let (row2, col2) = other.0;

            if col1 < col2 {
                Ordering::Less
            } else if col1 > col2 {
                Ordering::Greater
            } else {
                row1.cmp(&row2)
            }
        }
    }

    let mut heap: BinaryHeap<EastOrderPt> = BinaryHeap::new();
    for p in rounded_rocks {
        heap.push(EastOrderPt(*p));
    }

    let mut set_round_rocks = vec![];
    while let Some(EastOrderPt((row, col))) = heap.pop() {
        let mut new_col = col;
        while new_col < height - 1
            && !cube_rocks.contains(&(row, new_col + 1))
            && !set_round_rocks.contains(&(row, new_col + 1))
        {
            new_col += 1;
        }

        set_round_rocks.push((row, new_col));
    }

    set_round_rocks
}

#[allow(dead_code)]
fn tilt_west(rounded_rocks: &[Point], cube_rocks: &[Point]) -> Vec<Point> {
    #[derive(Debug, PartialEq, Eq)]
    struct WestOrderPt(Point);

    impl PartialOrd for WestOrderPt {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for WestOrderPt {
        fn cmp(&self, other: &Self) -> Ordering {
            let (row1, col1) = self.0;
            let (row2, col2) = other.0;

            if col1 > col2 {
                Ordering::Less
            } else if col1 < col2 {
                Ordering::Greater
            } else {
                row1.cmp(&row2)
            }
        }
    }

    let mut heap: BinaryHeap<WestOrderPt> = BinaryHeap::new();
    for p in rounded_rocks {
        heap.push(WestOrderPt(*p));
    }

    let mut set_round_rocks = vec![];
    while let Some(WestOrderPt((row, col))) = heap.pop() {
        let mut new_col = col;
        while new_col > 0
            && !cube_rocks.contains(&(row, new_col - 1))
            && !set_round_rocks.contains(&(row, new_col - 1))
        {
            new_col -= 1;
        }

        set_round_rocks.push((row, new_col));
    }

    set_round_rocks
}

#[allow(dead_code)]
fn tilt_south(rounded_rocks: &[Point], cube_rocks: &[Point], height: usize) -> Vec<Point> {
    #[derive(Debug, PartialEq, Eq)]
    struct SouthOrderPt(Point);

    impl PartialOrd for SouthOrderPt {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for SouthOrderPt {
        fn cmp(&self, other: &Self) -> Ordering {
            let (row1, col1) = self.0;
            let (row2, col2) = other.0;

            if row1 < row2 {
                Ordering::Less
            } else if row1 > row2 {
                Ordering::Greater
            } else {
                col1.cmp(&col2)
            }
        }
    }

    let mut heap: BinaryHeap<SouthOrderPt> = BinaryHeap::new();
    for p in rounded_rocks {
        heap.push(SouthOrderPt(*p));
    }

    let mut set_round_rocks = vec![];

    while let Some(SouthOrderPt((row, col))) = heap.pop() {
        let mut new_row = row;
        while new_row < height - 1
            && !cube_rocks.contains(&(new_row + 1, col))
            && !set_round_rocks.contains(&(new_row + 1, col))
        {
            new_row += 1;
        }

        set_round_rocks.push((new_row, col));
    }

    set_round_rocks
}

#[allow(dead_code)]
fn tilt_north(rounded_rocks: &[Point], cube_rocks: &[Point]) -> Vec<Point> {
    #[derive(Debug, PartialEq, Eq)]
    struct NorthOrderPt(Point);

    impl PartialOrd for NorthOrderPt {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for NorthOrderPt {
        fn cmp(&self, other: &Self) -> Ordering {
            let (row1, col1) = self.0;
            let (row2, col2) = other.0;

            if row1 > row2 {
                Ordering::Less
            } else if row1 < row2 {
                Ordering::Greater
            } else {
                col1.cmp(&col2)
            }
        }
    }

    let mut heap: BinaryHeap<NorthOrderPt> = BinaryHeap::new();
    for p in rounded_rocks {
        heap.push(NorthOrderPt(*p));
    }

    let mut set_round_rocks = vec![];
    while let Some(NorthOrderPt((row, col))) = heap.pop() {
        let mut new_row = row;
        while new_row > 0
            && !cube_rocks.contains(&(new_row - 1, col))
            && !set_round_rocks.contains(&(new_row - 1, col))
        {
            new_row -= 1;
        }

        set_round_rocks.push((new_row, col));
    }

    set_round_rocks
}
