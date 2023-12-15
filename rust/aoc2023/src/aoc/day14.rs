use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

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
        const CYCLES_TO_COMPLETE: usize = 1000000000;
        let mut rounded_rocks = self.rounded_rocks.clone();

        // Key = state
        // Value = index that state was seen at
        let mut seen_states: HashMap<Vec<Point>, usize> = HashMap::new();

        // Figure out where the cycle starts
        let mut cycles_completed = 0;
        while cycles_completed < CYCLES_TO_COMPLETE {
            // north -> west -> south -> east
            let r1 = tilt_north(&rounded_rocks, &self.cube_rocks);
            let r2 = tilt_west(&r1, &self.cube_rocks);
            let r3 = tilt_south(&r2, &self.cube_rocks, self.height);
            let r4 = tilt_east(&r3, &self.cube_rocks, self.height);
            rounded_rocks = r4;
            cycles_completed += 1;

            if seen_states.contains_key(&rounded_rocks) {
                break;
            }

            seen_states.insert(rounded_rocks.clone(), cycles_completed);
        }

        let last_seen_state = seen_states.get(&rounded_rocks).unwrap();
        let cycle_length = cycles_completed - last_seen_state;

        // Now that we know what the cycle length is, along with the number of cycles completed,
        // we can "jump" ahead towards the final state
        //
        // CYCLES_TO_COMPLETE - cycles_completed
        // => Gives us the number of cycles left to complete. Note that this does NOT tell us where
        //    the cycle started, just that the cycle is of that length. So, we'll need to apply an
        //    offset to get to the correct base state (where the cycle begins).
        //
        // ((CYCLES_TO_COMPLETE - cycles_completed) / cycle_length)
        // => Gives us the number of cycles left to complete, in terms of the cycle length
        //
        // ((CYCLES_TO_COMPLETE - cycles_completed) / cycle_length) * cycle_length
        // => Maps the base index 0 (cycles_completed = 0) to the corresponding index that is before the
        //    final state.
        //
        // ((CYCLES_TO_COMPLETE - cycles_completed) / cycle_length) * cycle_length + last_seen_state
        // => Correctly maps the previous value so that it maps with the first state in the original cycle.
        cycles_completed = ((CYCLES_TO_COMPLETE - cycles_completed) / cycle_length) * cycle_length
            + last_seen_state;
        while cycles_completed < CYCLES_TO_COMPLETE {
            let r1 = tilt_north(&rounded_rocks, &self.cube_rocks);
            let r2 = tilt_west(&r1, &self.cube_rocks);
            let r3 = tilt_south(&r2, &self.cube_rocks, self.height);
            let r4 = tilt_east(&r3, &self.cube_rocks, self.height);
            rounded_rocks = r4;
            cycles_completed += 1;
        }

        // Compute the total load
        let mut total_load = 0;
        for (row, _) in rounded_rocks {
            total_load += self.height - row;
        }

        total_load.into()
    }

    fn day() -> u32 {
        14
    }

    fn year() -> u32 {
        2023
    }
}

/// Tilts the rocks east, returning the new positions of the rocks.
///
/// # Parameters
/// - `rounded_rocks`: The positions of the rounded rocks.
/// - `cube_rocks`: The positions of the cube rocks.
///
/// # Returns
/// The new positions of the rounded rocks.
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

            match col1.cmp(&col2) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => row1.cmp(&row2),
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

/// Tilts the rocks west, returning the new positions of the rocks.
///
/// # Parameters
/// - `rounded_rocks`: The positions of the rounded rocks.
/// - `cube_rocks`: The positions of the cube rocks.
///
/// # Returns
/// The new positions of the rounded rocks.
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

            match col1.cmp(&col2) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => row1.cmp(&row2),
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

/// Tilts the rocks south, returning the new positions of the rocks.
///
/// # Parameters
/// - `rounded_rocks`: The positions of the rounded rocks.
/// - `cube_rocks`: The positions of the cube rocks.
///
/// # Returns
/// The new positions of the rounded rocks.
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

            match row1.cmp(&row2) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => col1.cmp(&col2),
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

/// Tilts the rocks north, returning the new positions of the rocks.
///
/// # Parameters
/// - `rounded_rocks`: The positions of the rounded rocks.
/// - `cube_rocks`: The positions of the cube rocks.
///
/// # Returns
/// The new positions of the rounded rocks.
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

            match row1.cmp(&row2) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => col1.cmp(&col2),
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
