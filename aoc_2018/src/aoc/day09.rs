use std::collections::VecDeque;

// https://adventofcode.com/2018/day/9
#[allow(dead_code)]
pub fn execute(input: &[String]) -> (usize, usize) {
    let split_input: Vec<_> = input[0]
        .split(" players; last marble is worth ")
        .map(|x| x.replace(" points", ""))
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let num_players = split_input[0];
    let last_marble = split_input[1];
    (
        part1(num_players, last_marble),
        part2(num_players, last_marble * 100),
    )
}

pub fn part1(num_players: usize, last_marble: usize) -> usize {
    let mut marbles: Vec<usize> = vec![0, 1];
    let mut players: Vec<usize> = vec![0; num_players];

    // zero-indexed
    let mut cur_marble_idx: i32 = 1;
    let mut cur_player: usize = 1;
    // one-indexed
    let mut cur_marble: usize = 2;

    while cur_marble != last_marble + 1 {
        cur_player %= num_players;

        if cur_marble % 23 == 0 {
            cur_marble_idx -= 7;
            if cur_marble_idx < 0 {
                cur_marble_idx = (marbles.len() as i32) - cur_marble_idx.abs();
            }

            players[cur_player] +=
                marbles.remove(if cur_marble_idx == (marbles.len() - 1) as i32 {
                    0
                } else {
                    cur_marble_idx as usize
                }) + cur_marble;

            cur_player += 1;
            cur_marble += 1;
            continue;
        }

        cur_marble_idx =
            add_to_marble_arr(&mut marbles, cur_marble_idx as usize, cur_marble) as i32;
        cur_marble += 1;
        cur_player += 1;
    }

    *players.iter().max().unwrap()
}

pub fn part2(num_players: usize, last_marble: usize) -> usize {
    let mut circle: VecDeque<usize> = VecDeque::with_capacity(last_marble);
    circle.push_back(0);
    let mut player_scores = vec![0; num_players];

    for i in 1..=last_marble {
        if i % 23 == 0 {
            player_scores[i % num_players] += i;
            // Rotate 7 counterclockwise
            for _ in 0..7 {
                let t = circle.pop_back().expect("pop back error");
                circle.push_front(t);
            }

            // Remove the current marble (the "front")
            player_scores[i % num_players] += circle.pop_front().expect("pop front error");
        } else {
            // Rotate 2 clockwise
            for _ in 0..2 {
                let t = circle.pop_front().expect("pop back error");
                circle.push_back(t);
            }

            // Then add the new marble (the current one)
            circle.push_front(i);
        }
    }

    *player_scores.iter().max().unwrap()
}

/// Adds the marble to the given vector and returns the new index. Decided to make this function
/// since I was tired and didn't want to figure out how to deal with vectors and wrapping around.
///
/// # Parameters
/// * `marbles` - The vector containing all marbles.
/// * `idx` - The index of the current marble.
/// * `num` - The marble number to add.
///
/// # Returns
/// The new index.
fn add_to_marble_arr(marbles: &mut Vec<usize>, idx: usize, num: usize) -> usize {
    if idx + 2 < marbles.len() {
        marbles.insert(idx + 2, num);
        return idx + 2;
    }

    if marbles.len() >= 2 && idx == marbles.len() - 2 {
        marbles.push(num);
        return marbles.len() - 1;
    }

    marbles.insert(1, num);
    1
}
