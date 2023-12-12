use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use common::problem::day::{AoCProblem, Solution};

pub type Point = (usize, usize);

pub const VERTICAL_TILE: char = '|';
pub const HORIZONTAL_TILE: char = '-';
pub const NORTH_EAST_TILE: char = 'L';
pub const NORTH_WEST_TILE: char = 'J';
pub const SOUTH_WEST_TILE: char = '7';
pub const SOUTH_EAST_TILE: char = 'F';
pub const GROUND_TILE: char = '.';
pub const START_TILE: char = 'S';
pub const MARKED_TILE: char = '#';

pub struct Day10 {
    surface_pipes: Vec<Vec<Tile>>,
    starting_location: Point,
}

impl AoCProblem for Day10 {
    fn prepare(input: String) -> Self {
        let mut starting_location = (0, 0);

        let raw_surface_pipes: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();

        let mut surface_pipes = vec![];
        for (row_idx, row) in raw_surface_pipes.iter().enumerate() {
            let mut this_row = vec![];
            for (col_idx, c) in row.iter().enumerate() {
                this_row.push(match *c {
                    VERTICAL_TILE => Tile::Vertical,
                    HORIZONTAL_TILE => Tile::Horizontal,
                    NORTH_WEST_TILE => Tile::NorthWestBend,
                    NORTH_EAST_TILE => Tile::NorthEastBend,
                    SOUTH_WEST_TILE => Tile::SouthWestBend,
                    SOUTH_EAST_TILE => Tile::SouthEastBend,
                    GROUND_TILE => Tile::Ground,
                    START_TILE => {
                        starting_location = (row_idx, col_idx);
                        let mut can_go_up = false;
                        let mut can_go_down = false;
                        let mut can_go_right = false;
                        let mut can_go_left = false;
                        if row_idx != 0 {
                            let up_raw_tile = raw_surface_pipes[row_idx - 1][col_idx];
                            // Can either be vertical (|) or southwest bend (7) or southeast bend (F)
                            can_go_up = up_raw_tile == VERTICAL_TILE
                                || up_raw_tile == SOUTH_WEST_TILE
                                || up_raw_tile == SOUTH_EAST_TILE;
                        }

                        if row_idx + 1 < raw_surface_pipes.len() {
                            let down_raw_tile = raw_surface_pipes[row_idx + 1][col_idx];
                            // Can either be vertical (|) or northeast bend (L) or northwest bend (J)
                            can_go_down = down_raw_tile == VERTICAL_TILE
                                || down_raw_tile == NORTH_EAST_TILE
                                || down_raw_tile == NORTH_WEST_TILE;
                        }

                        if col_idx != 0 {
                            let left_raw_tile = raw_surface_pipes[row_idx][col_idx - 1];
                            // Can either be horizontal (-) or northeast (L) or southeast (F)
                            can_go_left = left_raw_tile == HORIZONTAL_TILE
                                || left_raw_tile == NORTH_EAST_TILE
                                || left_raw_tile == SOUTH_EAST_TILE;
                        }

                        if col_idx + 1 < raw_surface_pipes[0].len() {
                            let right_raw_tile = raw_surface_pipes[row_idx][col_idx + 1];
                            // Can either be horizontal (-) or J (northwest) or 7 (southwest)
                            can_go_right = right_raw_tile == HORIZONTAL_TILE
                                || right_raw_tile == NORTH_WEST_TILE
                                || right_raw_tile == SOUTH_WEST_TILE;
                        }

                        match (can_go_down, can_go_up, can_go_left, can_go_right) {
                            (true, true, false, false) => Tile::Vertical,
                            (false, false, true, true) => Tile::Horizontal,
                            (true, false, true, false) => Tile::SouthWestBend,
                            (false, true, false, true) => Tile::NorthEastBend,
                            (true, false, false, true) => Tile::SouthEastBend,
                            (false, true, true, false) => Tile::NorthWestBend,
                            c => panic!("unknown start directional: {c:?}"),
                        }
                    }
                    _ => unreachable!(),
                });
            }

            surface_pipes.push(this_row);
        }

        Self {
            surface_pipes,
            starting_location,
        }
    }

