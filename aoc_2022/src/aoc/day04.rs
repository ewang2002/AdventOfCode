use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day04 {
    // (left pair min, left pair max, right pair min, right pair max)
    assignments: Vec<(usize, usize, usize, usize)>,
}

impl AoCProblem for Day04 {
    fn prepare(input: String) -> Self {
        // Most sane way of parsing, totally couldn't have made it simpler by
        // using multiple lines.
        Self {
            assignments: input
                .lines()
                .map(|l| l.split_once(',').unwrap())
                .map(|(left, right)| {
                    (
                        left.split_once('-').unwrap(),
                        right.split_once('-').unwrap(),
                    )
                })
                .map(|((l_min, l_max), (r_min, r_max))| {
                    (
                        l_min.parse().unwrap(),
                        l_max.parse().unwrap(),
                        r_min.parse().unwrap(),
                        r_max.parse().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        self.assignments
            .iter()
            .filter(|(l_min, l_max, r_min, r_max)| {
                // Left conditional:
                // l    .234567.
                // r    ...45...
                (l_min <= r_min && r_max <= l_max)
                    // Right conditional:
                    // l    ...45...
                    // r    .234567.
                    || (r_min <= l_min && l_max <= r_max)
            })
            .count()
            .into()
    }

    fn part2(&mut self) -> Solution {
        self.assignments
            .iter()
            .filter(|(l_min, l_max, r_min, r_max)| {
                // Case 1
                // l    .234567.
                // r    ...45...
                (l_min <= r_min && r_max <= l_max)
                    // Case 2
                    // l    ...45...
                    // r    .234567.
                    || (r_min <= l_min && l_max <= r_max)
                    // Case 3
                    // l    .234567.
                    // r    ..345678
                    || (l_min <= r_min && l_max <= r_max && r_min <= l_max)
                    // Case 4
                    // l    ..345678
                    // r    .234567.
                    || (r_min <= l_min && r_max <= l_max && l_min <= r_max)
            })
            .count()
            .into()
    }
}
