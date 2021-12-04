use std::collections::HashMap;

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (i32, i32) {
    // Safe to unwrap parsed nums since we're not dealing with any invalid input.
    let points: Vec<(i32, i32)> = input
        .iter()
        .map(|x| x.split(", ").collect::<Vec<&str>>())
        .map(|y| (y[0].parse::<i32>().unwrap(), y[1].parse::<i32>().unwrap()))
        .collect();

    // Find the top-left (x, y) and bottom-right (x, y) points. Essentially, we're going to
    // restrict ourselves to a "box" where we can test each point individually.
    let mut tl_x: i32 = -1;
    let mut tl_y: i32 = -1;
    let mut br_x: i32 = -1;
    let mut br_y: i32 = -1;

    for (x, y) in &points {
        if x > &br_x || br_x == -1 { br_x = *x; }
        if &tl_x > x || tl_x == -1 { tl_x = *x; }
        if y > &br_y || br_y == -1 { br_y = *y; }
        if &tl_y > y || tl_y == -1 { tl_y = *y; }
    }

    // (x, y, x, y)
    let box_bounds = (tl_x, tl_y, br_x, br_y);
    return (part1(&points, box_bounds), part2(&points, box_bounds));
}

// --- Day 6: Chronal Coordinates ---
// The device on your wrist beeps several times, and once again you feel like you're falling.
//
// "Situation critical," the device announces. "Destination indeterminate. Chronal interference
// detected. Please specify new target coordinates."
//
// The device then produces a list of coordinates (your puzzle input). Are they places it thinks
// are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a
// manual.
//
// If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the
// largest distance from the other points.
//
// Using only the Manhattan distance, determine the area around each coordinate by counting the
// number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance
// to any other coordinate).
//
// Your goal is to find the size of the largest area that isn't infinite. For example, consider the
// following list of coordinates:
//
//  1, 1
//  1, 6
//  8, 3
//  3, 4
//  5, 5
//  8, 9
//
// If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top
// left:
//
//  ..........
//  .A........
//  ..........
//  ........C.
//  ...D......
//  .....E....
//  .B........
//  ..........
//  ..........
//  ........F.
// This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan
// distance, each location's closest coordinate can be determined, shown here in lowercase:
//
//  aaaaa.cccc
//  aAaaa.cccc
//  aaaddecccc
//  aadddeccCc
//  ..dDdeeccc
//  bb.deEeecc
//  bBb.eeee..
//  bbb.eeefff
//  bbb.eeffff
//  bbb.ffffFf
//
// Locations shown as . are equally far from two or more coordinates, and so they don't count as
// being closest to any.
//
// In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here,
// their areas extend forever outside the visible grid. However, the areas of coordinates D and E
// are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's
// location itself). Therefore, in this example, the size of the largest area is 17.
//
// What is the size of the largest area that isn't infinite?

pub fn part1(points: &Vec<(i32, i32)>, bounds: (i32, i32, i32, i32)) -> i32 {
    let (tl_x, tl_y, br_x, br_y) = bounds;
    // Step 1: Populate hashmap of all points to check.
    let mut map: HashMap<(i32, i32), PointInfo> = HashMap::new();
    for pt in points {
        map.insert(*pt, PointInfo { num_points: 0, is_valid: true });
    }

    // Step 2: Test the very outer layer. Whatever points is the closest to the point in the outer
    // layer will be marked invalid.

    // 2.1: Test left + right side.
    for x in (tl_x - 1)..=(br_x + 1) {
        let (pt1, _) = get_nearest_point(points, (x, tl_y - 1));
        map.get_mut(&pt1).unwrap().is_valid = false;
        let (pt2, _) = get_nearest_point(points, (x, br_y + 1));
        map.get_mut(&pt2).unwrap().is_valid = false;
    }

    // 2.2: Test top + bottom side.
    for y in (tl_y - 1)..=(br_y + 1) {
        let (pt1, _) = get_nearest_point(points, (tl_x - 1, y));
        map.get_mut(&pt1).unwrap().is_valid = false;
        let (pt2, _) = get_nearest_point(points, (br_x + 1, y));
        map.get_mut(&pt2).unwrap().is_valid = false;
    }

    // Step 3: Now test every point in between.
    for x in tl_x..=br_x {
        for y in tl_y..=br_y {
            let (point, unique) = get_nearest_point(points, (x, y));
            if !unique {
                continue;
            }
            map.get_mut(&point).unwrap().num_points += 1;
        }
    }

    // Get the highest value in the hashmap.
    return map.iter()
        .filter(|&x| x.1.is_valid)
        .max_by(|a, b| a.1.num_points.cmp(&b.1.num_points))
        .map(|(_k, v)| v.num_points).unwrap();
}

