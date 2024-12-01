use common::problem::day::{AoCProblem, Solution};
use std::collections::HashMap;

pub struct Day01 {
    left_list: Vec<isize>,
    right_list: Vec<isize>,
}

impl AoCProblem for Day01 {
    fn prepare(input: String) -> Self {
        let values = input
            .lines()
            .map(|l| l.split_once("   ").unwrap())
            .map(|(l, r)| (l.parse::<_>().unwrap(), r.parse::<_>().unwrap()))
            .collect::<Vec<(_, _)>>();
        Self {
            left_list: values.iter().map(|(l, _)| *l).collect(),
            right_list: values.iter().map(|(_, r)| *r).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut left = self.left_list.clone();
        let mut right = self.right_list.clone();
        left.sort_unstable();
        right.sort_unstable();

        let mut ttl_distance = 0;
        for (&l, &r) in left.iter().zip(right.iter()) {
            ttl_distance += (r - l).abs();
        }

        ttl_distance.into()
    }

    fn part2(&mut self) -> Solution {
        let mut occurrence_count = HashMap::new();
        for elem in self.right_list.iter() {
            *occurrence_count.entry(*elem).or_insert(0) += 1;
        }

        let mut similarity_score = 0;
        for elem in self.left_list.iter_mut() {
            similarity_score += *elem * occurrence_count.get(elem).unwrap_or(&0);
        }

        similarity_score.into()
    }

    fn day() -> u32 {
        1
    }

    fn year() -> u32 {
        2024
    }
}
