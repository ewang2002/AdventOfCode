use std::collections::{HashMap};
use crate::aoc::aoc_problem::AoCProblem;

pub struct Day12 {
    all_nodes: HashMap<String, Node>,
}

// https://adventofcode.com/2021/day/12
impl<'a> AoCProblem<usize, usize> for Day12 {
    fn prepare(input: Vec<String>) -> Self {
        let all_mappings: Vec<(String, String)> = input.iter().map(|x| {
            let start_end = x.split("-").collect::<Vec<_>>();
            (start_end[0].to_string(), start_end[1].to_string())
        }).collect();

        let mut all_nodes: HashMap<String, Node> = HashMap::new();
        for (from, to) in &all_mappings {
            all_nodes.entry(String::from(from)).or_insert(Node::new(from));
            all_nodes.entry(String::from(to)).or_insert(Node::new(to));

            let f = all_nodes.get_mut(&*from).unwrap();
            f.add_neighbor(&to);

            let t = all_nodes.get_mut(&*to).unwrap();
            t.add_neighbor(&from);
        }

        return Day12 { all_nodes };
    }

    fn part1(&self) -> usize {
        let initial_explored = HashMap::from([
            ("start", 1)
        ]);
        let start_node = self.all_nodes.get("start").unwrap();
        return number_of_paths(&self.all_nodes, start_node, initial_explored, &|node, explored| {
            node.is_small_cave() && explored.get(node.name.as_str()).is_some()
        });
    }

    fn part2(&self) -> usize {
        let initial_explored = HashMap::from([
            ("start", 1)
        ]);
        let start_node = self.all_nodes.get("start").unwrap();
        return number_of_paths(&self.all_nodes, start_node, initial_explored, &|node, explored| {
            if !node.is_small_cave() {
                return false;
            }

            match explored.get(&*node.name) {
                Some(n) => {
                    if n + 1 > 2 {
                        return true;
                    }
                }
                None => {}
            };

            explored.iter()
                .filter(|x| (*x).0 != &"start" && (*x).0.chars().all(|y| y.is_lowercase()))
                .filter(|x| x.1 >= &2).count() > 1
        });
    }
}

pub struct Node {
    pub name: String,
    neighbors: Vec<String>,
}

impl Node {
    /// Creates a new `Node` instance with the specified `name`.
    ///
    /// # Parameters
    /// - `name`: The name of this node.
    pub fn new(name: &String) -> Self {
        Self {
            name: name.clone(),
            neighbors: Vec::new(),
        }
    }

    /// Determines whether the cave represented by this node is a small cave.
    ///
    /// # Returns
    /// `true` if this is a small cave and `false` otherwise.
    pub fn is_small_cave(&self) -> bool {
        return self.name.to_lowercase() == self.name;
    }

    /// Adds a neighbor to this node.
    ///
    /// # Parameters
    /// - `node`: The other node.
    pub fn add_neighbor(&mut self, node: &String) -> () {
        self.neighbors.push(node.clone());
    }
}

/// Calculates the number of paths possible.
///
/// # Parameters
/// - `all_nodes`: All possible nodes.
/// - `curr_node`: The current nodes.
/// - `explored`: All explored nodes, along with the number of times that node has been explored.
/// - `checker`: The function that determines whether this is a valid path.
///
/// # Returns
/// The number of paths possible.
fn number_of_paths<F>(all_nodes: &HashMap<String, Node>, curr_node: &Node,
                      explored: HashMap<&str, usize>, checker: &F) -> usize
    where F: Fn(&Node, &HashMap<&str, usize>) -> bool {
    if curr_node.name == "end" {
        return 1;
    }

    let mut num_paths = 0;
    for neighbor in &curr_node.neighbors {
        if neighbor == "start" {
            continue;
        }

        let this_neighbor_node = all_nodes.get(&*neighbor).unwrap();
        if checker(this_neighbor_node, &explored) {
            continue;
        }

        let mut cloned_explored = explored.clone();
        *cloned_explored.entry(neighbor).or_insert(0) += 1;

        num_paths += number_of_paths(
            all_nodes,
            this_neighbor_node,
            cloned_explored,
            checker,
        );
    }

    return num_paths;
}