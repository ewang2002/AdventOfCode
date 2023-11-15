use std::collections::{HashMap, HashSet};

use common::problem::day::{AoCProblem, Solution};

// Lol, did someone say... *brute-force?*
pub struct Day15 {
    sensors: Vec<Sensor>,
    raw_input: Vec<(isize, isize, isize, isize)>,
    all_sensor_locations: HashSet<(isize, isize)>,
    all_beacon_locations: HashSet<(isize, isize)>,
}

impl AoCProblem for Day15 {
    fn prepare(input: String) -> Self {
        let mut sensors = vec![];
        let mut raw_input = vec![];
        let mut all_beacon_locations = HashSet::new();
        let mut all_sensor_locations = HashSet::new();
        for line in input.lines() {
            let cleaned_str = line
                .replace("Sensor at ", "")
                .replace(" closest beacon is at ", "")
                .replace("x=", "")
                .replace("y=", "")
                .replace(' ', "");
            let (this_beacon, closest_beacon) = cleaned_str.split_once(':').unwrap();
            let (x, y) = this_beacon.split_once(',').unwrap();
            let (c_x, c_y) = closest_beacon.split_once(',').unwrap();
            let sensor_x = x.parse::<isize>().unwrap();
            let sensor_y = y.parse::<isize>().unwrap();
            let beacon_x = c_x.parse::<isize>().unwrap();
            let beacon_y = c_y.parse::<isize>().unwrap();

            sensors.push(Sensor::new(sensor_x, sensor_y, beacon_x, beacon_y));

            raw_input.push((sensor_x, sensor_y, beacon_x, beacon_y));

            all_sensor_locations.insert((sensor_x, sensor_y));
            all_beacon_locations.insert((beacon_x, beacon_y));
        }

        Self {
            sensors,
            raw_input,
            all_sensor_locations,
            all_beacon_locations,
        }
    }

    fn part1(&mut self) -> Solution {
        const Y: isize = 2000000;
        // First, find a good starting point.
        let mut dist_y = isize::MAX;
        let mut start_x = 0;
        for sensor in &self.sensors {
            if (sensor.closest_beacon_y - Y).abs() < dist_y {
                start_x = sensor.closest_beacon_x;
                dist_y = (sensor.closest_beacon_y - Y).abs();
            }
        }

        // Once we have a good starting point, we can figure out how many points
        // are not beacons
        //
        // Start from the left side.
        let mut ct = 0;
        let mut x = start_x - 1;
        loop {
            if self.sensors.iter().any(|b| b.is_in_sensor_range((x, Y))) {
                ct += !self.all_beacon_locations.contains(&(x, Y)) as isize;
                x -= 1;
                continue;
            }

            break;
        }

        x = start_x + 1;
        loop {
            if self.sensors.iter().any(|b| b.is_in_sensor_range((x, Y))) {
                ct += !self.all_beacon_locations.contains(&(x, Y)) as isize;
                x += 1;
                continue;
            }

            break;
        }

        ct += if self
            .sensors
            .iter()
            .any(|b| b.is_in_sensor_range((start_x, Y)))
        {
            !self.all_beacon_locations.contains(&(start_x, Y)) as isize
        } else {
            0
        };

        ct.into()
    }

    fn part2(&mut self) -> Solution {
        let mut dict: HashMap<(isize, isize), usize> = HashMap::new();
        const DIFF: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
        const MAX_XY: isize = 4_000_000;

        for sensor in &self.sensors {
            let mut x = sensor.x + sensor.manhattan_distance + 1;
            let mut y = sensor.y;
            for (dx, dy) in DIFF {
                let mut remaining = sensor.manhattan_distance + 1;
                while remaining > 0 {
                    if (0..=MAX_XY).contains(&x) && (0..=MAX_XY).contains(&y) {
                        *dict.entry((x, y)).or_insert(0) += 1;
                    }
                    x += dx;
                    y += dy;
                    remaining -= 1;
                }

                if (0..=MAX_XY).contains(&x) && (0..=MAX_XY).contains(&y) {
                    *dict.entry((x, y)).or_insert(0) += 1;
                }
            }
        }

        let high_pts = dict
            .into_iter()
            .filter(|(_, v)| *v >= 4)
            .collect::<Vec<_>>();

        for (pt, _) in &high_pts {
            if self.sensors.iter().any(|x| x.is_in_sensor_range(*pt)) {
                continue;
            }

            return (pt.0 * 4000000 + pt.1).into();
        }

        0.into()
    }

    fn day() -> u32 {
        15
    }
}

#[derive(Copy, Clone)]
struct Sensor {
    x: isize,
    y: isize,
    closest_beacon_x: isize,
    closest_beacon_y: isize,
    manhattan_distance: isize,
}

impl Sensor {
    pub fn new(x: isize, y: isize, beacon_x: isize, beacon_y: isize) -> Self {
        let dist = (x - beacon_x).abs() + (y - beacon_y).abs();
        Sensor {
            x,
            y,
            closest_beacon_x: beacon_x,
            closest_beacon_y: beacon_y,
            manhattan_distance: dist,
        }
    }

    pub fn is_in_sensor_range(&self, beacon: impl Into<(isize, isize)>) -> bool {
        let (other_x, other_y) = beacon.into();
        (self.x - other_x).abs() + (self.y - other_y).abs() <= self.manhattan_distance
    }
}

impl From<&Sensor> for (isize, isize) {
    fn from(b: &Sensor) -> Self {
        (b.closest_beacon_x, b.closest_beacon_y)
    }
}
