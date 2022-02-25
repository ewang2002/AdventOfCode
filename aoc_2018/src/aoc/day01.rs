use std::collections::HashSet;

// https://adventofcode.com/2018/day/1
#[allow(dead_code)]
pub fn execute(input: &[String]) -> (i32, i64) {
    (part1(input), part2(input))
}

pub fn part1(input: &[String]) -> i32 {
    input
        .iter()
        .map(|x| x.parse::<i32>().expect("Parse error."))
        .sum()
}

pub fn part2(input: &[String]) -> i64 {
    let mut freq = 0;
    let num_vec: Vec<i64> = input
        .iter()
        .map(|x| {
            x.parse::<i64>()
                .unwrap_or_else(|_| panic!("A parse error occurred: {}", x))
        })
        .collect();
    let mut set: HashSet<i64> = HashSet::new();
    loop {
        for num in &num_vec {
            freq += num;
            if set.contains(&freq) {
                return freq;
            }
            set.insert(freq);
        }
    }
}
