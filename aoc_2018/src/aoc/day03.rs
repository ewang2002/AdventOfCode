use std::collections::{HashMap};

// https://adventofcode.com/2018/day/3
#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (i32, i32) {
    // Get all claims from the input vector
    let claims: Vec<Claim> = get_claims_from_input(input);
    // We're going to map each possible point to the number of times we've seen that point.
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    // Iterate through every claim.
    for claim in &claims {
        // And add each point to the map (or add one to number of times we've seen it).
        for pair in claim.get_all_points() {
            *map.entry(pair).or_default() += 1;
        }
    }

    return (part1(&map), part2(&map, &claims));
}

pub fn part1(map: &HashMap<(i32, i32), i32>) -> i32 {
    return map.values().map(|&x| if x == 1 { 0 } else { 1 }).sum();
}

pub fn part2(map: &HashMap<(i32, i32), i32>, claims: &Vec<Claim>) -> i32 {
    for claim in claims {
        // Same idea as part 1. Here, we're checking to make sure ALL points in this claim
        // were seen only once.
        if claim.get_all_points().iter().all(|p| map[&p] == 1) {
            return claim.claim_id;
        }
    }

    return -1;
}

fn get_claims_from_input(input: &Vec<String>) -> Vec<Claim> {
    return input.iter().map(|x| {
        let fixed_str = x
            .replace("#", "")
            .replace(" @ ", " ")
            .replace( ":", "")
            .replace("x", " ")
            .replace(",", " ");

        let arr: Vec<&str> = fixed_str.split(" ").collect();
        return Claim {
            claim_id: arr[0].parse().unwrap(),
            left_edge: arr[1].parse().unwrap(),
            top_edge: arr[2].parse().unwrap(),
            width: arr[3].parse().unwrap(),
            height: arr[4].parse().unwrap()
        };
    }).collect();
}

pub struct Claim {
    claim_id: i32,
    left_edge: i32,
    top_edge: i32,
    width: i32,
    height: i32
}

impl Claim {
    pub fn get_all_points(&self) -> Vec<(i32, i32)> {
        let mut return_vec: Vec<(i32, i32)> = Vec::new();
        for i in self.left_edge..(self.left_edge + self.width) {
            for j in self.top_edge..(self.top_edge + self.height) {
                return_vec.push((i, j));
            }
        }

        return return_vec;
    }
}


// if map.contains_key(&pair) {
//      *map.get_mut(&pair).unwrap() += 1;
//      continue;
// }
// map.insert(pair, 1);