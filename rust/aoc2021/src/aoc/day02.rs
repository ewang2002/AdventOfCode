use common::problem::day::{AoCProblem, Solution};

enum Direction {
    Up,
    Down,
    Forward,
}

pub struct Day02 {
    submarine_direction: Vec<(Direction, i32)>,
}

// https://adventofcode.com/2021/day/2
impl AoCProblem for Day02 {
    fn prepare(input: String) -> Self {
        Self {
            submarine_direction: input
                .lines()
                .map(|x| {
                    let dir_num = x.split(' ').collect::<Vec<_>>();
                    let dir = match dir_num[0] {
                        "forward" => Direction::Forward,
                        "down" => Direction::Down,
                        "up" => Direction::Up,
                        _ => panic!("invalid direction {}", dir_num[0]),
                    };

                    (dir, dir_num[1].parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut depth = 0;
        let mut horiz_pos = 0;

        self.submarine_direction.iter().for_each(|(dir, amt)| {
            match dir {
                Direction::Forward => horiz_pos += amt,
                Direction::Down => depth += amt,
                Direction::Up => depth -= amt,
            };
        });

        (depth * horiz_pos).into()
    }

    fn part2(&mut self) -> Solution {
        let mut depth = 0;
        let mut horiz_pos = 0;
        let mut aim = 0;

        self.submarine_direction.iter().for_each(|(dir, amt)| {
            match dir {
                Direction::Forward => {
                    horiz_pos += amt;
                    depth += aim * amt;
                }
                Direction::Down => aim += amt,
                Direction::Up => aim -= amt,
            };
        });

        (depth * horiz_pos).into()
    }

    fn day() -> u32 {
        2
    }
}
