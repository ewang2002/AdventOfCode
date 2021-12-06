use std::collections::{VecDeque};

// https://adventofcode.com/2018/day/8
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

pub fn part1(tree: &Tree) -> usize {
    return tree.sum_of_meta_data();
}

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