type PowerGrid = [[i32; 300]; 300];

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (String, String) {
    let serial_number = input[0].parse::<i32>()
        .expect("parse error.");
    return (part1(serial_number), part2(serial_number));
}

// --- Day 11: Chronal Charge ---
// You watch the Elves and their sleigh fade into the distance as they head toward the North Pole.
//
// Actually, you're the one fading. The falling sensation returns.
//
// The low fuel warning light is illuminated on your wrist-mounted device. Tapping it once causes
// it to project a hologram of the situation: a 300x300 grid of fuel cells and their current power
// levels, some negative. You're not sure what negative power means in the context of time travel,
// but it can't be good.
//
// Each fuel cell has a coordinate ranging from 1 to 300 in both the X (horizontal) and Y
// (vertical) direction. In X,Y notation, the top-left cell is 1,1, and the top-right cell is 300,1.
//
// The interface lets you select any 3x3 square of fuel cells. To increase your chances of getting
// to your destination, you decide to choose the 3x3 square with the largest total power.
//
// The power level in a given fuel cell can be found through the following process:
//
// - Find the fuel cell's rack ID, which is its X coordinate plus 10.
// - Begin with a power level of the rack ID times the Y coordinate.
// - Increase the power level by the value of the grid serial number (your puzzle input).
// - Set the power level to itself multiplied by the rack ID.
// - Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds
// digit become 0).
// - Subtract 5 from the power level.
//
// For example, to find the power level of the fuel cell at 3,5 in a grid with serial number 8:
// - The rack ID is 3 + 10 = 13.
// - The power level starts at 13 * 5 = 65.
// - Adding the serial number produces 65 + 8 = 73.
// - Multiplying by the rack ID produces 73 * 13 = 949.
// - The hundreds digit of 949 is 9.
// - Subtracting 5 produces 9 - 5 = 4.
// So, the power level of this fuel cell is 4.
//
// Here are some more example power levels:
// - Fuel cell at  122,79, grid serial number 57: power level -5.
// - Fuel cell at 217,196, grid serial number 39: power level  0.
// - Fuel cell at 101,153, grid serial number 71: power level  4.
//
// Your goal is to find the 3x3 square which has the largest total power. The square must be
// entirely within the 300x300 grid. Identify this square using the X,Y coordinate of its top-left
// fuel cell. For example:
//
// For grid serial number 18, the largest total 3x3 square has a top-left corner of 33,45 (with a
// total power of 29); these fuel cells appear in the middle of this 5x5 region:
//
//  -2  -4   4   4   4
//  -4   4   4   4  -5
//   4   3   3   4  -4
//   1   1   2   4  -3
//  -1   0   2  -5  -2
//
// For grid serial number 42, the largest 3x3 square's top-left is 21,61 (with a total power of
// 30); they are in the middle of this region:
//
//  -3   4   2   2   2
//  -4   4   3   3   4
//  -5   3   3   4  -4
//   4   3   3   4  -3
//   3   3   3  -5  -1
// What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total
// power?

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

    return format!("{},{}", top_left_x + 1, top_left_y + 1);
}

// --- Part Two ---
// You discover a dial on the side of the device; it seems to let you select a square of any size,
// not just 3x3. Sizes from 1x1 to 300x300 are supported.
//
// Realizing this, you now must find the square of any size with the largest total power. Identify
// this square by including its size as a third parameter after the top-left coordinate: a 9x9
// square with a top-left corner of 3,5 is identified as 3,5,9.
//
// For example:
// - For grid serial number 18, the largest total square (with a total power of 113) is 16x16 and
// has a top-left corner of 90,269, so its identifier is 90,269,16.
// - For grid serial number 42, the largest total square (with a total power of 119) is 12x12 and
// has a top-left corner of 232,251, so its identifier is 232,251,12.
//
// What is the X,Y,size identifier of the square with the largest total power?

pub fn part2(serial_number: i32) -> String {
    let power_grid = construct_grid(serial_number);
    let mut all_regions: Vec<BoxedRegion> = vec![];
    for y in 0..300 {
        for x in 0..300 {
            all_regions.push(BoxedRegion::new(x, y, x, y, &power_grid));
        }
    }

    let mut max_region: BoxedRegion = all_regions
        .iter()
        .max_by_key(|x| x.sum)
        .unwrap()
        .clone();

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

    return format!("{},{},{}", max_region.top_left_x + 1, max_region.top_left_y + 1,
                   max_region.bottom_right_y - max_region.top_left_y + 1);
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

    return power_grid;
}

/// Prints the power grid out.
///
/// # Parameters
/// - `grid`: The grid to print out.
#[allow(dead_code)]
fn print_power_grid(grid: &PowerGrid) -> () {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            print!("{}\t", grid[x][y]);
        }
        println!();
    }
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
    pub fn new(top_left_x: usize, top_left_y: usize, bottom_right_x: usize, bottom_right_y: usize,
               grid: &PowerGrid) -> Self {
        let mut sum: i32 = 0;
        for y in top_left_y..=bottom_right_y {
            for x in top_left_x..=bottom_right_x {
                sum += grid[x][y];
            }
        }

        return BoxedRegion {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            sum,
            is_valid: true,
        };
    }

    /// Adds one to the bottom right `(x, y)` coordinates, adding the appropriate sums as needed.
    ///
    /// This will modify `is_valid` as needed.
    ///
    /// # Parameters
    /// - `grid`: The power grid.
    pub fn add_one(&mut self, grid: &PowerGrid) -> () {
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