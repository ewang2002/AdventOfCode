use common::problem::day::{AoCProblem, Solution};
use common::regex::Regex;
use std::collections::HashSet;

const MAX_HEIGHT: isize = 103;
const MAX_WIDTH: isize = 101;

pub struct Day14 {
    robots: Vec<Robot>,
}

impl AoCProblem for Day14 {
    fn prepare(input: String) -> Self {
        let re = Regex::new(r"p=(\d*),(\d*) v=(-?\d*),(-?\d*)").unwrap();

        let mut robots = vec![];
        for line in input.lines() {
            for (_, [x, y, dx, dy]) in re.captures_iter(line).map(|c| c.extract()) {
                robots.push(Robot::new(
                    (x.parse().unwrap(), y.parse().unwrap()),
                    (dx.parse().unwrap(), dy.parse().unwrap()),
                ));
            }
        }

        Self { robots }
    }

    fn part1(&mut self) -> Solution {
        let mut top_left_quad = 0;
        let mut top_right_quad = 0;
        let mut bottom_left_quad = 0;
        let mut bottom_right_quad = 0;
        for robot in self.robots.iter() {
            let mut i = robot.i();
            let mut j = robot.j();
            for _ in 0..100 {
                i += robot.di();
                j += robot.dj();

                if i < 0 {
                    i += MAX_HEIGHT;
                }

                if j < 0 {
                    j += MAX_WIDTH;
                }

                i %= MAX_HEIGHT;
                j %= MAX_WIDTH;
            }

            if i < MAX_HEIGHT / 2 && j < MAX_WIDTH / 2 {
                top_left_quad += 1;
            } else if i < MAX_HEIGHT / 2 && j > MAX_WIDTH / 2 {
                top_right_quad += 1;
            } else if i > MAX_HEIGHT / 2 && j < MAX_WIDTH / 2 {
                bottom_left_quad += 1;
            } else if i > MAX_HEIGHT / 2 && j > MAX_WIDTH / 2 {
                bottom_right_quad += 1;
            }
        }

        (top_left_quad * top_right_quad * bottom_left_quad * bottom_right_quad).into()
    }

    fn part2(&mut self) -> Solution {
        // Note: My original solution for this part is commented at the bottom of this file.
        //
        // In the original solution, I made the assumption that when the robots do arrange
        // themselves into the Easter egg, they will all be connected (i.e., a robot that is
        // part of the Easter egg will have robots as their direct neighbors). Assuming the
        // assumption is correct (which it is), the idea is pretty simple:
        // - For each second that passes,
        //      - Move each robot once according to the velocity vector given.
        //      - Then, run DFS on each robot and see if there's a large group of robots.
        //        Here, "large group of robots" simply means if the size of the group is greater
        //        than 30% of the total number of robots (30% being picked randomly).
        //      - If there is, then we're done and we can stop.
        //      - Else, we continue.
        // This theory doesn't appear to work on the example input (actually, the example input
        // doesn't look like it could ever *form* the Easter egg to begin with.
        //
        //
        //
        // It turns out that, after reviewing the Easter egg that is created, at the time when
        // the robots do form the Easter egg, there are NO overlaps whatsoever between robots.
        // In other words, for each potential spot a robot can be, there can be at MOST 1 robot
        // there. So, the optimized algorithm (directly below) simply uses that observation.
        let mut robot_locations = vec![];
        for robot in self.robots.iter() {
            robot_locations.push((robot.i(), robot.j()));
        }

        let mut seconds_elapsed = 0;
        loop {
            for (robot_idx, (i, j)) in robot_locations.iter_mut().enumerate() {
                *i += self.robots[robot_idx].di();
                *j += self.robots[robot_idx].dj();

                if *i < 0 {
                    *i += MAX_HEIGHT;
                }

                if *j < 0 {
                    *j += MAX_WIDTH;
                }

                *i %= MAX_HEIGHT;
                *j %= MAX_WIDTH;
            }

            if robot_locations.iter().collect::<HashSet<_>>().len() == self.robots.len() {
                return (seconds_elapsed + 1).into();
            }

            seconds_elapsed += 1;
        }
    }

