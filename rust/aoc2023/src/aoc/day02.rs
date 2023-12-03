use std::cmp::max;

use common::problem::day::{AoCProblem, Solution};

const BLUE_INDEX: usize = 0;
const RED_INDEX: usize = 1;
const GREEN_INDEX: usize = 2;

pub struct Day02 {
    // game_info - list of games
    // game_info[i] - the game information for the (i + 1)th game
    // game_info[i][j] - the jth subset containing information about what was revealed
    game_info: Vec<Vec<[usize; 3]>>,
}

impl AoCProblem for Day02 {
    fn prepare(input: String) -> Self {
        let mut game_info = vec![];

        // For each game (round)
        for game in input.lines() {
            let mut this_game = vec![];
            // We only care about the subset of cubes
            let (_, data) = game.split_once(": ").unwrap();

            // For each subset of cubes
            for subset in data.split("; ") {
                let mut parsed_subset = [0; 3];

                // For each set of cubes that were revealed
                for cube_info in subset.split(", ") {
                    let (amt, color) = cube_info.split_once(' ').unwrap();
                    let cube_color_idx = match color {
                        "blue" => BLUE_INDEX,
                        "red" => RED_INDEX,
                        "green" => GREEN_INDEX,
                        _ => panic!("unknown color '{color}'"),
                    };

                    let num_cubes = amt.parse::<usize>().unwrap();
                    parsed_subset[cube_color_idx] += num_cubes;
                }

                this_game.push(parsed_subset);
            }

            game_info.push(this_game);
        }

        Self { game_info }
    }

    fn part1(&mut self) -> Solution {
        self.game_info
            .iter()
            .enumerate()
            .filter(|(_, rounds)| {
                rounds.iter().all(|round| {
                    round[RED_INDEX] <= 12 && round[GREEN_INDEX] <= 13 && round[BLUE_INDEX] <= 14
                })
            })
            .map(|(idx, _)| idx + 1)
            .sum::<usize>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        self.game_info
            .iter()
            .map(|rounds| {
                rounds
                    .iter()
                    .fold([0, 0, 0], |acc, curr| {
                        [
                            max(acc[0], curr[0]),
                            max(acc[1], curr[1]),
                            max(acc[2], curr[2]),
                        ]
                    })
                    .into_iter()
                    .product::<usize>()
            })
            .sum::<usize>()
            .into()
    }

    fn day() -> u32 {
        2
    }

    fn year() -> u32 {
        2023
    }
}
