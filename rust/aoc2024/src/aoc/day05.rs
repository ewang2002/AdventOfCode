use std::collections::HashMap;

use common::{
    constants::TWO_NEWLINE,
    graphs::topological_sort,
    problem::day::{AoCProblem, Solution},
};

pub struct Day05 {
    page_ordering_rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
    bad_updates: Vec<usize>,
}

impl AoCProblem for Day05 {
    fn prepare(input: String) -> Self {
        let mut page_ordering_rules = HashMap::new();
        let (raw_rules, raw_updates) = input.split_once(TWO_NEWLINE).unwrap();
        for line in raw_rules.lines() {
            let (before, after) = line.split_once("|").unwrap();
            let parsed_before = before.parse::<usize>().unwrap();
            let parsed_after = after.parse::<usize>().unwrap();
            page_ordering_rules
                .entry(parsed_before)
                .or_insert(Vec::new())
                .push(parsed_after);
        }

        let mut updates = Vec::new();
        for line in raw_updates.lines() {
            updates.push(
                line.split(',')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect(),
            );
        }

        Self {
            page_ordering_rules,
            updates,
            bad_updates: vec![],
        }
    }

    fn part1(&mut self) -> Solution {
        let mut sum_middle_nums = 0;
        for (uidx, update) in self.updates.iter().enumerate() {
            let mut is_ok = true;
            for (i, page) in update.iter().enumerate() {
                for next_page in update.iter().skip(i + 1) {
                    if !self.page_ordering_rules.contains_key(page)
                        || !self.page_ordering_rules[page].contains(next_page)
                    {
                        is_ok = false;
                        break;
                    }
                }
            }

            if is_ok {
                sum_middle_nums += update[update.len() / 2];
            } else {
                self.bad_updates.push(uidx);
            }
        }

        sum_middle_nums.into()
    }

    fn part2(&mut self) -> Solution {
        let mut sum_middle_nums = 0;
        for bad_idx in self.bad_updates.iter() {
            let mut isolated_ordering = HashMap::new();
            for (k, v) in &self.page_ordering_rules {
                if self.updates[*bad_idx].contains(k) {
                    isolated_ordering.insert(*k, v.clone());
                }
            }

            let ordering = topological_sort(&isolated_ordering);
            let mut new_update = vec![];
            for o in ordering {
                if self.updates[*bad_idx].contains(o) {
                    new_update.push(*o);
                }
            }

            sum_middle_nums += new_update[new_update.len() / 2];
        }

        sum_middle_nums.into()
    }

    fn day() -> u32 {
        5
    }

    fn year() -> u32 {
        2024
    }
}
