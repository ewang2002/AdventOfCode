use std::cmp::min;

use common::{problem::day::{AoCProblem, Solution}, constants::TWO_NEWLINE};

type AlmanacEntry = [usize; 3];

pub struct Day05 {
    seeds: Vec<usize>,
    seed_to_soil: Vec<AlmanacEntry>,
    soil_to_fertilizer: Vec<AlmanacEntry>,
    fertilizer_to_water: Vec<AlmanacEntry>,
    water_to_light: Vec<AlmanacEntry>,
    light_to_temp: Vec<AlmanacEntry>,
    temp_to_humidity: Vec<AlmanacEntry>,
    humidity_to_loc: Vec<AlmanacEntry>,
}

impl AoCProblem for Day05 {
    fn prepare(input: String) -> Self {
        let split_input: Vec<&str> = input.split(TWO_NEWLINE).collect();
        // Parse seeds
        let (_, raw_seeds) = split_input[0].split_once(": ").unwrap();
        let seeds: Vec<usize> = raw_seeds.split_whitespace().map(|seed| seed.parse::<usize>().unwrap()).collect();

        let map_to_usize_vec = |s: &str| -> [usize; 3] {
            let mut iter = s.split(" ").map(|m| m.parse::<usize>().unwrap());
            [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()]
        };

        // Parse seed-to-soil
        let seed_to_soil: Vec<_> = split_input[1].lines().skip(1)
            .map(map_to_usize_vec)
            .collect();
            
        // Parse soil-to-fertilizer
        let soil_to_fertilizer: Vec<_> = split_input[2].lines().skip(1)
            .map(map_to_usize_vec)
            .collect();

        let fertilizer_to_water: Vec<_> = split_input[3].lines().skip(1)
            .map(map_to_usize_vec)
            .collect();

        let water_to_light: Vec<_> = split_input[4].lines().skip(1)
            .map(map_to_usize_vec)
            .collect();

        let light_to_temp: Vec<_> = split_input[5].lines().skip(1)
            .map(map_to_usize_vec)
            .collect();

        let temp_to_humidity: Vec<_> = split_input[6].lines().skip(1)
            .map(map_to_usize_vec)
            .collect();

        let humidity_to_loc: Vec<_> = split_input[7].lines().skip(1)
            .map(map_to_usize_vec)
            .collect();

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humidity,
            humidity_to_loc
        }
    }

    fn part1(&mut self) -> Solution {
        let mut lowest_location = usize::MAX;
        for seed in &self.seeds {
            let soil = almanac_mapper(&self.seed_to_soil, *seed);
            let fertilizer = almanac_mapper(&self.soil_to_fertilizer, soil);
            let water = almanac_mapper(&self.fertilizer_to_water, fertilizer);
            let light = almanac_mapper(&self.water_to_light, water);
            let temp = almanac_mapper(&self.light_to_temp, light);
            let humidity = almanac_mapper(&self.temp_to_humidity, temp);
            let location = almanac_mapper(&self.humidity_to_loc, humidity);
            lowest_location = min(lowest_location, location);
        }

        lowest_location.into()
    }

    fn part2(&mut self) -> Solution {
        // TODO
        0.into()
    }

    fn day() -> u32 {
        5
    }

    fn year() -> u32 {
        2023
    }
}

/// Maps an input to its output according to the list of almanac entries.
/// 
/// # Parameters
/// - `data`: The almanac.
/// - `input`: The input item.
/// 
/// # Returns
/// THe output based on the almanac.
fn almanac_mapper(data: &[AlmanacEntry], input: usize) -> usize {
    for [dest, src, range] in data {
        let lower_src = *src;
        let upper_src = *src + *range - 1;
        if lower_src <= input && input <= upper_src {
            return *dest + input - lower_src;
        }
    }

    // Any source numbers that aren't mapped correspond to the same destination number.
    input
}