    fn part1(&mut self) -> Solution {
        // We're assuming that the longest path is just the path that involves exploring every single
        // tile connected from the start. Divide by 2 because we aren't allowed to step on the same
        // tiles again.
        (get_loop_tile_coordinates(&self.surface_pipes, self.starting_location).len() / 2).into()
    }

    fn part2(&mut self) -> Solution {
        // Step 0: We want to get all coordinates that are connected to the main loop. In particular,
        // "Any tile that isn't part of the main loop can count as being enclosed by the loop."
        let main_loop_coords =
            get_loop_tile_coordinates(&self.surface_pipes, self.starting_location);

        // Step 1: Scale the grid up by 3x by representing each tile as a char[3][3]. For example, the tiles
        // '|' and 'L' would be represented as
        //          .#.          .#.
        //          .#.          .##
        //          .#.          ...
        //
        // This way, we can see exactly what tiles are accessible from outside the loop (without needing
        // to worry about the squeezing aspect of the initial grid).
        let mut scaled_grid =
            vec![vec![GROUND_TILE; self.surface_pipes[0].len() * 3]; self.surface_pipes.len() * 3];
        for (row_idx, row) in self.surface_pipes.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                // If this tile isn't part of the main loop, then we don't care about it. Like the instructions
                // said, "Any tile that isn't part of the main loop can count as being enclosed by the loop."
                if !main_loop_coords.contains(&(row_idx, col_idx)) {
                    continue;
                }

                let scaled_row_idx = row_idx * 3;
                let scaled_col_idx = col_idx * 3;
                match tile {
                    Tile::Vertical => {
                        scaled_grid[scaled_row_idx][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 2][scaled_col_idx + 1] = MARKED_TILE;
                    }
                    Tile::Horizontal => {
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 2] = MARKED_TILE;
                    }
                    Tile::NorthEastBend => {
                        scaled_grid[scaled_row_idx][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 2] = MARKED_TILE;
                    }
                    Tile::NorthWestBend => {
                        scaled_grid[scaled_row_idx][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx] = MARKED_TILE;
                    }
                    Tile::SouthWestBend => {
                        scaled_grid[scaled_row_idx + 2][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx] = MARKED_TILE;
                    }
                    Tile::SouthEastBend => {
                        scaled_grid[scaled_row_idx + 2][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 1] = MARKED_TILE;
                        scaled_grid[scaled_row_idx + 1][scaled_col_idx + 2] = MARKED_TILE;
                    }
                    Tile::Ground => continue,
                }
            }
        }

        // Step 2: DFS through the scaled grid to see what's reachable from the outside. For this, we can
        // pick any random starting point as long as it's by the edge of the grid.
        let starting_point = scaled_grid[0]
            .iter()
            .enumerate()
            .find(|(_, t)| **t == GROUND_TILE)
            .map(|(col, _)| (0, col))
            .or_else(|| {
                scaled_grid[scaled_grid.len() - 1]
                    .iter()
                    .enumerate()
                    .find(|(_, t)| **t == GROUND_TILE)
                    .map(|(col, _)| (scaled_grid.len() - 1, col))
            })
            .expect("could not find starting point.");

        let mut stack = vec![starting_point];
        while let Some((x, y)) = stack.pop() {
            if scaled_grid[x][y] == MARKED_TILE {
                continue;
            }

            scaled_grid[x][y] = MARKED_TILE;
            if x != 0 {
                stack.push((x - 1, y));
            }

            if x + 1 < scaled_grid.len() {
                stack.push((x + 1, y));
            }

            if y != 0 {
                stack.push((x, y - 1));
            }

            if y + 1 < scaled_grid[0].len() {
                stack.push((x, y + 1));
            }
        }

        // Step 3: We now want to compute the number of 3x3 grids that contains JUST
        // the ground tile. In other words, we want to compute the number of coordinates
        // (x, y) such that x % 3 == 0 AND y % 3 == 0 AND, with the fact that this coordinate
        // is the top-left coordinate of this 3x3 grid, whose corresponding grid is
        //
        //          ...
        //          ...
        //          ...
        let mut area_enclosed = 0;
        for r_idx in (0..scaled_grid.len()).step_by(3) {
            for c_idx in (0..scaled_grid[r_idx].len()).step_by(3) {
                let mut all_ground = true;
                'grid_check: for dr in 0..3 {
                    for dc in 0..3 {
                        if scaled_grid[r_idx + dr][c_idx + dc] != GROUND_TILE {
                            all_ground = false;
                            break 'grid_check;
                        }
                    }
                }

                if all_ground {
                    area_enclosed += 1;
                }
            }
        }

        // 31 - too low
        print_grid(&scaled_grid);
        area_enclosed.into()
    }

    fn day() -> u32 {
        10
    }

    fn year() -> u32 {
        2023
    }
}

