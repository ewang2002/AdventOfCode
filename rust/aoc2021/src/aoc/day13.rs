use common::problem::day::{AoCProblem, Solution};
use std::collections::HashSet;

pub struct Day13 {
    coordinates: HashSet<(i32, i32)>,
    fold_directions: Vec<(char, i32)>,
}

// https://adventofcode.com/2021/day/13
impl AoCProblem for Day13 {
    fn prepare(input: String) -> Self {
        let coordinates = input
            .lines()
            .filter(|x| x.contains(','))
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .collect::<HashSet<(i32, i32)>>();

        let fold_directions = input
            .lines()
            .filter(|x| x.contains('='))
            .map(|line| {
                let (along, amt) = line.split_once('=').unwrap();
                (
                    if along.ends_with('x') { 'x' } else { 'y' },
                    amt.parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        Day13 {
            coordinates,
            fold_directions,
        }
    }

    fn part1(&mut self) -> Solution {
        run_fold_direction(&self.coordinates, self.fold_directions[0])
            .len()
            .into()
    }

    fn part2(&mut self) -> Solution {
        let mut points = self.coordinates.clone();
        for dir in &self.fold_directions {
            points = run_fold_direction(&points, *dir);
        }

        let min_x = points.iter().min_by_key(|p| p.0).expect("invalid min x").0;
        let max_x = points.iter().max_by_key(|p| p.0).expect("invalid max x").0;
        let min_y = points.iter().min_by_key(|p| p.1).expect("invalid min y").1;
        let max_y = points.iter().max_by_key(|p| p.1).expect("invalid max y").1;

        // Create output string with the message
        let mut message = String::new();
        message.push('\n');
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                message.push(if points.contains(&(x, y)) { '#' } else { ' ' });
            }
            message.push('\n');
        }

        message.into()
    }

    fn day() -> u32 {
        13
    }
}

/// Runs one fold direction, applying the directions to all valid points and returning a new set
/// containing the new points.
///
/// # Parameters
/// - `curr`: The current points.
/// -` fold_dir`: The fold direction to apply.
///
/// # Returns
/// The new points.
fn run_fold_direction(curr: &HashSet<(i32, i32)>, fold_dir: (char, i32)) -> HashSet<(i32, i32)> {
    let mut new_points: HashSet<(i32, i32)> = HashSet::new();
    let (dir, at) = fold_dir;
    match dir {
        'x' => {
            for (x, y) in curr {
                if *x <= at {
                    new_points.insert((*x, *y));
                    continue;
                }
                new_points.insert((x - (2 * (x - at)), *y));
            }
        }
        'y' => {
            for (x, y) in curr {
                if *y <= at {
                    new_points.insert((*x, *y));
                    continue;
                }
                new_points.insert((*x, y - (2 * (y - at))));
            }
        }
        _ => panic!("Unknown direction: {}", dir),
    };

    new_points
}
