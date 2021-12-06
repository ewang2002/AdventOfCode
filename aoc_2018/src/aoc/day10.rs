use std::collections::HashSet;

type I64Pair = (i64, i64);

// https://adventofcode.com/2018/day/10
#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (String, u64) {
    let mut points: Vec<Point> = vec![];

    input.iter().map(|x| {
        return x.replace("position=<", "")
            .replace("> velocity=<", ", ")
            .replace(">", "");
    }).map(|x| {
        return x.split(",")
            .collect::<Vec<_>>()
            .iter()
            .map(|y| y.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
    }).for_each(|x| {
        if x.len() != 4 {
            panic!("invalid input.");
        }

        points.push(Point::new(x[0], x[1], x[2], x[3]));
    });

    // For the sake of consistency, pass this as reference so we can print it out for part 2
    let mut minimum_seconds: u64 = 0;
    return (part1(points, &mut minimum_seconds), part2(minimum_seconds));
}

pub fn part1(mut points: Vec<Point>, minimum_seconds: &mut u64) -> String {
    let mut min_sec: u64 = 0;
    let mut min_group: u32 = calculate_group_count(&points);
    // It should be noted that the group count forms some sort of a quadratic relationship. We
    // start with a large number of groups (since the points are all scattered), but eventually the
    // points will be in a position such that the number of groups will be at a minimum. After this
    // point passes, the number of groups will rapidly increase.

    let mut increased_amt = 0;
    for i in 1..50_000 {
        apply_to_all_pts(&mut points, |x| x.increment_second());
        let group_count = calculate_group_count(&points);

        // If we're increasing too many times, then we assume that we've hit the minimum point
        // and can thus stop
        if increased_amt > 20 {
            break;
        }

        if group_count > min_group {
            increased_amt += 1;
            continue;
        }


        if group_count < min_group {
            increased_amt = 0;
            min_group = group_count;
            min_sec = i;
        }
    }

    println!("Minimum Seconds: {} -> Groups: {}", min_sec, min_group);
    apply_to_all_pts(&mut points, |x| {
        x.reset();
        x.increment_by(min_sec as i64);
    });

    let mut all_points: HashSet<(i64, i64)> = HashSet::new();
    for pt in &points {
        all_points.insert((pt.curr_pos_x, pt.curr_pos_y));
    }

    // Find bounding box
    let min_x = points.iter()
        .min_by_key(|p| p.curr_pos_x)
        .expect("invalid min x found")
        .curr_pos_x;
    let max_x = points.iter()
        .max_by_key(|p| p.curr_pos_x)
        .expect("invalid max x found")
        .curr_pos_x;
    let min_y = points.iter()
        .min_by_key(|p| p.curr_pos_y)
        .expect("invalid min y found")
        .curr_pos_y;
    let max_y = points.iter()
        .max_by_key(|p| p.curr_pos_y)
        .expect("invalid max y found")
        .curr_pos_y;

    // Create output string with the message
    let mut str = String::new();
    str.push('\n');
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            str.push(if all_points.contains(&(x, y)) { '#' } else { '.' });
        }
        str.push('\n');
    }

    *minimum_seconds = min_sec;
    return str;
}

pub fn part2(min_sec: u64) -> u64 {
    return min_sec;
}

/// Applies a given function to all `Point` structures in the vector.
///
/// # Parameters
/// - `pts`: A reference to a vector of `Point` structures.
/// - `func`: The function to apply to all points.
///
/// # Notes
/// - Should we be using `Fn` or `FnMut` here?
fn apply_to_all_pts<F>(pts: &mut Vec<Point>, func: F) -> ()
    where F: Fn(&mut Point) -> () {
    pts.into_iter().for_each(|x| func(x));
}

/// Calculates the number of "grouped" points. Two points are grouped if they are direct neighbors
/// with each other (i.e. point A is point B's direct neighbor if and only if A is directly above,
/// below, left, or right of B).
///
/// # Parameters
/// - `pts`: A reference to a vector of `Point` structures.
///
/// # Returns
/// - The number of groups.
fn calculate_group_count(pts: &Vec<Point>) -> u32 {
    let mut explored_points: HashSet<I64Pair> = HashSet::new();
    let mut all_points: HashSet<I64Pair> = HashSet::new();
    for point_info in pts {
        all_points.insert((point_info.curr_pos_x, point_info.curr_pos_y));
    }

    let mut groups: u32 = 0;

    for point in &all_points {
        if explored_points.contains(&point) {
            continue;
        }

        explore_neighbors(point, &all_points, &mut explored_points);
        groups += 1;
    }

    return groups;
}

/// Recursively explores all neighbors (definition of neighbor is provided above) of a point.
///
/// # Parameters
/// - `current_point`: The current point.
/// - `all_points`: All points to check.
/// - `explored_points`: All points that are explored.
///
/// # Notes
/// - As this is a recursive function, this can be optimized.
fn explore_neighbors(current_point: &I64Pair, all_points: &HashSet<I64Pair>,
                     explored_points: &mut HashSet<I64Pair>) -> () {
    if explored_points.contains(&current_point) {
        return;
    }

    if !all_points.contains(&current_point) {
        return;
    }

    explored_points.insert(*current_point);

    let (x, y) = current_point;
    let deref_x = *x;
    let deref_y = *y;
    explore_neighbors(&(deref_x + 1, deref_y), all_points, explored_points);
    explore_neighbors(&(deref_x, deref_y + 1), all_points, explored_points);
    explore_neighbors(&(deref_x - 1, deref_y), all_points, explored_points);
    explore_neighbors(&(deref_x, deref_y - 1), all_points, explored_points);
}

#[derive(Clone)]
pub struct Point {
    velocity: I64Pair,
    initial_pos: I64Pair,
    curr_pos_x: i64,
    curr_pos_y: i64,
}

impl Point {
    /// Creates a new `Point` structure with the specified initial position and velocities.
    ///
    /// # Parameters
    /// - `pos_x`: The initial `x`-position.
    /// - `pos_y`: The initial `y`-position.
    /// - `vel_x`: The velocity in the `x`-direction.
    /// - `vel_y`: The velocity in the `y`-direction.
    ///
    /// # Returns
    /// - The new `Point`.
    pub fn new(pos_x: i64, pos_y: i64, vel_x: i64, vel_y: i64) -> Self {
        return Point {
            velocity: (vel_x, vel_y),
            initial_pos: (pos_x, pos_y),
            curr_pos_x: pos_x,
            curr_pos_y: pos_y,
        };
    }

    /// Moves the `Point` coordinates by the velocity one time.
    pub fn increment_second(&mut self) -> () {
        self.curr_pos_x += self.velocity.0;
        self.curr_pos_y += self.velocity.1;
    }

    /// Resets the `Point` to the initial position.
    pub fn reset(&mut self) -> () {
        self.curr_pos_x = self.initial_pos.0;
        self.curr_pos_y = self.initial_pos.1;
    }

    /// Increments the `Point` some number of times.
    ///
    /// # Parameters
    /// - `sec`: The number of seconds that will have passed.
    pub fn increment_by(&mut self, sec: i64) -> () {
        self.curr_pos_x += self.velocity.0 * sec;
        self.curr_pos_y += self.velocity.1 * sec;
    }
}