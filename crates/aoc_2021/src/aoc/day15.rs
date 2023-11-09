use common::day::{AoCProblem, Solution};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Point = (usize, usize);

const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub struct Day15 {
    risk_levels: Vec<Vec<usize>>,
}

// https://adventofcode.com/2021/day/15
impl AoCProblem for Day15 {
    fn prepare(input: String) -> Self {
        let risk_levels: Vec<Vec<_>> = input
            .lines()
            .map(|i| i.bytes().map(|v| (v - b'0') as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { risk_levels }
    }

    fn part1(&mut self) -> Solution {
        find_shortest_path_score(
            &self.risk_levels,
            |levels, x, y| levels[x][y],
            (self.risk_levels.len() - 1, self.risk_levels[0].len() - 1),
        )
        .into()
    }

    fn part2(&mut self) -> Solution {
        find_shortest_path_score(
            &self.risk_levels,
            |levels, x, y| {
                let dx = x / levels.len();
                let dy = y / levels.len();
                let orig = levels[x % levels.len()][y % levels[0].len()] + dx + dy;
                if orig < 10 {
                    orig
                } else {
                    (orig % 10) + 1
                }
            },
            (
                self.risk_levels.len() * 5 - 1,
                self.risk_levels[0].len() * 5 - 1,
            ),
        )
        .into()
    }
}

/// Finds the shortest path score from the top-left point of the risk level "maze" to the specified
/// `end_point` point of the "maze" using Dijkstra's Algorithm (thanks CSE 100).
///
/// # Parameters
/// - `input`: The input risk level "maze."
/// - `weight_fn`: The function that is used to calculate the weight of a given point.
/// - `end_point`: The target end point; this is where the "maze" ends at.
///
/// # Returns
/// The shortest path score.
fn find_shortest_path_score<F>(input: &[Vec<usize>], weight_fn: F, end_point: Point) -> usize
where
    F: Fn(&[Vec<usize>], usize, usize) -> usize,
{
    // Also see: https://doc.rust-lang.org/std/collections/binary_heap/index.html
    let mut heap = BinaryHeap::new();
    heap.push(PointValue::new(0, 0, 0));

    let mut explored = HashMap::new();
    while !heap.is_empty() {
        let node = heap.pop().unwrap();
        let point = node.point;
        if explored.contains_key(&point) {
            continue;
        }

        explored.entry(point).or_insert(node.weight);
        let (x, y) = point;
        for (dx, dy) in NEIGHBORS {
            let new_x = (x as i32) + dx;
            let new_y = (y as i32) + dy;
            if new_x < 0 || new_y < 0 || new_x > end_point.0 as i32 || new_y > end_point.1 as i32 {
                continue;
            }

            let new_val = node.weight + weight_fn(input, new_x as usize, new_y as usize);
            heap.push(PointValue::new(new_x as usize, new_y as usize, new_val));
        }
    }

    explored[&end_point]
}

struct PointValue {
    weight: usize,
    point: (usize, usize),
}

impl PointValue {
    /// Creates a new `PointValue` instance with the specified coordinates and weight.
    ///
    /// # Parameters
    /// - `x`: The `x`-coordinate.
    /// - `y`: The `y`-coordinate.
    /// - `weight`: The weight.
    pub fn new(x: usize, y: usize, weight: usize) -> Self {
        Self {
            point: (x, y),
            weight,
        }
    }
}

// We need these so we can produce a min-heap.
impl Eq for PointValue {}

impl PartialEq<Self> for PointValue {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd<Self> for PointValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointValue {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the meaning of greater/less than weights
        match self.weight.cmp(&other.weight) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}
