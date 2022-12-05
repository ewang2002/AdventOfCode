use std::collections::VecDeque;

use crate::aoc::aoc_problem::AoCProblem;

pub struct Day05 {
    stacks: [VecDeque<char>; 9],
    rearrangements: Vec<(usize, usize, usize)>,
}

impl AoCProblem<usize, usize> for Day05 {
    fn prepare(input: &str) -> Self {
        let chunks = input.split("\r\n\r\n").collect::<Vec<_>>();
        // chunks[0] => stack arrangement
        // chunks[1] => rearrangements
        let raw_arrangements = chunks[0]
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // 1 => 1
        // 2 => 5
        // 3 => 9
        // 4x + 1

        let mut stacks = [
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ];

        for line in &raw_arrangements[..raw_arrangements.len() - 1] {
            let mut i = 0;
            while 4 * i + 1 < line.len() {
                if line[4 * i + 1] == ' ' {
                    i += 1;
                    continue;
                }

                stacks[i].push_front(line[4 * i + 1]);
                i += 1;
            }
        }

        let mut rearrangements = vec![];
        for line in chunks[1].lines() {
            let mut num_iterator = line
                .split_whitespace()
                .filter_map(|l| l.parse::<usize>().ok());
            rearrangements.push((
                num_iterator.next().unwrap(),
                num_iterator.next().unwrap(),
                num_iterator.next().unwrap(),
            ));
        }

        Self {
            stacks,
            rearrangements,
        }
    }

    fn part1(&mut self) -> usize {
        let mut arrangements = self.stacks.clone();
        for (mut move_amt, from_stack, to_stack) in &self.rearrangements {
            while move_amt > 0 {
                let elem = arrangements[*from_stack - 1].pop_back().unwrap();
                arrangements[*to_stack - 1].push_back(elem);
                move_amt -= 1;
            }
        }

        let mut s = String::new();
        for stack in arrangements {
            if let Some(c) = stack.back() {
                s.push(*c);
            }
        }

        println!("{}", s);
        0
    }

    fn part2(&mut self) -> usize {
        let mut arrangements = self.stacks.clone();
        let mut temp = vec![];
        for (mut move_amt, from_stack, to_stack) in &self.rearrangements {
            while move_amt > 0 {
                let elem = arrangements[*from_stack - 1].pop_back().unwrap();
                temp.push(elem);
                move_amt -= 1;
            }

            for e in temp.iter().rev() {
                arrangements[*to_stack - 1].push_back(*e);
            }

            temp.clear();
        }

        let mut s = String::new();
        for stack in arrangements {
            if let Some(c) = stack.back() {
                s.push(*c);
            }
        }

        println!("{}", s);
        0
    }
}
