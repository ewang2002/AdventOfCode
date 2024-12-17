use std::collections::{HashMap, VecDeque};

use common::{
    constants::TWO_NEWLINE,
    problem::day::{AoCProblem, Solution},
};

pub struct Day15 {
    raw_warehouse_map: String,
    direction_list: Vec<Direction>,
}

impl AoCProblem for Day15 {
    fn prepare(input: String) -> Self {
        let (raw_warehouse_map, directions) = input.split_once(TWO_NEWLINE).unwrap();
        let mut direction_list = vec![];
        for row in directions.lines() {
            direction_list.extend(row.chars().map(Direction::new));
        }

        Self {
            raw_warehouse_map: String::from(raw_warehouse_map),
            direction_list,
        }
    }

    fn part1(&mut self) -> Solution {
        let mut warehouse_map = vec![];
        let mut robot_i = 0;
        let mut robot_j = 0;
        for (ridx, row) in self.raw_warehouse_map.lines().enumerate() {
            let mut map_row = vec![];
            for (cidx, c) in row.char_indices() {
                if c == '@' {
                    (robot_i, robot_j) = (ridx as isize, cidx as isize);
                }

                map_row.push(MapItem::new(c));
            }

            warehouse_map.push(map_row);
        }

        for direction in self.direction_list.iter() {
            let (di, dj) = direction.directional_vector();
            let mut base_i = robot_i;
            let mut base_j = robot_j;
            // Look for next free space
            loop {
                match warehouse_map[base_i as usize][base_j as usize] {
                    MapItem::Robot => {
                        base_i += di;
                        base_j += dj;
                    }
                    MapItem::Box => {
                        base_i += di;
                        base_j += dj;
                    }
                    MapItem::Wall => {
                        // No free space left
                        base_i = robot_i;
                        base_j = robot_j;
                        break;
                    }
                    MapItem::Free => {
                        break;
                    }
                }
            }

            // If we have no free spots left, then we can just continue.
            if base_i == robot_i && base_j == robot_j {
                continue;
            }

            let mut next_i = base_i;
            let mut next_j = base_j;

            loop {
                match warehouse_map[next_i as usize][next_j as usize] {
                    MapItem::Robot => {
                        warehouse_map[next_i as usize][next_j as usize] = MapItem::Free;
                        while warehouse_map[base_i as usize][base_j as usize] == MapItem::Box {
                            base_i += -di;
                            base_j += -dj;
                        }
                        warehouse_map[base_i as usize][base_j as usize] = MapItem::Robot;
                        robot_i = base_i;
                        robot_j = base_j;
                        break;
                    }
                    MapItem::Box => {
                        warehouse_map[next_i as usize][next_j as usize] = MapItem::Free;
                        while warehouse_map[base_i as usize][base_j as usize] == MapItem::Box {
                            base_i += -di;
                            base_j += -dj;
                        }
                        warehouse_map[base_i as usize][base_j as usize] = MapItem::Box;
                        next_i += -di;
                        next_j += -dj;
                    }
                    MapItem::Wall => {
                        break;
                    }
                    MapItem::Free => {
                        next_i += -di;
                        next_j += -dj;
                    }
                }
            }
        }

        let mut gps_sum = 0;
        for (rid, row) in warehouse_map.into_iter().enumerate() {
            for cid in (0..row.len()).filter(|cid| row[*cid] == MapItem::Box) {
                gps_sum += 100 * rid + cid;
            }
        }

        gps_sum.into()
    }