// --- Part Two ---
// On the other hand, if the coordinates are safe, maybe the best you can do is try to find a
// region near as many coordinates as possible.
//
// For example, suppose you want the sum of the Manhattan distance to all of the coordinates to be
// less than 32. For each location, add up the distances to all of the given coordinates; if the
// total of those distances is less than 32, that location is within the desired region. Using the
// same coordinates as above, the resulting region looks like this:
//
//  ..........
//  .A........
//  ..........
//  ...###..C.
//  ..#D###...
//  ..###E#...
//  .B.###....
//  ..........
//  ..........
//  ........F.
//
// In particular, consider the highlighted location 4,3 located at the top middle of the region.
// Its calculation is as follows, where abs() is the absolute value function:
//
// - Distance to coordinate A: abs(4-1) + abs(3-1) =  5
// - Distance to coordinate B: abs(4-1) + abs(3-6) =  6
// - Distance to coordinate C: abs(4-8) + abs(3-3) =  4
// - Distance to coordinate D: abs(4-3) + abs(3-4) =  2
// - Distance to coordinate E: abs(4-5) + abs(3-5) =  3
// - Distance to coordinate F: abs(4-8) + abs(3-9) = 10
// - Total distance: 5 + 6 + 4 + 2 + 3 + 10 = 30
//
// Because the total distance to all coordinates (30) is less than 32, the location is within the
// region.
//
// This region, which also includes coordinates D and E, has a total size of 16.
//
// Your actual region will need to be much larger than this example, though, instead including all
// locations with a total distance of less than 10000.
//
// What is the size of the region containing all locations which have a total distance to all given
// coordinates of less than 10000?

pub fn part2(points: &Vec<(i32, i32)>, bounds: (i32, i32, i32, i32)) -> i32 {
    let (tl_x, tl_y, br_x, br_y) = bounds;
    let mut region_size = 0;

    for x in tl_x..=br_x {
        for y in tl_y..=br_y {
            let dist_sum: i32 = points
                .iter()
                .map(|pt| manhattan_distance(*pt, (x, y)))
                .sum();
            if dist_sum < 10000 {
                region_size += 1;
            }
        }
    }

    return region_size;
}



/// Given a vector of points (denoted `pts`) and a test point, find a point in `pts` that is the
/// closest to the test point by Manhattan distance.
///
/// # Arguments
/// * `pts` - The points.
/// * `test_pt` - The target point.
///
/// # Returns
/// A typle. The first element in the tuple is a point in `pts` that is the closest to the given
/// point. The second element is a boolean value that tells you if that is the unique point.
fn get_nearest_point(pts: &Vec<(i32, i32)>, test_pt: (i32, i32)) -> ((i32, i32), bool) {
    let mut unique = true;

    let mut min_dist = 1_000_000;
    let mut min_point = (0, 0);
    for pt in pts {
        let dist = manhattan_distance(*pt, test_pt);
        if min_dist > dist {
            min_point = *pt;
            min_dist = dist;
            unique = true;
            continue;
        }

        // This means that we've found another point that has the same distance.
        if min_dist == dist {
            unique = false;
        }
    }

    return (min_point, unique);
}

/// Returns the Manhattan Distance between two points.
///
/// # Arguments
/// * `pt1` - The first point.
/// * `pt2` - The second point.
///
/// # Returns
/// The Manhattan Distance.
fn manhattan_distance(pt1: (i32, i32), pt2: (i32, i32)) -> i32 {
    return (pt1.0 - pt2.0).abs() + (pt1.1 - pt2.1).abs();
}

struct PointInfo {
    num_points: i32,
    is_valid: bool,
}
