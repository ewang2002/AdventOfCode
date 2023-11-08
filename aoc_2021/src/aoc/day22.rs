use crate::aoc::aoc_problem::{AoCProblem, Solution};
use std::collections::HashSet;

pub struct Day22 {
    reboot_steps: Vec<RebootStep>,
}

// https://adventofcode.com/2021/day/22
impl AoCProblem for Day22 {
    fn prepare(input: String) -> Self {
        let mut reboot_steps: Vec<RebootStep> = vec![];
        for line in input.lines() {
            // on x=10..12,y=10..12,z=10..12
            let (toggle_val, rest) = line.split_once(" ").unwrap();
            let parsed_rest = rest
                .replace("x=", "")
                .replace(",y=", " ")
                .replace(",z=", " ")
                .replace("..", " ");

            let mut iterator = parsed_rest.split(' ').map(|x| x.parse::<i32>().unwrap());

            reboot_steps.push(RebootStep {
                toggle: toggle_val == "on",
                from_x: iterator.next().unwrap(),
                to_x: iterator.next().unwrap(),
                from_y: iterator.next().unwrap(),
                to_y: iterator.next().unwrap(),
                from_z: iterator.next().unwrap(),
                to_z: iterator.next().unwrap(),
            });
        }

        Self { reboot_steps }
    }

    fn part1(&mut self) -> Solution {
        let mut enabled: HashSet<(i32, i32, i32)> = HashSet::new();
        for reboot_step in &self.reboot_steps {
            if reboot_step.from_x < -50
                || reboot_step.to_x > 50
                || reboot_step.from_y < -50
                || reboot_step.to_y > 50
                || reboot_step.from_z < -50
                || reboot_step.to_z > 50
            {
                continue;
            }

            for x in reboot_step.from_x..=reboot_step.to_x {
                for y in reboot_step.from_y..=reboot_step.to_y {
                    for z in reboot_step.from_z..=reboot_step.to_z {
                        if reboot_step.toggle {
                            enabled.insert((x, y, z));
                            continue;
                        }

                        enabled.remove(&(x, y, z));
                    }
                }
            }
        }

        enabled.len().into()
    }

    fn part2(&mut self) -> Solution {
        0.into()
    }
}

#[derive(Copy, Clone)]
struct RebootStep {
    from_x: i32,
    to_x: i32,
    from_y: i32,
    to_y: i32,
    from_z: i32,
    to_z: i32,
    toggle: bool,
}
