use common::day::{AoCProblem, Solution};
use std::collections::HashMap;

pub struct Day12 {
    all_nodes: HashMap<String, Node>,
    start_idx: usize,
    all_small_cave_idx: Vec<usize>,
}

// https://adventofcode.com/2021/day/12
impl AoCProblem for Day12 {
    fn prepare(input: String) -> Self {
        let all_mappings: Vec<(String, String)> = input
            .lines()
            .map(|x| {
                let start_end = x.split('-').collect::<Vec<_>>();
                (start_end[0].to_string(), start_end[1].to_string())
            })
            .collect();

        let mut all_nodes: HashMap<String, Node> = HashMap::new();
        for (from, to) in &all_mappings {
            all_nodes
                .entry(String::from(from))
                .or_insert_with(|| Node::new(from));
            all_nodes
                .entry(String::from(to))
                .or_insert_with(|| Node::new(to));

            let f = all_nodes.get_mut(from).unwrap();
            f.add_neighbor(to);

            let t = all_nodes.get_mut(to).unwrap();
            t.add_neighbor(from);
        }

        let mut all_lowercase_idx: Vec<usize> = vec![];
        for (id, node) in all_nodes.values_mut().enumerate() {
            node.id = id;
            if node.is_small_cave() {
                all_lowercase_idx.push(node.id);
            }
        }

        return Day12 {
            start_idx: all_nodes.iter().position(|x| x.0 == "start").unwrap(),
            all_nodes,
            all_small_cave_idx: all_lowercase_idx,
        };
    }

    fn part1(&mut self) -> Solution {
        let mut initial_explored: Vec<usize> = vec![0; self.all_nodes.len()];
        initial_explored[self.start_idx] += 1;
        let start_node = self.all_nodes.get("start").unwrap();

        number_of_paths(
            &self.all_nodes,
            start_node,
            initial_explored,
            &|node, explored| node.is_small_cave() && explored[node.id] > 0,
        )
        .into()
    }

    fn part2(&mut self) -> Solution {
        let mut initial_explored: Vec<usize> = vec![0; self.all_nodes.len()];
        initial_explored[self.start_idx] += 1;
        let start_node = self.all_nodes.get("start").unwrap();

        number_of_paths(
            &self.all_nodes,
            start_node,
            initial_explored,
            &|node, explored| {
                if !node.is_small_cave() {
                    return false;
                }

                if explored[node.id] + 1 > 2 {
                    return true;
                }

                self.all_small_cave_idx
                    .iter()
                    .filter(|x| explored[**x] >= 2)
                    .count()
                    > 1
            },
        )
        .into()
    }
}

pub struct Node {
    name: String,
    id: usize,
    neighbors: Vec<String>,
}

impl Node {
    /// Creates a new `Node` instance with the specified `name`.
    ///
    /// # Parameters
    /// - `name`: The name of this node.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            neighbors: Vec::new(),
            id: usize::MAX,
        }
    }

    /// Determines whether the cave represented by this node is a small cave.
    ///
    /// # Returns
    /// `true` if this is a small cave and `false` otherwise.
    pub fn is_small_cave(&self) -> bool {
        self.name.to_lowercase() == self.name
    }

    /// Adds a neighbor to this node.
    ///
    /// # Parameters
    /// - `node`: The other node.
    pub fn add_neighbor(&mut self, node: &str) {
        self.neighbors.push(node.to_string());
    }
}

/// Calculates the number of paths possible.
///
/// # Parameters
/// - `all_nodes`: All possible nodes.
/// - `curr_node`: The current nodes.
/// - `explored`: The number of times each node (indexed by ID) has been explored.
/// - `checker`: The function that determines whether this is a valid path.
///
/// # Returns
/// The number of paths possible.
fn number_of_paths<F>(
    all_nodes: &HashMap<String, Node>,
    curr_node: &Node,
    explored: Vec<usize>,
    checker: &F,
) -> usize
where
    F: Fn(&Node, &Vec<usize>) -> bool,
{
    if curr_node.name == "end" {
        return 1;
    }

    let mut num_paths = 0;
    for neighbor in &curr_node.neighbors {
        if neighbor == "start" {
            continue;
        }

        let this_neighbor_node = all_nodes.get(neighbor).unwrap();
        if checker(this_neighbor_node, &explored) {
            continue;
        }

        let mut cloned_explored = explored.clone();
        cloned_explored[this_neighbor_node.id] += 1;
        num_paths += number_of_paths(all_nodes, this_neighbor_node, cloned_explored, checker);
    }

    num_paths
}
