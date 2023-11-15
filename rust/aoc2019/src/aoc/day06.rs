use common::problem::day::{AoCProblem, Solution};
use std::collections::{HashMap, VecDeque};

pub struct Day06 {
    input: Vec<String>,
}

// https://adventofcode.com/2019/day/6
impl AoCProblem for Day06 {
    fn prepare(input: String) -> Self {
        Self {
            input: input.lines().map(|d| d.to_string()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        // We want the graph to be "one-way" (i.e. directed) so we don't double-count
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for map_entry in &self.input {
            let (to, from) = map_entry.split_once(')').unwrap();
            graph
                .entry(to.to_string())
                .or_default()
                .push(from.to_string());
        }

        calculate_paths(&graph, "COM", 0).into()
    }

    fn part2(&mut self) -> Solution {
        // Want the graph to be "both-ways" (i.e. undirected) so we can find a path
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for map_entry in &self.input {
            let (to, from) = map_entry.split_once(')').unwrap();
            graph
                .entry(to.to_string())
                .or_default()
                .push(from.to_string());

            graph
                .entry(from.to_string())
                .or_default()
                .push(to.to_string())
        }

        let mut queue = VecDeque::new();
        let start = graph
            .iter()
            .find(|(_, n)| n.contains(&"YOU".to_string()))
            .expect("Should have 'YOU'")
            .0
            .as_str();
        let end = graph
            .iter()
            .find(|(_, n)| n.contains(&"SAN".to_string()))
            .expect("Should have 'SAN'")
            .0
            .as_str();

        queue.push_back(start);
        let mut explored: HashMap<&str, usize> = HashMap::new();
        explored.insert(start, 0);
        let default = vec![];

        // BFS time!
        while let Some(key) = queue.pop_front() {
            let num = *explored.get(key).unwrap();

            if key == "SAN" {
                break;
            }

            for neighbor in graph.get(key).unwrap_or(&default) {
                if explored.contains_key(neighbor.as_str()) {
                    continue;
                }

                explored.insert(neighbor.as_str(), num + 1);
                queue.push_back(neighbor);
            }
        }

        explored.get(end).unwrap().into()
    }

    fn day() -> u32 {
        6
    }
}

/// Calculates the number of possible paths from node A to node B, including subsets.
///
/// For example, if we have the graph
/// ```
/// A -- x -- y -- B
/// ```
/// then there are 6 paths possible (Ax, Ay, AB, xy, xB, yB).
///
/// # Parameters
/// - `graph`: The graph.
/// - `planet`: The planet to check.
/// - `height`: The number of nodes checked in this "path."
///
/// # Returns
/// The number of paths
fn calculate_paths(graph: &HashMap<String, Vec<String>>, planet: &str, height: usize) -> usize {
    let neighbors = graph.get(planet);
    if let Some(neighbors) = neighbors {
        let mut amt = 0;
        for neighbor in neighbors {
            amt += calculate_paths(graph, neighbor, height + 1) + height + 1;
        }

        amt
    } else {
        0
    }
}
