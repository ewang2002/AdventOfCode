use std::cmp::{max, min};
use std::collections::HashMap;
use crate::aoc::aoc_problem::AoCProblem;

pub struct Day05 {
    points: Vec<ToFromPoint>,
}

impl AoCProblem<usize, usize> for Day05 {
    fn prepare(input: Vec<String>) -> Self {
        return Day05 {
            points: input.iter()
                .map(|x| {
                    let pts = x.replace(" -> ", ",")
                        .split(",")
                        .map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>();
                    return ToFromPoint {x1: pts[0], y1: pts[1], x2: pts[2], y2: pts[3]};
                }).collect::<Vec<_>>()
        };
    }

    // --- Day 5: Hydrothermal Venture ---
    // You come across a field of hydrothermal vents on the ocean floor! These vents constantly
    // produce large, opaque clouds, so it would be best to avoid them if possible.
    //
    // They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents
    // (your puzzle input) for you to review. For example:
    //
    //  0,9 -> 5,9
    //  8,0 -> 0,8
    //  9,4 -> 3,4
    //  2,2 -> 2,1
    //  7,0 -> 7,4
    //  6,4 -> 2,0
    //  0,9 -> 2,9
    //  3,4 -> 1,4
    //  0,0 -> 8,8
    //  5,5 -> 8,2
    //
    // Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are
    // the coordinates of one end the line segment and x2,y2 are the coordinates of the other end.
    // These line segments include the points at both ends. In other words:
    //
    // An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
    // An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
    // For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
    //
    // So, the horizontal and vertical lines from the above list would produce the following
    // diagram:
    //
    //  .......1..
    //  ..1....1..
    //  ..1....1..
    //  .......1..
    //  .112111211
    //  ..........
    //  ..........
    //  ..........
    //  ..........
    //  222111....
    //
    // In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each
    // position is shown as the number of lines which cover that point or . if no line covers that
    // point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is
    // formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
    //
    // To avoid the most dangerous areas, you need to determine the number of points where at least
    // two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger
    // - a total of 5 points.
    //
    // Consider only horizontal and vertical lines. At how many points do at least two lines
    // overlap?
    fn part1(&self) -> usize {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        self.points.iter()
            .filter(|pts| pts.x1 == pts.x2 || pts.y1 == pts.y2)
            .for_each(|pts| {
                if pts.x1 == pts.x2 {
                    let min_y = min(pts.y1, pts.y2);
                    let max_y = max(pts.y1, pts.y2);
                    for y_val in min_y..=max_y {
                        let key = (pts.x1, y_val);
                        let entry = map.entry(key).or_insert(0);
                        *entry += 1;
                    }
                } else {
                    let min_x = min(pts.x1, pts.x2);
                    let max_x = max(pts.x1, pts.x2);
                    for x_val in min_x..=max_x {
                        let key = (x_val, pts.y1);
                        let entry = map.entry(key).or_insert(0);
                        *entry += 1;
                    }
                }
            });

        return map.values().filter(|x| x >= &&2).count();
    }

    // --- Part Two ---
    // Unfortunately, considering only horizontal and vertical lines doesn't give you the full
    // picture; you need to also consider diagonal lines.
    //
    // Because of the limits of the hydrothermal vent mapping system, the lines in your list will
    // only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
    //
    // An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
    // An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
    //
    // Considering all lines from the above example would now produce the following diagram:
    //
    //  1.1....11.
    //  .111...2..
    //  ..2.1.111.
    //  ...1.2.2..
    //  .112313211
    //  ...1.2....
    //  ..1...1...
    //  .1.....1..
    //  1.......1.
    //  222111....
    //
    // You still need to determine the number of points where at least two lines overlap. In the
    // above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12
    // points.
    //
    // Consider all of the lines. At how many points do at least two lines overlap?
    fn part2(&self) -> usize {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        self.points.iter()
            .filter(|pts| pts.x1 == pts.x2
                || pts.y1 == pts.y2
                || (pts.y1 - pts.y2).abs() == (pts.x1 - pts.x2).abs())
            .for_each(|pts| {
                let mut dx = if pts.x2 > pts.x1 {1} else {-1};
                let mut dy = if pts.y2 > pts.y1 {1} else {-1};

                if pts.x1 == pts.x2 {
                    dx = 0;
                }

                if pts.y1 == pts.y2 {
                    dy = 0;
                }

                let mut curr_x = pts.x1;
                let mut curr_y = pts.y1;
                let entry = map.entry((curr_x, curr_y)).or_insert(0);
                *entry += 1;

                while curr_x != pts.x2 || curr_y != pts.y2 {
                    curr_x += dx;
                    curr_y += dy;
                    let entry = map.entry((curr_x, curr_y)).or_insert(0);
                    *entry += 1;
                }
            });

        return map.values().filter(|x| x >= &&2).count();
    }
}

struct ToFromPoint {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}