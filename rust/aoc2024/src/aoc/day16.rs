use common::problem::day::{AoCProblem, Solution};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub struct Day16 {
    maze: Vec<Vec<char>>,
}

impl AoCProblem for Day16 {
    fn prepare(input: String) -> Self {
        Self {
            maze: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut heap: BinaryHeap<PointCost> = BinaryHeap::new();
        heap.push(PointCost::new(self.maze.len() - 2, 1, Direction::East));
        let mut explored: HashSet<(usize, usize, (isize, isize))> = HashSet::new();
        while let Some(p) = heap.pop() {
            if self.maze[p.i][p.j] == '#' {
                continue;
            }

            if self.maze[p.i][p.j] == 'E' {
                return p.cost.into();
            }

            if explored.contains(&(p.i, p.j, p.dir.directional_vector())) {
                continue;
            }

            explored.insert((p.i, p.j, p.dir.directional_vector()));
            heap.push(p.walk());
            heap.push(p.turn_clockwise().walk());
            heap.push(p.turn_counterclockwise().walk());
        }

        unreachable!("lol")
    }

    fn part2(&mut self) -> Solution {
        #[derive(PartialEq, Eq)]
        struct PointCostHistory {
            pt: PointCost,
            history: HashSet<(usize, usize)>,
        }

        impl Ord for PointCostHistory {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.pt.cmp(&other.pt)
            }
        }

        impl PartialOrd for PointCostHistory {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut all_paths = vec![];

        let mut min_cost = usize::MAX;
        let mut node_cost_map: HashMap<(usize, usize, (isize, isize)), usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        heap.push(PointCostHistory {
            pt: PointCost::new(self.maze.len() - 2, 1, Direction::East),
            history: HashSet::new(),
        });

        while let Some(mut p) = heap.pop() {
            if self.maze[p.pt.i][p.pt.j] == '#' {
                continue;
            }

            if p.pt.cost > min_cost {
                continue;
            }

            p.history.insert((p.pt.i, p.pt.j));

            if self.maze[p.pt.i][p.pt.j] == 'E' {
                // If we reach the end, check if the cost to reach the end is better than
                // what we had previously.
                //
                // If so, then reset any existing paths and save the new path and cost.
                // If they're equal, then we can save the existing path.
                match p.pt.cost.cmp(&min_cost) {
                    Ordering::Equal => {
                        all_paths.push(p.history);
                    }
                    Ordering::Less => {
                        min_cost = p.pt.cost;
                        all_paths = vec![p.history];
                    }
                    _ => {}
                }

                continue;
            }

            // Check to see if, at this node, we've already seen a better costing node.
            // If so, we can skip this iteration. If not, then we can save it and move on.
            let cost_at_node = node_cost_map
                .entry((p.pt.i, p.pt.j, p.pt.dir.directional_vector()))
                .or_insert(usize::MAX);
            if p.pt.cost > *cost_at_node {
                continue;
            }

            *cost_at_node = p.pt.cost;

            // Consider all possible neighbors
            for neighbor in [
                p.pt.walk(),
                p.pt.turn_clockwise().walk(),
                p.pt.turn_counterclockwise().walk(),
            ] {
                if p.history.contains(&(neighbor.i, neighbor.j)) {
                    continue;
                }

                heap.push(PointCostHistory {
                    pt: neighbor,
                    history: p.history.clone(),
                });
            }
        }

        let mut all_pts = HashSet::new();
        for path in all_paths {
            all_pts.extend(path);
        }

        all_pts.len().into()
    }

    fn day() -> u32 {
        16
    }

    fn year() -> u32 {
        2024
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PointCost {
    i: usize,
    j: usize,
    dir: Direction,
    cost: usize,
}

impl PointCost {
    pub fn new(i: usize, j: usize, dir: Direction) -> Self {
        Self { i, j, dir, cost: 0 }
    }

    pub fn walk(&self) -> Self {
        let (di, dj) = self.dir.directional_vector();
        Self {
            i: (self.i as isize + di) as usize,
            j: (self.j as isize + dj) as usize,
            dir: self.dir,
            cost: self.cost + 1,
        }
    }

    pub fn turn_clockwise(&self) -> Self {
        Self {
            i: self.i,
            j: self.j,
            dir: self.dir.clockwise(),
            cost: self.cost + 1000,
        }
    }

    pub fn turn_counterclockwise(&self) -> Self {
        Self {
            i: self.i,
            j: self.j,
            dir: self.dir.counterclockwise(),
            cost: self.cost + 1000,
        }
    }
}

impl Ord for PointCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PointCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    West,
    South,
    North,
    East,
}

impl Direction {
    pub fn directional_vector(&self) -> (isize, isize) {
        match self {
            Direction::West => (0, -1),
            Direction::South => (1, 0),
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
        }
    }

    pub fn counterclockwise(&self) -> Self {
        match self {
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::North => Direction::West,
            Direction::East => Direction::North,
        }
    }

    pub fn clockwise(&self) -> Self {
        match self {
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::North => Direction::East,
            Direction::East => Direction::South,
        }
    }
}