    fn day() -> u32 {
        14
    }

    fn year() -> u32 {
        2024
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Robot {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Robot {
    pub fn new(location: (isize, isize), velocity: (isize, isize)) -> Self {
        let (x, y) = location;
        let (dx, dy) = velocity;
        Self { x, y, dx, dy }
    }

    /// Gets the row index. This is equivalent to getting the `y` field, but is slightly
    /// easier for me to understand when reading my code.
    ///
    /// # Returns
    /// The row index.
    pub fn i(&self) -> isize {
        self.y
    }

    /// Gets the column index. This is equivalent to getting the `x` field, but is slightly
    /// easier for me to understand when reading my code.
    ///
    /// # Returns
    /// The column index.
    pub fn j(&self) -> isize {
        self.x
    }

    /// Gets the change in the row index per second. This is equivalent to getting the `dy`
    /// field, but is slightly easier for me to understand when reading my code.
    ///
    /// # Returns
    /// The change in row index per second.
    pub fn di(&self) -> isize {
        self.dy
    }

    /// Gets the change in the column index per second. This is equivalent to getting the `dx`
    /// field, but is slightly easier for me to understand when reading my code.
    ///
    /// # Returns
    /// The change in column index per second.
    pub fn dj(&self) -> isize {
        self.dx
    }
}

/*
   fn part2(&mut self) -> Solution {
       fn get_length_of_connected_robots(robots: &[(isize, isize)], i: isize, j: isize, explored: &mut HashSet<(isize, isize)>) -> usize {
           fn explore(robots: &HashSet<(isize, isize)>, seen: &mut HashSet<(isize, isize)>, i: isize, j: isize) {
               if !robots.contains(&(i, j)) {
                   return;
               }

               if seen.contains(&(i, j)) {
                   return;
               }

               seen.insert((i, j));

               for di in -1..=1 {
                   for dj in -1..=1 {
                       if di == dj && di == 0 {
                           continue;
                       }

                       explore(robots, seen, i + di, j + dj);
                   }
               }
           }

           let mut seen = HashSet::new();
           explore(&robots.into_iter().cloned().collect(), &mut seen, i, j);
           explored.extend(seen.iter());
           seen.len()
       }

       let mut robot_locations = vec![];
       for robot in self.robots.iter() {
           robot_locations.push((robot.i(), robot.j()));
       }

       for n in 0..50000 {
           if n > 8000 {
               println!("{n}");
           }
           for (robot_idx, (i, j)) in robot_locations.iter_mut().enumerate() {
               *i += self.robots[robot_idx].di();
               *j += self.robots[robot_idx].dj();

               if *i < 0 {
                   *i += MAX_HEIGHT;
               }

               if *j < 0 {
                   *j += MAX_WIDTH;
               }

               *i %= MAX_HEIGHT;
               *j %= MAX_WIDTH;
           }

           let mut already_checked = HashSet::new();
           for pt @ (i, j) in robot_locations.iter() {
               if already_checked.contains(pt) {
                   continue;
               }


               let len = get_length_of_connected_robots(&robot_locations, *i, *j, &mut already_checked);
               if len > (robot_locations.len() as f64 * 0.3) as usize {
                   let mut display = vec![vec![0; MAX_WIDTH as usize]; MAX_HEIGHT as usize];
                   for (i, j) in robot_locations.iter() {
                       display[*i as usize][*j as usize] += 1;
                   }

                   let display = display.iter().map(|col| col.iter().map(|x| if *x > 0 { format!("{x}") } else { ".".to_string() }).collect::<Vec<_>>().join("")).collect::<Vec<_>>().join("\n");
                   println!("Iteration {} ({len} > {}):\n{}\n", n + 1, robot_locations.len(), display);
                   return (n + 1).into();
               }
           }
       }

       0.into()
   }
*/
