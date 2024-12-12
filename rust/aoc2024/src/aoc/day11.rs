use std::collections::HashMap;

use common::problem::day::{AoCProblem, Solution};

pub struct Day11 {
    stones: Vec<usize>,
}

impl AoCProblem for Day11 {
    fn prepare(input: String) -> Self {
        Self {
            stones: input.split(' ').map(|n| n.parse().unwrap()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        blink(&self.stones, 25).into()
    }

    fn part2(&mut self) -> Solution {
        blink(&self.stones, 75).into()
    }

    fn day() -> u32 {
        11
    }

    fn year() -> u32 {
        2024
    }
}

/// Builds a graph that represents the stones. Note that this function does not return
/// a map; rather, it uses an existing graph and builds on top of it.
///
/// In the map, the key is a stone and the value is a list of all stone(s) where each stone
/// is created after a "blink" and according to the rules described in the writeup.
///
///
/// # Parameters
/// - `num`: The number to start building the graph from.
/// - `graph`: A map representing the graph to build on top of.
///
/// # Example
/// Consider the following map:
///
/// ```txt
///     {0 -> [1]}
///     {1 -> [2024]}
///     {2024 -> [20, 24]}
///     {20 -> [2, 0]}
///     {24 -> [2, 4]}
///     {4 -> ...}
///     ...
/// ```
///
/// Here, the stone 0 turns into the stone 1 after a blink. The stone 1 turns into
/// the stone 2024 after a blink. The stone 2024 turns into two stones, 20 and 24.  
fn build_number_graph(num: usize, graph: &mut HashMap<usize, Vec<usize>>) {
    if graph.contains_key(&num) {
        return;
    }

    if num == 0 {
        graph.insert(num, vec![1]);
        build_number_graph(1, graph);
    } else {
        let digits = get_digits(num);

        if digits.len() % 2 == 0 {
            let left = combine_digits(&digits[..digits.len() / 2]);
            let right = combine_digits(&digits[digits.len() / 2..]);
            graph.insert(num, vec![left, right]);
            build_number_graph(left, graph);
            build_number_graph(right, graph);
        } else {
            graph.insert(num, vec![num * 2024]);
            build_number_graph(num * 2024, graph);
        }
    }
}

/// "Blinks" the specified number of times, where for each blink the stones are
/// transformed accordingly.
///
/// In this implementation, the idea is that if we start from a stone X, then to get
/// all the stones that will exist after the specified number of blinks B, we can simply do
/// a level-order traversal; the stones at level B will be all the stones that will occur
/// after blinking B times.
///
/// For example, if we have the stone 0, then the graph might look like:
/// ```txt
///     {0 -> [1]}
///     {1 -> [2024]}
///     {2024 -> [20, 24]}
///     {20 -> [2, 0]}
///     {24 -> [2, 4]}
///     {2 -> [4048]}
///     {4 -> [8096]}
///     {4048 -> [40, 48]}
///     {8096 -> [80, 96]}
///     ...
/// ```
/// In tree format, this might look like
/// ```txt
///                     0                           level 0
///                     |
///                     1                           level 1
///                     |
///                   2024                          level 2
///                     |
///               |----------|
///              20          24                     level 3
///               |          |
///       |-------|         |-----|
///       2       0         2     4                 level 4
///       |       |         |     |
///      4048  <cycle>    4048    8096              level 5
///       |                |         |
///    |-----|       |-----|         |-----|
///   40     48     40     48       80     96       level 6
/// ```
/// Note here that `<cycle>` below the `0` means that we will refer back to all the nodes below the
/// top-most `0` node.
///
/// To get all the stones after 4 blinks, we just need to get all the stones at the 4th level. So,
///
/// | Level | Stones                           |
/// | ----- | -------------------------------- |
/// | 0     | `[0]`                            |
/// | 1     | `[1]`                            |
/// | 2     | `[2024]`                         |
/// | 3     | `[20, 24]`                       |
/// | 4     | `[2, 0, 2, 4]`                   |
/// | 5     | `[4048, 1, 4048, 8096]`          |
/// | 6     | `[40, 48, 2024, 40, 48, 80, 96]` |
///
/// # Parameters
/// - `stones`: The list of stones to start with.
/// - `num_times`: The number of times to blink.
///
/// # Returns
/// The number of stones after you've "blinked" the specified number of times.
fn blink(stones: &[usize], num_times: usize) -> usize {
    fn dfs(
        graph: &HashMap<usize, Vec<usize>>,
        stone: usize,
        remaining_blinks: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        match cache.get(&(stone, remaining_blinks)) {
            Some(s) => *s,
            None => {
                if remaining_blinks == 0 {
                    1
                } else {
                    let mut sum = 0;
                    for neighbor in graph.get(&stone).unwrap() {
                        sum += dfs(graph, *neighbor, remaining_blinks - 1, cache);
                    }

                    cache.insert((stone, remaining_blinks), sum);
                    sum
                }
            }
        }
    }

    let mut graph = HashMap::new();
    for stone in stones {
        build_number_graph(*stone, &mut graph);
    }

    let mut sum = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        sum += dfs(&graph, *stone, num_times, &mut cache)
    }

    sum
}

/// Combines a list of digits into a number.
///
/// # Parameters
/// - `nums`: The list of digits.
///
/// # Returns
/// The number with those digits in that order.
fn combine_digits(digits: &[usize]) -> usize {
    let mut combined = 0;
    for num in digits {
        combined = combined * 10 + *num;
    }

    combined
}

/// Gets all digits in a number.
///
/// # Parameters
/// - `num`: The number.
///
/// # Returns
/// The digits in the number.
fn get_digits(mut num: usize) -> Vec<usize> {
    let mut stack = vec![];
    while num > 0 {
        stack.push(num % 10);
        num /= 10;
    }

    let mut digits = vec![];
    while let Some(s) = stack.pop() {
        digits.push(s);
    }

    digits
}
