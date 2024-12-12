use std::collections::HashSet;

use common::problem::day::{AoCProblem, Solution};

pub struct Day12 {
    garden_plots: Vec<Vec<char>>,
}

impl AoCProblem for Day12 {
    fn prepare(input: String) -> Self {
        Self {
            garden_plots: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut price = 0;
        let mut explored = HashSet::new();
        for (row_idx, row) in self.garden_plots.iter().enumerate() {
            for (col_idx, plant) in row.iter().enumerate() {
                if explored.contains(&(row_idx as isize, col_idx as isize)) {
                    continue;
                }

                let (perimeter, seen) = calculate_area_perimeter(
                    &self.garden_plots,
                    row_idx as isize,
                    col_idx as isize,
                    *plant,
                );
                price += perimeter * seen.len();
                explored.extend(seen);
            }
        }

        price.into()
    }

    fn part2(&mut self) -> Solution {
        // To better calculate the number of sides, I'm going to increase the size of the graph by 3x the amount.
        // For example, the graph
        //
        //          AAAA
        //          BBCD
        //          BBCC
        //          EEEC
        //
        // will become
        //
        //          AAAAAAAAAAAA
        //          AAAAAAAAAAAA
        //          AAAAAAAAAAAA
        //          BBBBBBCCCDDD
        //          BBBBBBCCCDDD
        //          BBBBBBCCCDDD
        //          BBBBBBCCCCCC
        //          BBBBBBCCCCCC
        //          BBBBBBCCCCCC
        //          EEEEEEEEECCC
        //          EEEEEEEEECCC
        //          EEEEEEEEECCC
        //
        // Then, if we want to find how many sides there are, we can easily just DFS along the sides of the region without
        // needing to worry about tracking explored points or edge cases like skipping parts of a region.
        let mut scaled_plot =
            vec![vec!['.'; self.garden_plots[0].len() * 3]; self.garden_plots.len() * 3];
        for (i, row) in self.garden_plots.iter().enumerate() {
            for (j, plant) in row.iter().enumerate() {
                for a in 0..3 {
                    for b in 0..3 {
                        scaled_plot[i * 3 + a][j * 3 + b] = *plant;
                    }
                }
            }
        }

        let mut price = 0;
        let mut explored = HashSet::new();
        for (row_idx, row) in scaled_plot.iter().enumerate() {
            for (col_idx, plant) in row.iter().enumerate() {
                if explored.contains(&(row_idx as isize, col_idx as isize)) {
                    continue;
                }

                let (_, seen) = calculate_area_perimeter(
                    &scaled_plot,
                    row_idx as isize,
                    col_idx as isize,
                    *plant,
                );

                let num_sides = calculate_num_sides(&scaled_plot, *plant, &seen);
                price += (seen.len() / 9) * num_sides;
                explored.extend(seen);
            }
        }

        price.into()
    }

    fn day() -> u32 {
        12
    }

    fn year() -> u32 {
        2024
    }
}

const DIRECTIONS: [(isize, isize); 4] = [
    // Right
    (0, 1),
    // Down
    (1, 0),
    // Left
    (0, -1),
    // Up
    (-1, 0),
];

const CORNER_DIRECTIONS: [(isize, isize); 4] = [(1, -1), (1, 1), (-1, -1), (-1, 1)];

fn is_out_of_bounds(garden_plot: &[Vec<char>], i: isize, j: isize) -> bool {
    i < 0 || i >= garden_plot.len() as isize || j < 0 || j >= garden_plot[0].len() as isize
}

/// Calculates the number of sides this region has.
///
/// # Parameters
/// - `garden_plot`: The garden plot.
/// - `plant`: The plant belonging to the region.
/// - `region`: The list of all points within this region.
///
/// # Returns
/// The number of sides this region has.
fn calculate_num_sides(
    garden_plot: &[Vec<char>],
    plant: char,
    region: &HashSet<(isize, isize)>,
) -> usize {
    // Get a set of all the points that belong to the edge of the region.
    let edge_points = region
        .iter()
        .cloned()
        .filter(|&(pt_i, pt_j)| {
            [DIRECTIONS, CORNER_DIRECTIONS]
                .concat()
                .into_iter()
                .any(|(di, dj)| {
                    is_out_of_bounds(garden_plot, pt_i + di, pt_j + dj)
                        || garden_plot[(pt_i + di) as usize][(pt_j + dj) as usize] != plant
                })
        })
        .collect::<HashSet<_>>();

    edge_points
        .iter()
        .filter(|&&(i, j)| {
            (edge_points.contains(&(i + 1, j)) || edge_points.contains(&(i - 1, j)))
                && (edge_points.contains(&(i, j + 1)) || edge_points.contains(&(i, j - 1)))
        })
        .count()
}

/// Calculates the area and perimeter of the plant region.
///
/// # Parameters
/// - `garden_plot`: The garden plot.
/// - `i`: The `i`th index where we should start calculating the area and perimeter.
/// - `j`: The `j`th index where we should start calculating the area and perimeter.
/// - `plant`: The plant to look for.
///
/// # Returns
/// A tuple where the first item is the perimeter and the second item is a set of all
/// points in the region (which can be used to get the area).
fn calculate_area_perimeter(
    garden_plot: &[Vec<char>],
    i: isize,
    j: isize,
    plant: char,
) -> (usize, HashSet<(isize, isize)>) {
    fn helper(
        garden_plot: &[Vec<char>],
        plant: char,
        i: isize,
        j: isize,
        explored: &mut HashSet<(isize, isize)>,
        perimeter: &mut usize,
    ) {
        if explored.contains(&(i, j)) {
            return;
        }

        explored.insert((i, j));
        // Go check out the other neighbors
        let mut same_plant_neighbors = 0;
        for (di, dj) in DIRECTIONS {
            if is_out_of_bounds(garden_plot, i + di, j + dj) {
                continue;
            }

            if garden_plot[(i + di) as usize][(j + dj) as usize] != plant {
                continue;
            }

            helper(garden_plot, plant, i + di, j + dj, explored, perimeter);
            same_plant_neighbors += 1;
        }

        *perimeter += 4 - same_plant_neighbors;
    }

    let mut perimeter = 0;
    let mut explored = HashSet::new();
    helper(garden_plot, plant, i, j, &mut explored, &mut perimeter);
    (perimeter, explored)
}
