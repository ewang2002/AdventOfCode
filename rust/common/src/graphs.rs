use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Performs a topological sort on the given directed acyclic graph.
///
/// # Parameters
/// - `graph`: The graph. This must be directed, and must not be cyclic.
///
/// # Returns
/// The topological ordering of the nodes.
pub fn topological_sort<E>(graph: &HashMap<E, Vec<E>>) -> Vec<&E>
where
    E: Hash + Eq,
{
    let mut stack = VecDeque::new();
    for node in graph.keys() {
        if stack.contains(&node) {
            continue;
        }

        explore(graph, &mut stack, node);
    }

    let mut ordering = vec![];
    while !stack.is_empty() {
        ordering.push(stack.pop_front().unwrap());
    }

    ordering
}

fn explore<'a, E>(graph: &'a HashMap<E, Vec<E>>, stack: &mut VecDeque<&'a E>, node: &'a E)
where
    E: Hash + Eq,
{
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if stack.contains(&neighbor) {
                continue;
            }

            explore(graph, stack, neighbor);
        }
    }

    stack.push_back(node);
}
