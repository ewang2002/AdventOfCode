use std::collections::HashMap;

use common::problem::day::{AoCProblem, Solution};

pub struct Day12 {
    condition_records: Vec<(Vec<char>, Vec<usize>)>,
}

impl AoCProblem for Day12 {
    fn prepare(input: String) -> Self {
        Self {
            condition_records: input
                .lines()
                .map(|l| l.split_once(' ').unwrap())
                .map(|(raw_damaged_springs, raw_groupings)| {
                    (
                        raw_damaged_springs.chars().collect(),
                        raw_groupings
                            .split(',')
                            .map(|s| s.parse().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut sum = 0;
        for (arrangements, groupings) in &mut self.condition_records {
            arrangements.push('.');
            sum += get_num_arrangements(arrangements, groupings, 0, 0, 0, &mut HashMap::new());
            arrangements.pop();
        }

        sum.into()
    }

    fn part2(&mut self) -> Solution {
        let mut sum = 0;
        for (arrangements, groupings) in &self.condition_records {
            let mut unfolded_arrangements = arrangements.clone();
            let mut unfolded_groupings = groupings.clone();
            for _ in 0..4 {
                unfolded_arrangements.push('?');
                unfolded_arrangements.extend(arrangements);

                unfolded_groupings.extend(groupings);
            }

            unfolded_arrangements.push('.');
            sum += get_num_arrangements(
                &unfolded_arrangements,
                &unfolded_groupings,
                0,
                0,
                0,
                &mut HashMap::new(),
            );
        }

        sum.into()
    }

    fn day() -> u32 {
        12
    }

    fn year() -> u32 {
        2023
    }
}

/// Gets the number of arrangements that can be created from the initial arrangement template
/// and the groupings.
///
/// # Parameters
/// - `arrangements`: The initial arrangement template (e.g., "#.#.###" as a slice of chars). Note that
///                   this must have a '.' at the end, so if the initial template doesn't have the dot at
///                   the end, you'll need to add it yourself.
/// - `groupings`: The groupings of the damaged springs (e.g., [1, 1, 3]).
/// - `a_idx`: The index of the current arrangement we are considering. This will most likely be 0
///            for the first call.
/// - `g_idx`: The index of the current grouping we are considering. This will most likely be 0
///            for the first call.
/// - `num_broken`: The number of broken springs we have seen so far. This will most likely be 0
///                 for the first call.
/// - `cache`: The cache of previously computed answers. Here, the key is a tuple of `(a_idx, g_idx, num_broken)`
///            and the value is the number of arrangements that can be created from that configuration.
fn get_num_arrangements(
    arrangements: &[char],
    groupings: &[usize],
    a_idx: usize,
    g_idx: usize,
    num_broken: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    // BASE CASE: We've gone through all arrangements and we've found all groupings.
    if a_idx >= arrangements.len() && g_idx >= groupings.len() {
        return 1;
    }

    // BASE CASE: We've gone through all arrangements but we haven't found all groupings.
    if a_idx >= arrangements.len() {
        return 0;
    }

    // BASE CASE: We have more damaged springs than current grouping allows
    if g_idx < groupings.len() && num_broken > groupings[g_idx] {
        return 0;
    }

    // Check the cache to see if this answer has already been computed.
    if let Some(res) = cache.get(&(a_idx, g_idx, num_broken)) {
        return *res;
    }

    let res = match arrangements[a_idx] {
        '#' => get_num_arrangements(
            arrangements,
            groupings,
            a_idx + 1,
            g_idx,
            num_broken + 1,
            cache,
        ),
        '.' => {
            // If we have exactly the right number of broken springs, then we can consider
            // the next grouping
            if g_idx < groupings.len() && num_broken == groupings[g_idx] {
                get_num_arrangements(arrangements, groupings, a_idx + 1, g_idx + 1, 0, cache)
            } else if num_broken == 0 {
                // If we have no broken springs, then we stay on the same grouping
                get_num_arrangements(arrangements, groupings, a_idx + 1, g_idx, 0, cache)
            } else {
                // If we do not have the right number of broken springs (most likely too little), then this
                // configuration is invalid.
                0
            }
        }
        _ => {
            // Consider the case when the ? is a #
            let mut s = get_num_arrangements(
                arrangements,
                groupings,
                a_idx + 1,
                g_idx,
                num_broken + 1,
                cache,
            );

            // Consider the case when the ? is a .
            if g_idx < groupings.len() && num_broken == groupings[g_idx] {
                s += get_num_arrangements(arrangements, groupings, a_idx + 1, g_idx + 1, 0, cache);
            } else if num_broken == 0 {
                s += get_num_arrangements(arrangements, groupings, a_idx + 1, g_idx, 0, cache);
            }

            s
        }
    };

    cache.insert((a_idx, g_idx, num_broken), res);
    res
}
