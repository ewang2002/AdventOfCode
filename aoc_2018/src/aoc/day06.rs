use std::collections::HashMap;

// https://adventofcode.com/2018/day/6
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
