use std::collections::{BinaryHeap, HashSet};

use common::problem::day::{AoCProblem, Solution};

pub struct Day17 {
    map: Vec<Vec<usize>>,
}

impl AoCProblem for Day17 {
    fn prepare(input: String) -> Self {
        Self {
            map: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        find_minimum_heat_loss(&self.map, 0, 3).into()
    }

    fn part2(&mut self) -> Solution {
        find_minimum_heat_loss(&self.map, 4, 10).into()
    }

    fn day() -> u32 {
        17
    }

    fn year() -> u32 {
        2023
    }
}

/// Finds the minimum heat loss path from the top left corner to the bottom right corner of the map,
/// where the crucible must move at least `min_steps` blocks in the same direction before changing,
/// and at most `max_steps` blocks in the same direction before changing.
///
/// # Parameters
/// - `map`: The heat map.
/// - `min_steps`: The minimum number of steps the crucible must move in the same direction before changing.
/// - `max_steps`: The maximum number of steps the crucible can move in the same direction before changing.
///
/// # Returns
/// The minimum heat loss path from the top left corner to the bottom right corner of the map.
///
/// # Panics
/// If no path is found.
fn find_minimum_heat_loss(map: &[Vec<usize>], min_steps: usize, max_steps: usize) -> usize {
    #[derive(PartialEq, Eq)]
    struct Node {
        coords: (isize, isize),
        direction: (isize, isize),
        dir_stepped_ct: usize,
        heat_loss: usize,
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.heat_loss.cmp(&self.heat_loss))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.heat_loss.cmp(&self.heat_loss)
        }
    }

    // (x, y, direction, dir_stepped_ct)
    let mut visited: HashSet<(isize, isize, (isize, isize), usize)> = HashSet::new();

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        heap.push(Node {
            coords: (0, 0),
            direction: dir,
            dir_stepped_ct: 0,
            heat_loss: 0,
        });
    }

    while let Some(n) = heap.pop() {
        let (x, y) = n.coords;
        if x == map.len() as isize - 1
            && y == map[0].len() as isize - 1
            && n.dir_stepped_ct >= min_steps
        {
            return n.heat_loss;
        }

        if visited.contains(&(x, y, n.direction, n.dir_stepped_ct)) {
            continue;
        }

        visited.insert((x, y, n.direction, n.dir_stepped_ct));

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (dx, dy) = dir;
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= map.len() as isize || ny >= map[0].len() as isize {
                continue;
            }

            // We must move at least `min_steps` blocks in the same direction before changing direction
            if dir != n.direction && n.dir_stepped_ct < min_steps {
                continue;
            }

            // We can move at most `max_steps` blocks in the same direction
            if dir == n.direction && n.dir_stepped_ct + 1 > max_steps {
                continue;
            }

            // We aren't allowed to back in the opposite direction
            if (dx, dy) == (-n.direction.0, -n.direction.1) {
                continue;
            }

            heap.push(Node {
                coords: (nx, ny),
                direction: dir,
                dir_stepped_ct: if dir == n.direction {
                    n.dir_stepped_ct + 1
                } else {
                    1
                },
                heat_loss: n.heat_loss + map[nx as usize][ny as usize],
            });
        }
    }

    panic!("No path found");
}