    fn part2(&mut self) -> Solution {
        let mut warehouse_map = vec![];
        let mut robot_i = 0;
        let mut robot_j = 0;
        let mut box_id = 0;
        for row in self.raw_warehouse_map.lines() {
            let mut map_row = vec![];
            for c in row.chars() {
                match c {
                    '@' => {
                        (robot_i, robot_j) = (warehouse_map.len() as isize, map_row.len() as isize);
                        map_row.push(ModifiedMapItem::Robot);
                        map_row.push(ModifiedMapItem::Free);
                    }
                    '.' => {
                        map_row.push(ModifiedMapItem::Free);
                        map_row.push(ModifiedMapItem::Free);
                    }
                    '#' => {
                        map_row.push(ModifiedMapItem::Wall);
                        map_row.push(ModifiedMapItem::Wall);
                    }
                    'O' => {
                        map_row.push(ModifiedMapItem::DoubleBox(box_id, false));
                        map_row.push(ModifiedMapItem::DoubleBox(box_id, true));
                        box_id += 1;
                    }
                    _ => {}
                };
            }

            warehouse_map.push(map_row);
        }

        for direction in self.direction_list.iter() {
            let (di, dj) = direction.directional_vector();

            // First, figure out what the robot will end up moving.
            //  - points_to_move will hold a list of all points that will be moved.
            let mut points_to_move = HashMap::new();

            let mut stack = VecDeque::new();
            stack.push_back((robot_i, robot_j));
            while let Some((i, j)) = stack.pop_back() {
                if points_to_move.contains_key(&(i, j)) {
                    continue;
                }

                match warehouse_map[i as usize][j as usize] {
                    ModifiedMapItem::Wall => {
                        // If we have a wall, then we can't even move that direction.
                        points_to_move.clear();
                        break;
                    }
                    ModifiedMapItem::Free => {
                        // If this next spot is a free spot, then we are done with this iteration.
                        continue;
                    }
                    ModifiedMapItem::Robot => {
                        points_to_move.insert((i, j), ModifiedMapItem::Robot);
                        stack.push_back((i + di, j + dj));
                    }
                    ModifiedMapItem::DoubleBox(_, is_right) => {
                        points_to_move.insert((i, j), warehouse_map[i as usize][j as usize]);
                        match (di, dj) {
                            (0, 1) | (0, -1) => {
                                stack.push_back((i + di, j + dj));
                            }
                            (1, 0) | (-1, 0) => {
                                stack.push_back((i + di, j + dj));
                                // If we're going down, we need to consider the box that might be next to us (right/left)...
                                if is_right {
                                    stack.push_back((i, j - 1));
                                } else {
                                    stack.push_back((i, j + 1));
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }

            // If we have no points to move, then we're done with this direction
            if points_to_move.is_empty() {
                continue;
            }

            // Otherwise, we move each point as needed
            for (i, j) in points_to_move.keys() {
                warehouse_map[*i as usize][*j as usize] = ModifiedMapItem::Free;
            }

            for ((i, j), item) in points_to_move {
                if item == ModifiedMapItem::Robot {
                    robot_i = i + di;
                    robot_j = j + dj;
                }
                warehouse_map[(i + di) as usize][(j + dj) as usize] = item;
            }
        }

        let mut gps_sum = 0;
        for (rid, row) in warehouse_map.into_iter().enumerate() {
            for cid in (0..row.len())
                .filter(|cid| matches!(row[*cid], ModifiedMapItem::DoubleBox(_, false)))
            {
                gps_sum += 100 * rid + cid;
            }
        }

        gps_sum.into()
    }

    fn day() -> u32 {
        15
    }

    fn year() -> u32 {
        2024
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ModifiedMapItem {
    Robot,
    Wall,
    Free,
    DoubleBox(usize, bool),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapItem {
    Robot,
    Box,
    Wall,
    Free,
}

impl MapItem {
    pub fn new(map_char: char) -> Self {
        match map_char {
            '#' => MapItem::Wall,
            '.' => MapItem::Free,
            'O' => MapItem::Box,
            '@' => MapItem::Robot,
            _ => unimplemented!("character {map_char} not supported"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Direction {
    pub fn new(move_char: char) -> Self {
        match move_char {
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '^' => Direction::Up,
            _ => unimplemented!("character {move_char} not supported"),
        }
    }

    pub fn directional_vector(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
        }
    }
}
