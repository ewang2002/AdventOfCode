use common::problem::day::{AoCProblem, Solution};

pub struct Day25 {
    sea_map: Vec<Vec<char>>,
}

// https://adventofcode.com/2021/day/25
impl AoCProblem for Day25 {
    fn prepare(input: String) -> Self {
        Self {
            sea_map: input.lines().map(|x| x.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut num_steps = 0;
        let mut curr_map = self.sea_map.clone();
        let max_i = curr_map.len();
        let max_j = curr_map[0].len();

        loop {
            num_steps += 1;

            let mut num_movements = 0;

            for b in [true, false] {
                let orig = curr_map.clone();
                for i in 0..max_i {
                    for j in 0..max_j {
                        match orig[i][j] {
                            'v' if !b => {
                                // Going down
                                if orig[(i + 1) % max_i][j] == '.' {
                                    curr_map[i][j] = '.';
                                    curr_map[(i + 1) % max_i][j] = 'v';
                                    num_movements += 1;
                                }
                            }
                            '>' if b => {
                                if orig[i][(j + 1) % max_j] == '.' {
                                    curr_map[i][j] = '.';
                                    curr_map[i][(j + 1) % max_j] = '>';
                                    num_movements += 1;
                                }
                            }
                            _ => continue,
                        };
                    }
                }
            }

            if num_movements == 0 {
                break;
            }
        }

        num_steps.into()
    }

    fn part2(&mut self) -> Solution {
        // Free star that I can't get. :(
        0.into()
    }

    fn day() -> u32 {
        25
    }
}
