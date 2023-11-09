use common::day::{AoCProblem, Solution};
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct Day21 {
    p1_starting: usize,
    p2_starting: usize,
}

// https://adventofcode.com/2021/day/21
impl AoCProblem for Day21 {
    fn prepare(input: String) -> Self {
        assert_eq!(2, input.len());
        Self {
            p1_starting: input
                .lines()
                .nth(0)
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
            p2_starting: input
                .lines()
                .nth(1)
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut player_position: [usize; 2] = [self.p1_starting, self.p2_starting];
        let mut player_scores: [usize; 2] = [0, 0];

        let mut p: usize = 0;
        let mut die_val = 1;
        let mut rolled = 0;
        loop {
            let mut n = 0;
            for _ in 0..3 {
                n += die_val;
                die_val = (die_val % 10) + 1;
            }

            rolled += 3;

            player_position[p] = ((player_position[p] - 1 + n) % 10) + 1;
            player_scores[p] += player_position[p];
            if player_scores[p] >= 1000 {
                break;
            }

            p = (p + 1) % 2;
        }

        (min(player_scores[0], player_scores[1]) * rolled).into()
    }

    fn part2(&mut self) -> Solution {
        let r = play_dirac(
            &mut HashMap::new(),
            self.p1_starting,
            self.p2_starting,
            0,
            0,
            true,
        );
        max(r.0, r.1).into()
    }
}

// Key = game state,
// Value = number of times player 1, player 2 won that universe corresponding to that game state.
type Cache = HashMap<(usize, usize, usize, usize, bool), (usize, usize)>;

/// Plays a game using the Dirac Die.
///
/// # Parameters
/// - `cache`: The cache.
/// - `p1_p`: Player 1's position.
/// - `p2_p`: Player 2's position.
/// - `p1_s`: Player 1's score.
/// - `p2_s`: Player 2's score.
/// - `p1_turn`: Whether it is player 1's turn.
///
/// # Returns
/// The number of universes that both player 1 (first element in the tuple) and 2 won (second
/// element of the tuple) won.
fn play_dirac(
    cache: &mut Cache,
    p1_p: usize,
    p2_p: usize,
    p1_s: usize,
    p2_s: usize,
    p1_turn: bool,
) -> (usize, usize) {
    let key = (p1_p, p2_p, p1_s, p2_s, p1_turn);
    let val = cache.get(&key);
    if let Some(v) = val {
        return *v;
    }

    // Player 1 won this universe.
    if p1_s >= 21 {
        cache.insert(key, (1, 0));
        return (1, 0);
    }

    // Player 2 won this universe.
    if p2_s >= 21 {
        cache.insert(key, (0, 1));
        return (0, 1);
    }

    let mut result: (usize, usize) = (0, 0);
    // We could have also done:
    // for d1 in 1..=3 {
    //      for d2 in 1..=3 {
    //          for d3 in 1..=3 {
    //              ...
    // To simulate each die roll. Then, we can add the result (d1 + d2 + d3) and that will be used
    // in the calculation of the new position for the new game state.
    for roll in 3..=9 {
        // These represent all possible rolls we can get (since we roll a die of 1, 2, 3) three
        // times.
        let freq_of_roll = match roll {
            // 1 + 1 + 1
            3 => 1,
            // 1 + 1 + 2, 1 + 2 + 1, 2 + 1 + 1
            4 => 3,
            // 1 + 2 + 2, 2 + 1 + 2, 2 + 2 + 1
            // 1 + 1 + 3, 1 + 3 + 1, 3 + 1 + 1
            5 => 6,
            // 1 + 2 + 3, ... [3!]
            // 2 + 2 + 2
            6 => 7,
            // 2 + 2 + 3, ... [3!/2]
            // 1 + 3 + 3, ... [3!/2]
            7 => 6,
            // 3 + 3 + 2, ... [3!/2]
            8 => 3,
            // 3 + 3 + 3
            9 => 1,
            _ => panic!("unknown combination {}", roll),
        };

        let mut new_p1_p = p1_p;
        let mut new_p2_p = p2_p;
        let mut new_p1_s = p1_s;
        let mut new_p2_s = p2_s;

        if p1_turn {
            new_p1_p = ((p1_p - 1 + roll) % 10) + 1;
            new_p1_s = p1_s + new_p1_p;
        } else {
            new_p2_p = ((p2_p - 1 + roll) % 10) + 1;
            new_p2_s = p2_s + new_p2_p;
        }

        let (p1_w, p2_w) = play_dirac(cache, new_p1_p, new_p2_p, new_p1_s, new_p2_s, !p1_turn);
        result.0 += p1_w * freq_of_roll;
        result.1 += p2_w * freq_of_roll;
    }

    cache.insert(key, result);
    result
}
