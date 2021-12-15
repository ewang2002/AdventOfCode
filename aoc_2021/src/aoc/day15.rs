use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::aoc::aoc_problem::AoCProblem;

type Point = (usize, usize);
const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub struct Day15 {
    risk_levels: Vec<Vec<usize>>
}

impl AoCProblem<usize, usize> for Day15 {
    fn prepare(input: Vec<String>) -> Self {
        let risk_levels: Vec<Vec<_>> = input.iter()
            .map(|i| i.bytes().map(|v| (v - b'0') as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { risk_levels }
    }

    fn part1(&self) -> usize {
        find_shortest_path_score(&self.risk_levels)
    }

    fn part2(&self) -> usize {
        0
    }
}

/// Finds the shortest path score from the top-left point of the "maze" to the bottom-right point
/// of the "maze" using Dijkstra's Algorithm (thanks CSE 100).
///
/// # Parameters
/// - `input`: The input risk level "maze."
///
/// # Returns
/// The shortest path score.
fn find_shortest_path_score(input: &[Vec<usize>]) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(PointValue::new(0, 0, 0));

    let end_point = (input.len() - 1, input[0].len() - 1);
    let mut explored = HashMap::new();
    while heap.len() > 0 {
        let node = heap.pop().unwrap();
        let point = node.point;
        if explored.contains_key(&point) {
            continue;
        }

        explored.entry(point).or_insert(0);
        let (x, y) = point;
        explored.insert(point, node.weight);
        for (dx, dy) in NEIGHBORS {
            let new_x = (x as i32) + dx;
            let new_y = (y as i32) + dy;
            if new_x < 0 || new_y < 0 || new_x > end_point.0 as i32 || new_y > end_point.1 as i32 {
                continue;
            }

            let new_val = node.weight + input[new_x as usize][new_y as usize];
            heap.push(PointValue::new(new_x as usize, new_y as usize, new_val));
        }
    }

    explored[&end_point]
}



#[derive(Copy, Clone)]
struct PointValue {
    weight: usize,
    point: (usize, usize),
}

impl Eq for PointValue {}

impl PartialEq<Self> for PointValue {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd<Self> for PointValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.weight < other.weight {
            Some(Ordering::Greater)
        } else if self.weight > other.weight {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for PointValue {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weight < other.weight {
            Ordering::Greater
        } else if self.weight > other.weight {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PointValue {
    pub fn new(x: usize, y: usize, weight: usize) -> Self {
        Self { point: (x, y), weight}
    }
}