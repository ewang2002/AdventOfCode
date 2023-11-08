use crate::aoc::aoc_problem::{AoCProblem, Solution};

// Find a sufficiently good bound to check. Brute-force for the win!
const BOUNDING_BOX: i32 = 350;
type Point = (i32, i32);

pub struct Day17 {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

// https://adventofcode.com/2021/day/17
impl AoCProblem for Day17 {
    fn prepare(input: String) -> Self {
        // target area: x=248..285, y=-85..-56
        let bounds: Vec<_> = input
            .lines()
            .nth(0)
            .unwrap()
            .replace("target area: x=", "")
            .replace(", y=", " ")
            .replace("..", " ")
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        assert_eq!(4, bounds.len());
        Self {
            min_x: bounds[0],
            max_x: bounds[1],
            min_y: bounds[2],
            max_y: bounds[3],
        }
    }

    fn part1(&mut self) -> Solution {
        let mut highest: i32 = i32::MIN;
        for dx in -BOUNDING_BOX..=BOUNDING_BOX {
            for dy in -BOUNDING_BOX..=BOUNDING_BOX {
                if let Some(val) =
                    launch_probe(dx, dy, (self.min_x, self.max_x), (self.min_y, self.max_y))
                {
                    if val > highest {
                        highest = val;
                    }
                }
            }
        }

        highest.into()
    }

    fn part2(&mut self) -> Solution {
        let mut valid: usize = 0;
        for dx in -BOUNDING_BOX..=BOUNDING_BOX {
            for dy in -BOUNDING_BOX..=BOUNDING_BOX {
                valid += match launch_probe(
                    dx,
                    dy,
                    (self.min_x, self.max_x),
                    (self.min_y, self.max_y),
                ) {
                    Some(_) => 1,
                    None => 0,
                };
            }
        }

        valid.into()
    }
}

/// Simulates launching a probe at the specified velocities.
///
/// # Parameters
/// - `x_vel`: The velocity in the `x`-direction.
/// - `y_vel`: The velocity in the `y`-direction.
/// - `x_bound`: The trench bound in the `x`-direction.
/// - `y_bound`: The trench bound in the `y`-direction.
///
/// # Returns
/// The highest `y`-value that was reached, if the probe actually made it into the trench. If this
/// isn't possible, `None` is returned.
fn launch_probe(mut x_vel: i32, mut y_vel: i32, x_bound: Point, y_bound: Point) -> Option<i32> {
    let (min_x, max_x) = x_bound;
    let (min_y, max_y) = y_bound;

    let mut highest_y = -1;
    let mut x = 0;
    let mut y = 0;
    while min_y <= y {
        if y_vel == 0 {
            highest_y = y;
        }

        if min_x <= x && x <= max_x && min_y <= y && y <= max_y {
            return Some(highest_y);
        }

        x += x_vel;
        y += y_vel;
        x_vel += if x_vel > 0 {
            -1
        } else if x_vel < 0 {
            1
        } else {
            0
        };
        y_vel -= 1;
    }

    None
}
