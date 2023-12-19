use std::collections::HashSet;

use common::problem::day::{AoCProblem, Solution};

pub struct Day18 {
    dig_plan: Vec<DigPlanStep>,
    colors: Vec<String>,
}

impl AoCProblem for Day18 {
    fn prepare(input: String) -> Self {
        Self {
            dig_plan: input.lines().filter_map(DigPlanStep::from_input).collect(),
            colors: input
                .lines()
                .map(|l| l[l.rfind('(').unwrap() + 2..l.len() - 1].to_string())
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut trenches: HashSet<(isize, isize)> = HashSet::new();
        let mut x = 0;
        let mut y = 0;

        // Dig the trench loop.
        for plan in &self.dig_plan {
            let (dx, dy) = plan.direction.to_coordinate();
            for _ in 0..plan.meters {
                x += dx;
                y += dy;
                trenches.insert((x, y));
            }
        }

        // Find a point that's inside the trench loop
        let mut inner_point = None;
        for (x, y) in &trenches {
            let mut other_pts = trenches.iter().filter(|(x2, y2)| x2 == x && y2 != y);

            let (other_x, other_y) = match other_pts.next() {
                Some(pt) => *pt,
                None => continue,
            };

            // We only want at most one other point with the same x coordinate.
            if other_pts.next().is_some() {
                continue;
            }

            inner_point = Some(((*x + other_x) / 2, (*y + other_y) / 2));
        }

        let inner_point = match inner_point {
            Some(p) => p,
            None => panic!("No inner point found"),
        };

        // With this point, flood fill the trench loop so that everything inside the trench loop is
        // marked.
        let mut stack = vec![inner_point];
        while let Some(p @ (x, y)) = stack.pop() {
            if trenches.contains(&p) {
                continue;
            }

            trenches.insert(p);
            stack.push((x - 1, y));
            stack.push((x + 1, y));
            stack.push((x, y - 1));
            stack.push((x, y + 1));
        }

        trenches.len().into()
    }

    fn part2(&mut self) -> Solution {
        let mut new_dig_plan: Vec<DigPlanStep> = Vec::new();
        for color in &self.colors {
            let distance = &color[..5];
            let direction = &color[5..];

            let distance = match usize::from_str_radix(distance, 16) {
                Ok(d) => d,
                Err(_) => panic!("Invalid distance '{distance}'"),
            };

            let dir = match direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                d => panic!("Invalid direction '{d}'"),
            };

            new_dig_plan.push(DigPlanStep {
                direction: dir,
                meters: distance,
            });
        }

        // Because the input is so large, we'll instead store the corner points of the trench loop
        let mut trench_corner_points: Vec<(isize, isize)> = vec![(0, 0)];

        let mut x = 0;
        let mut y = 0;
        for plan in &new_dig_plan {
            let (dx, dy) = plan.direction.to_coordinate();
            x += dx * plan.meters as isize;
            y += dy * plan.meters as isize;
            trench_corner_points.push((x, y));
        }

        // Use the shoelace formula to calculate the area of the trench loop.
        // See https://artofproblemsolving.com/wiki/index.php/Shoelace_Theorem
        // Let l = sum(x_i * y_{i+1 % n}) and r = sum(y_i * x_{i+1 % n})

        let mut l = 0;
        let mut r = 0;
        for i in 0..trench_corner_points.len() {
            let (x1, y1) = trench_corner_points[i];
            let (x2, y2) = trench_corner_points[(i + 1) % trench_corner_points.len()];
            l += x1 * y2;
            r += y1 * x2;
        }

        let area = (l - r).abs() / 2;

        // Compute the perimeter
        let mut perimeter = 0;
        for pt in trench_corner_points.windows(2) {
            let (x1, y1) = pt[0];
            let (x2, y2) = pt[1];
            perimeter += (x2 - x1).abs() + (y2 - y1).abs();
        }

        // Now, we can use Pick's theorem
        //
        // A = i + b / 2 - 1, where A is the area and b is the number of points on the boundary
        // and i is the number of points inside the polygon.
        //
        // We have A (`area`), we have b (`perimeter`), and we care about i + b (number of points
        // in interior + boundary). Solving for i gives us
        // A = i + (b / 2) - 1
        // -> A + 1 - (b / 2) = i
        //
        // and so
        // i + b = A + 1 - (b / 2) + b
        //       = A + 1 + (b / 2)
        //
        // Better explanation: https://www.reddit.com/r/adventofcode/comments/18lg2we/2023_day_18_why_1_instead_of_1/
        (area + perimeter / 2 + 1).into()
    }

    fn day() -> u32 {
        18
    }

    fn year() -> u32 {
        2023
    }
}

struct DigPlanStep {
    direction: Direction,
    meters: usize,
}

impl DigPlanStep {
    /// Parses a `DigPlanStep` from a string from the input file.
    ///
    /// # Parameters
    /// - `input`: The string to parse.
    ///
    /// # Returns
    /// A `DigPlanStep` if the string is valid, otherwise `None`.
    pub fn from_input(input: &str) -> Option<Self> {
        let input = input.replace(['(', ')'], "");
        let mut s = input.split(' ');
        let direction = match s.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return None,
        };

        let meters = match s.next().unwrap().parse() {
            Ok(m) => m,
            Err(_) => return None,
        };

        Some(Self { direction, meters })
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    /// Converts a `Direction` to a coordinate.
    ///
    /// # Returns
    /// A coordinate representing the direction.
    pub fn to_coordinate(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}
