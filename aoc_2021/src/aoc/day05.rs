use crate::aoc::aoc_problem::{AoCProblem, Solution};
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct Day05 {
    points: Vec<ToFromPoint>,
}

// https://adventofcode.com/2021/day/5
impl AoCProblem for Day05 {
    fn prepare(input: String) -> Self {
        return Day05 {
            points: input
                .lines()
                .map(|x| {
                    let pts = x
                        .replace(" -> ", ",")
                        .split(',')
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    ToFromPoint {
                        x1: pts[0],
                        y1: pts[1],
                        x2: pts[2],
                        y2: pts[3],
                    }
                })
                .collect::<Vec<_>>(),
        };
    }

    fn part1(&mut self) -> Solution {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        self.points
            .iter()
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

        return map.values().filter(|x| x >= &&2).count().into();
    }

    fn part2(&mut self) -> Solution {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        self.points
            .iter()
            .filter(|pts| {
                pts.x1 == pts.x2
                    || pts.y1 == pts.y2
                    || (pts.y1 - pts.y2).abs() == (pts.x1 - pts.x2).abs()
            })
            .for_each(|pts| {
                let mut dx = if pts.x2 > pts.x1 { 1 } else { -1 };
                let mut dy = if pts.y2 > pts.y1 { 1 } else { -1 };

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

        return map.values().filter(|x| x >= &&2).count().into();
    }
}

struct ToFromPoint {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
