use std::collections::{VecDeque};

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (usize, usize) {
    let header: Vec<usize> = input[0]
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut header_deque: VecDeque<usize> = VecDeque::from(header);

    // Construct the "tree"
    let tree = Tree::new(&mut header_deque);

    return (part1(&tree), part2(&tree));
}

// https://adventofcode.com/2018/day/8
//
// --- Day 8: Memory Maneuver ---
// The sleigh is much easier to pull than you'd expect for something its weight. Unfortunately,
// neither you nor the Elves know which way the North Pole is from here.
//
// You check your wrist device for anything that might help. It seems to have some kind of
// navigation system! Activating the navigation system produces more bad news: "Failed to start
// navigation system. Could not read software license file."
//
// The navigation system's license file consists of a list of numbers (your puzzle input). The
// numbers define a data structure which, when processed, produces some kind of tree that can be
// used to calculate the license number.
//
// The tree is made up of nodes; a single, outermost node forms the tree's root, and it contains
// all other nodes in the tree (or contains nodes that contain nodes, and so on).
//
// Specifically, a node consists of:
//
// A header, which is always exactly two numbers:
// The quantity of child nodes.
// The quantity of metadata entries.
// Zero or more child nodes (as specified in the header).
// One or more metadata entries (as specified in the header).
// Each child node is itself a node that has its own header, child nodes, and metadata. For
// example:
//
//  2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
//  A----------------------------------
//      B----------- C-----------
//                       D-----
// In this example, each node of the tree is also marked with an underline starting with a letter
// for easier identification. In it, there are four nodes:
// - A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
// - B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
// - C, which has 1 child node (D) and 1 metadata entry (2).
// - D, which has 0 child nodes and 1 metadata entry (99).
// The first check done on the license file is to simply add up all of the metadata entries. In
// this example, that sum is 1+1+2+10+11+12+2+99=138.
//
// What is the sum of all metadata entries?

pub fn part1(tree: &Tree) -> usize {
    return tree.sum_of_meta_data();
}

// --- Part Two ---
// The second check is slightly more complicated: you need to find the value of the root node (A in
// the example above).
//
// The value of a node depends on whether it has child nodes.
//
// If a node has no child nodes, its value is the sum of its metadata entries. So, the value of
// node B is 10+11+12=33, and the value of node D is 99.
//
// However, if a node does have child nodes, the metadata entries become indexes which refer to
// those child nodes. A metadata entry of 1 refers to the first child node, 2 to the second, 3 to
// the third, and so on. The value of this node is the sum of the values of the child nodes
// referenced by the metadata entries. If a referenced child node does not exist, that reference is
// skipped. A child node can be referenced multiple time and counts each time it is referenced. A
// metadata entry of 0 does not refer to any child node.
//
// For example, again using the above nodes:
//
// - Node C has one metadata entry, 2. Because node C has only one child node, 2 references a child
// node which does not exist, and so the value of node C is 0.
// - Node A has three metadata entries: 1, 1, and 2. The 1 references node A's first child node, B,
// and the 2 references node A's second child node, C. Because node B has a value of 33 and node C
// has a value of 0, the value of node A is 33+33+0=66.
//
// So, in this example, the value of the root node is 66.
//
// What is the value of the root node?

pub fn part2(tree: &Tree) -> usize {
    return tree.sum_second_check();
}

struct Node {
    child_nodes: Vec<Node>,
    metadata_entries: Vec<usize>
}

pub struct Tree {
    root_node: Node
}

impl Tree {
    /// Creates a new Tree from the given input.
    ///
    /// # Parameters
    /// * `input` - The puzzle input.
    ///
    /// # Returns
    /// The new `Tree`.
    pub fn new(input: &mut VecDeque<usize>) -> Self {
        let mut node = Node {child_nodes: vec![], metadata_entries: vec![]};
        Tree::process_input(input, &mut node);
        return Tree {root_node: node};
    }

    /// Populates the tree with the nodes.
    ///
    /// # Parameters
    /// * `input` - The input vector.
    /// * `curr_node` - The current node. Note that this node refers to the *current* node to
    /// deal with. This does *not* mean the previous (parent) node.
    fn process_input(input: &mut VecDeque<usize>, curr_node: &mut Node) {
        // No more input to process.
        if input.is_empty() {
            return;
        }

        let mut num_child_nodes = input.pop_front().expect("[a]");
        let mut num_meta_data = input.pop_front().expect("[b]");

        // If no child nodes, then put metadata in the current node and leave
        if num_child_nodes == 0 {
            while num_meta_data > 0 {
                curr_node.metadata_entries.push(input.pop_front().expect("[c]"));
                num_meta_data -= 1;
            }

            return;
        }

        // Otherwise, create a new node for each child node then attach to the current node.
        while num_child_nodes > 0 {
            let mut new_node = Node {child_nodes: vec![], metadata_entries: vec![]};
            Tree::process_input(input, &mut new_node);
            curr_node.child_nodes.push(new_node);
            num_child_nodes -= 1;
        }

        // Then, add the meta data to the current node.
        while num_meta_data > 0 {
            curr_node.metadata_entries.push(input.pop_front().expect("[d]"));
            num_meta_data -= 1;
        }
    }

    /// Gets the sum of all metadata entries.
    ///
    /// # Returns
    /// The sum of all metadata entries.
    pub fn sum_of_meta_data(&self) -> usize {
        return Tree::p_sum_meta_data(&self.root_node);
    }

    /// Gets the sum of the current node's metadata. This will recursively do the same to the other
    /// child nodes.
    ///
    /// # Returns
    /// The sum of this node's metadata entries + all child nodes.
    fn p_sum_meta_data(node: &Node) -> usize {
        let mut node_sum: usize = 0;
        for n in &node.child_nodes {
            node_sum += Tree::p_sum_meta_data(&n);
        }

        return node.metadata_entries.iter().sum::<usize>() + node_sum;
    }

    /// Gets the sum, as specified by the second part.
    ///
    /// # Returns
    /// The sum.
    pub fn sum_second_check(&self) -> usize {
        let mut sum: usize = 0;
        for md in &self.root_node.metadata_entries {
            sum += Tree::p_sum_second_check(&self.root_node, *md);
        }
        return sum;
    }

    /// Gets the sum, as specified by the second part, of this specific node and its children nodes.
    ///
    /// # Returns
    /// The sum of this node and its children nodes.
    fn p_sum_second_check(node: &Node, metadata: usize) -> usize {
        // Base Case
        if metadata == 0 {
            return 0;
        }

        let real_idx: usize = metadata - 1;
        if real_idx >= node.child_nodes.len() {
            return 0;
        }

        // If the child node is empty, then we can return the sum of the metadata of the child
        // node.
        if node.child_nodes[real_idx].child_nodes.is_empty() {
            return node.child_nodes[real_idx].metadata_entries.iter().sum();
        }

        // Otherwise, sum them up.
        let mut sum: usize = 0;
        for metadata in &node.child_nodes[real_idx].metadata_entries {
            sum += Tree::p_sum_second_check(&node.child_nodes[real_idx], *metadata);
        }

        return sum;
    }
}