/// Represents a tile in the surface pipe.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    /// | is a vertical pipe connecting north and south.
    Vertical,
    /// - is a horizontal pipe connecting east and west.
    Horizontal,
    /// L is a 90-degree bend connecting north and east.
    NorthEastBend,
    /// J is a 90-degree bend connecting north and west.
    NorthWestBend,
    /// 7 is a 90-degree bend connecting south and west.
    SouthWestBend,
    /// F is a 90-degree bend connecting south and east.
    SouthEastBend,
    /// . is ground; there is no pipe in this tile.
    Ground,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Vertical => f.write_char(VERTICAL_TILE),
            Tile::Horizontal => f.write_char(HORIZONTAL_TILE),
            Tile::NorthEastBend => f.write_char(NORTH_EAST_TILE),
            Tile::NorthWestBend => f.write_char(NORTH_WEST_TILE),
            Tile::SouthWestBend => f.write_char(SOUTH_WEST_TILE),
            Tile::SouthEastBend => f.write_char(SOUTH_EAST_TILE),
            Tile::Ground => f.write_char(GROUND_TILE),
        }
    }
}

/// Gets the coordinates of all tiles that are connected to the main loop.
///
/// # Parameters
/// - `surface_pipes`: The surface pipes.
/// - `starting_location`: The starting location.
///
/// # Returns
/// The coordinates of all tiles that are connected to the main loop.
fn get_loop_tile_coordinates(
    surface_pipes: &[Vec<Tile>],
    starting_location: Point,
) -> HashSet<Point> {
    let mut coords: HashSet<Point> = HashSet::new();
    let mut stack: Vec<Point> = vec![];
    stack.push(starting_location);

    while let Some(p @ (x, y)) = stack.pop() {
        if coords.contains(&p) {
            continue;
        }

        coords.insert(p);
        match surface_pipes[x][y] {
            Tile::Vertical => {
                // Check up and down
                if x != 0 {
                    stack.push((x - 1, y));
                }

                if x + 1 < surface_pipes.len() {
                    stack.push((x + 1, y));
                }
            }
            Tile::Horizontal => {
                // Check left and right
                if y != 0 {
                    stack.push((x, y - 1));
                }

                if y + 1 < surface_pipes[0].len() {
                    stack.push((x, y + 1));
                }
            }
            Tile::NorthEastBend => {
                // Check up and right
                if x != 0 {
                    stack.push((x - 1, y));
                }

                if y + 1 < surface_pipes[0].len() {
                    stack.push((x, y + 1));
                }
            }
            Tile::NorthWestBend => {
                // Check up and left
                if x != 0 {
                    stack.push((x - 1, y));
                }

                if y != 0 {
                    stack.push((x, y - 1));
                }
            }
            Tile::SouthWestBend => {
                // Check down and left
                if x + 1 < surface_pipes.len() {
                    stack.push((x + 1, y));
                }

                if y != 0 {
                    stack.push((x, y - 1));
                }
            }
            Tile::SouthEastBend => {
                // Check down and right
                if x + 1 < surface_pipes.len() {
                    stack.push((x + 1, y));
                }

                if y + 1 < surface_pipes[0].len() {
                    stack.push((x, y + 1));
                }
            }
            Tile::Ground => continue,
        }
    }

    coords
}

/// Prints the grid.
///
/// # Parameters
/// - `grid`: The grid.
#[allow(dead_code)]
fn print_grid<T: Display>(grid: &[Vec<T>]) {
    println!(
        "{}\n",
        grid.iter()
            .map(|row| row
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(""))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
