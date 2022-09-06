type PowerGrid = [[i32; 300]; 300];

// https://adventofcode.com/2018/day/11
#[allow(dead_code)]
pub fn execute(input: &[String]) -> (String, String) {
    let serial_number = input[0].parse::<i32>().expect("parse error.");
    (part1(serial_number), part2(serial_number))
}

pub fn part1(serial_number: i32) -> String {
    let power_grid = construct_grid(serial_number);
    let mut max_sum = 0;
    let mut top_left_x = 0;
    let mut top_left_y = 0;
    for y in 0..(300 - 3) {
        for x in 0..(300 - 3) {
            let mut sum = 0;

            for dy in 0..3 {
                for dx in 0..3 {
                    sum += power_grid[x + dx][y + dy];
                }
            }

            if sum > max_sum {
                top_left_x = x;
                top_left_y = y;
                max_sum = sum;
            }
        }
    }

    format!("{},{}", top_left_x + 1, top_left_y + 1)
}

pub fn part2(serial_number: i32) -> String {
    let power_grid = construct_grid(serial_number);
    let mut all_regions: Vec<BoxedRegion> = vec![];
    for y in 0..300 {
        for x in 0..300 {
            all_regions.push(BoxedRegion::new(x, y, x, y, &power_grid));
        }
    }

    let mut max_region: BoxedRegion = *all_regions.iter().max_by_key(|x| x.sum).unwrap();

    for _ in 1..300 {
        // all_regions.iter_mut().for_each(|x| x.add_one(&power_grid));
        // all_regions.retain(|x| x.is_valid);
        //
        // let max_region_here = all_regions
        //     .iter()
        //     .max_by_key(|x| x.sum)
        //     .unwrap();
        // if max_region_here.sum > max_region.sum {
        //     max_region = max_region_here.clone();
        // }
        let mut valid_regions: Vec<BoxedRegion> = vec![];
        let mut max_region_here = all_regions[0];
        for mut region in all_regions {
            region.add_one(&power_grid);
            if region.is_valid {
                if region.sum > max_region_here.sum {
                    max_region_here = region;
                }

                valid_regions.push(region);
                continue;
            }
        }

        if max_region_here.sum > max_region.sum {
            max_region = max_region_here;
        }

        all_regions = valid_regions;
    }

    format!(
        "{},{},{}",
        max_region.top_left_x + 1,
        max_region.top_left_y + 1,
        max_region.bottom_right_y - max_region.top_left_y + 1
    )
}

/// Constructs the power grid for this problem.
///
/// # Parameters
/// - `serial_number`: The serial number.
///
/// # Returns
/// - The 300x300 power grid.
fn construct_grid(serial_number: i32) -> PowerGrid {
    let mut power_grid: PowerGrid = [[0; 300]; 300];
    for x in 0..300 {
        let act_x = x + 1;

        // Find the fuel cell's rack ID, which is its X coordinate plus 10.
        let rack_id = act_x + 10;

        for y in 0..300 {
            let act_y = y + 1;
            // Begin with a power level of the rack ID times the Y coordinate.
            // Increase the power level by the value of the grid serial number (your puzzle input).
            let mut power_level = rack_id * act_y + serial_number;
            // Set the power level to itself multiplied by the rack ID.
            power_level *= rack_id;
            // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no
            // hundreds digit become 0).
            power_level = (power_level / 10_i32.pow(3 - 1)) % 10;
            // Subtract 5 from the power level.
            power_grid[x as usize][y as usize] = power_level - 5;
        }
    }

    power_grid
}

#[derive(Debug, Copy, Clone)]
struct BoxedRegion {
    top_left_x: usize,
    top_left_y: usize,
    bottom_right_x: usize,
    bottom_right_y: usize,
    sum: i32,
    is_valid: bool,
}

impl BoxedRegion {
    /// Creates a new `BoxedRegion` structure with the specified boxed coordinates and the power
    /// grid.
    ///
    /// # Parameters
    /// - `top_left_x`: The top-left `x`-coordinate.
    /// - `top_left_y`: The top-left `y`-coordinate.
    /// - `bottom_right_x`: The bottom-right `x`-coordinate.
    /// - `bottom_right_y`: The bottom-right `y`-coordinate.
    /// - `grid`: The power grid.
    ///
    /// # Returns
    /// - The new `BoxedRegion` structure.
    #[allow(clippy::needless_range_loop)]
    pub fn new(
        top_left_x: usize,
        top_left_y: usize,
        bottom_right_x: usize,
        bottom_right_y: usize,
        grid: &PowerGrid,
    ) -> Self {
        let mut sum: i32 = 0;
        for y in top_left_y..=bottom_right_y {
            for x in top_left_x..=bottom_right_x {
                sum += grid[x][y];
            }
        }

        BoxedRegion {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            sum,
            is_valid: true,
        }
    }

    /// Adds one to the bottom right `(x, y)` coordinates, adding the appropriate sums as needed.
    ///
    /// This will modify `is_valid` as needed.
    ///
    /// # Parameters
    /// - `grid`: The power grid.
    #[allow(clippy::needless_range_loop)]
    pub fn add_one(&mut self, grid: &PowerGrid) {
        self.bottom_right_x += 1;
        self.bottom_right_y += 1;
        if self.bottom_right_x >= 300 || self.bottom_right_y >= 300 {
            self.is_valid = false;
            return;
        }

        // Bottom right-most edge
        self.sum += grid[self.bottom_right_x][self.bottom_right_y];

        // Right edge excluding bottom right-most edge
        for y in self.top_left_y..self.bottom_right_y {
            self.sum += grid[self.bottom_right_x][y]
        }

        for x in self.top_left_x..self.bottom_right_x {
            self.sum += grid[x][self.bottom_right_y];
        }

        self.is_valid = true;
    }
}
