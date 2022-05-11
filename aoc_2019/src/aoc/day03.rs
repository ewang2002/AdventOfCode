use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, HashMap},
};

use crate::aoc::aoc_problem::AoCProblem;

pub struct Day03 {
    first_wire: Vec<WireComponent>,
    second_wire: Vec<WireComponent>,
}

// https://adventofcode.com/2019/day/3
impl AoCProblem<usize, usize> for Day03 {
    fn prepare(input: Vec<String>) -> Self {
        Self {
            first_wire: input[0].split(',').map(WireComponent::new).collect(),
            second_wire: input[1].split(',').map(WireComponent::new).collect(),
        }
    }

    fn part1(&mut self) -> usize {
        let mut set: HashSet<(i32, i32)> = HashSet::new();

        // For wire 1
        let mut curr_x = 0;
        let mut curr_y = 0;
        for wire in &self.first_wire {
            let (dx, dy, max) = get_dir(wire);
            let mut i = 0;
            while i < max {
                // One of dx or dy will be non-zero
                curr_x += dx;
                curr_y += dy;
                i += 1;
                set.insert((curr_x, curr_y));
            }
        }

        let mut heap = BinaryHeap::new();

        // For wire 2
        curr_x = 0;
        curr_y = 0;
        for wire in &self.second_wire {
            let (dx, dy, max) = get_dir(wire);
            let mut i = 0;

            while i < max {
                // One of dx or dy will be non-zero
                curr_x += dx;
                curr_y += dy;
                i += 1;
                if set.contains(&(curr_x, curr_y)) {
                    set.remove(&(curr_x, curr_y));
                    heap.push(Reverse(curr_x.abs() + curr_y.abs()));
                }
            }
        }

        heap.pop().expect("Nothing in the heap.").0 as usize
    }

    fn part2(&mut self) -> usize {
        let mut map: HashMap<(i32, i32), usize> = HashMap::new();

        // For wire 1
        let mut curr_x = 0;
        let mut curr_y = 0;
        let mut steps = 0;
        for wire in &self.first_wire {
            let (dx, dy, max) = get_dir(wire);
            let mut i = 0;
            while i < max {
                // One of dx or dy will be non-zero
                curr_x += dx;
                curr_y += dy;
                i += 1;
                steps += 1; 
                map.entry((curr_x, curr_y)).or_insert(steps);
            }
        }

        // For wire 2
        curr_x = 0;
        curr_y = 0;
        steps = 0; 
        let mut heap = BinaryHeap::new();
        for wire in &self.second_wire {
            let (dx, dy, max) = get_dir(wire);
            let mut i = 0;

            while i < max {
                // One of dx or dy will be non-zero
                curr_x += dx;
                curr_y += dy;
                i += 1;
                steps += 1;
                if let Some(v) = map.remove(&(curr_x, curr_y)) {
                    heap.push(Reverse(v + steps));
                }
            }
        }

        heap.pop().expect("Nothing in the heap.").0 as usize
    }
}

pub enum WireComponent {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl WireComponent {
    pub fn new(s: &str) -> Self {
        let amt = s[1..].parse::<i64>().unwrap();
        match s.chars().next() {
            Some(c) if c == 'U' => WireComponent::Up(amt),
            Some(c) if c == 'D' => WireComponent::Down(amt),
            Some(c) if c == 'L' => WireComponent::Left(amt),
            Some(c) if c == 'R' => WireComponent::Right(amt),
            _ => panic!("Unknown input."),
        }
    }
}


fn get_dir(wire_comp: &WireComponent) -> (i32, i32, i64) {
    let mut dx = 0;
    let mut dy = 0;
    let max;
    match wire_comp {
        WireComponent::Up(n) => {
            max = *n;
            dy = 1;
        }
        WireComponent::Down(n) => {
            max = *n;
            dy = -1;
        }
        WireComponent::Left(n) => {
            max = *n;
            dx = -1;
        }
        WireComponent::Right(n) => {
            max = *n;
            dx = 1;
        }
    }

    (dx, dy, max)
}