use std::collections::HashMap;

use common::{
    constants::TWO_NEWLINE,
    numbers::lcm,
    problem::day::{AoCProblem, Solution},
};

pub struct Day08 {
    graph: HashMap<String, Vec<String>>,
    instructions: Vec<char>,
}

impl AoCProblem for Day08 {
    fn prepare(input: String) -> Self {
        let (instr, raw_graph) = input.split_once(TWO_NEWLINE).unwrap();
        let mut graph = HashMap::new();
        for line in raw_graph.lines() {
            let data = line
                .replace(['(', ')'], "")
                .replace(" = ", " ")
                .replace(", ", " ");
            let mut iterator = data.split(' ');

            let node = iterator.next().unwrap();
            let left = iterator.next().unwrap();
            let right = iterator.next().unwrap();
            let children_nodes = graph.entry(String::from(node)).or_insert(vec![]);
            children_nodes.push(String::from(left));
            children_nodes.push(String::from(right));
        }

        Self {
            instructions: instr.chars().collect(),
            graph,
        }
    }

    fn part1(&mut self) -> Solution {
        let mut instruction_idx = 0;
        let mut curr_elem = "AAA";
        let mut num_steps = 0;
        loop {
            if curr_elem == "ZZZ" {
                break;
            }

            let curr_node = self.graph.get(curr_elem).unwrap();
            let curr_instr = self.instructions[instruction_idx];
            let next_step = match curr_instr {
                'L' => curr_node[0].as_str(),
                'R' => curr_node[1].as_str(),
                _ => unreachable!(),
            };

            curr_elem = next_step;

            instruction_idx += 1;
            instruction_idx %= self.instructions.len();
            num_steps += 1;
        }

        num_steps.into()
    }

    fn part2(&mut self) -> Solution {
        // Using LCM to compute the answer.
        //
        // From the example in part 2, it looks like when you start at 11A and 22A,
        // - 11A -> 11B -> 11Z implies 2 iterations
        // - 22A -> 22B -> 22C -> 22Z implies 3 iterations
        //
        // Further, notice that in this example, we effectively have cycles going on, with the 11A
        // node giving us a cycle that repeats every 2 times (11A -> 11B -> 11Z -> 11B -> 11Z -> ...)
        // and the 22A node giving us a cycle that repeats every 3 times. These cycles converge at the
        // same time at some point, which is represented by the LCM. Indeed, LCM(2, 3) = 6.
        //
        // This seems to generalize for bigger inputs, although I'm not sure if this is intended or
        // coincidental. In particular, is it guaranteed that we'll even have cycles?

        let mut all_curr_nodes: Vec<_> = self
            .graph
            .keys()
            .filter(|node| node.ends_with('A'))
            .collect();
        let mut num_steps = vec![0_usize; all_curr_nodes.len()];
        let mut instruction_idx = 0;

        loop {
            if all_curr_nodes.iter().all(|node| node.ends_with('Z')) {
                break;
            }

            let curr_instr = self.instructions[instruction_idx];
            for (idx, node) in all_curr_nodes
                .iter_mut()
                .enumerate()
                .filter(|(_, n)| !n.ends_with('Z'))
            {
                let curr_node = self.graph.get(node.as_str()).unwrap();
                let next_step = match curr_instr {
                    'L' => &curr_node[0],
                    'R' => &curr_node[1],
                    _ => unreachable!(),
                };

                *node = next_step;
                num_steps[idx] += 1;
            }

            instruction_idx += 1;
            instruction_idx %= self.instructions.len();
        }

        num_steps.into_iter().reduce(lcm).unwrap().into()
    }

    fn day() -> u32 {
        8
    }

    fn year() -> u32 {
        2023
    }
}
