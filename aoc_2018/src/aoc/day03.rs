use std::collections::{HashMap};

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

// https://adventofcode.com/2018/day/3
// --- Day 3: No Matter How You Slice It ---
//
// The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to
// someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night).
// Unfortunately, anomalies are still affecting them - nobody can even agree on how to cut the
// fabric.
//
// The whole piece of fabric they're working on is a very large square - at least 1000 inches on
// each side.
//
// Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims
// have an ID and consist of a single rectangle with edges parallel to the edges of the fabric.
// Each claim's rectangle is defined as follows:
//
// The number of inches between the left edge of the fabric and the left edge of the rectangle.
// The number of inches between the top edge of the fabric and the top edge of the rectangle.
// The width of the rectangle in inches.
// The height of the rectangle in inches.
// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the
// left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims
// the square inches of fabric represented by # (and ignores the square inches of fabric
// represented by .) in the diagram below:
//
//  ...........
//  ...........
//  ...#####...
//  ...#####...
//  ...#####...
//  ...#####...
//  ...........
//  ...........
//  ...........
// The problem is that many of the claims overlap, causing two or more claims to cover part of the
// same areas. For example, consider the following claims:
//
// #1 @ 1,3: 4x4
// #2 @ 3,1: 4x4
// #3 @ 5,5: 2x2
//
// Visually, these claim the following areas:
//
//  ........
//  ...2222.
//  ...2222.
//  .11XX22.
//  .11XX22.
//  .111133.
//  .111133.
//  ........
// The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to
// the others, does not overlap either of them.)
//
// If the Elves all proceed with their own plans, none of them will have enough fabric. How many
// square inches of fabric are within two or more claims?

pub fn part1(map: &HashMap<(i32, i32), i32>) -> i32 {
    return map.values().map(|&x| if x == 1 { 0 } else { 1 }).sum();
}

// Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch
// of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be
// able to make Santa's suit after all!
//
// For example, in the claims above, only claim 3 is intact after all claims are made.
//
// What is the ID of the only claim that doesn't overlap?

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