use std::collections::{VecDeque};

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (usize, usize) {
    let split_input: Vec<_> = input[0].split(" players; last marble is worth ")
        .map(|x| x.replace(" points", ""))
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let num_players = split_input[0];
    let last_marble = split_input[1];
    return (part1(num_players, last_marble), part2(num_players, last_marble * 100));
}

// --- Day 9: Marble Mania ---
// You talk to the Elves while you wait for your navigation system to initialize. To pass the time,
// they introduce you to their favorite marble game.
//
// The Elves play this game by taking turns arranging the marbles in a circle according to very
// particular rules. The marbles are numbered starting with 0 and increasing by 1 until every
// marble has a number.
//
// First, the marble numbered 0 is placed in the circle. At this point, while it contains only a
// single marble, it is still a circle: the marble is both clockwise from itself and
// counter-clockwise from itself. This marble is designated the current marble.
//
// Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle between
// the marbles that are 1 and 2 marbles clockwise of the current marble. (When the circle is large
// enough, this means that there is one marble between the marble that was just placed and the
// current marble.) The marble that was just placed then becomes the current marble.
//
// However, if the marble that is about to be placed has a number which is a multiple of 23,
// something entirely different happens. First, the current player keeps the marble they would
// have placed, adding it to their score. In addition, the marble 7 marbles counter-clockwise
// from the current marble is removed from the circle and also added to the current player's score.
// The marble located immediately clockwise of the marble that was removed becomes the new current
// marble.
//
// For example, suppose there are 9 players. After the marble with value 0 is placed in the middle,
// each player (shown in square brackets) takes a turn. The result of each of those turns would
// produce circles of marbles like this, where clockwise is to the right and the resulting current
// marble is in parentheses:
//
//  [-] (0)
//  [1]  0 (1)
//  [2]  0 (2) 1
//  [3]  0  2  1 (3)
//  [4]  0 (4) 2  1  3
//  [5]  0  4  2 (5) 1  3
//  [6]  0  4  2  5  1 (6) 3
//  [7]  0  4  2  5  1  6  3 (7)
//  [8]  0 (8) 4  2  5  1  6  3  7
//  [9]  0  8  4 (9) 2  5  1  6  3  7
//  [1]  0  8  4  9  2(10) 5  1  6  3  7
//  [2]  0  8  4  9  2 10  5(11) 1  6  3  7
//  [3]  0  8  4  9  2 10  5 11  1(12) 6  3  7
//  [4]  0  8  4  9  2 10  5 11  1 12  6(13) 3  7
//  [5]  0  8  4  9  2 10  5 11  1 12  6 13  3(14) 7
//  [6]  0  8  4  9  2 10  5 11  1 12  6 13  3 14  7(15)
//  [7]  0(16) 8  4  9  2 10  5 11  1 12  6 13  3 14  7 15
//  [8]  0 16  8(17) 4  9  2 10  5 11  1 12  6 13  3 14  7 15
//  [9]  0 16  8 17  4(18) 9  2 10  5 11  1 12  6 13  3 14  7 15
//  [1]  0 16  8 17  4 18  9(19) 2 10  5 11  1 12  6 13  3 14  7 15
//  [2]  0 16  8 17  4 18  9 19  2(20)10  5 11  1 12  6 13  3 14  7 15
//  [3]  0 16  8 17  4 18  9 19  2 20 10(21) 5 11  1 12  6 13  3 14  7 15
//  [4]  0 16  8 17  4 18  9 19  2 20 10 21  5(22)11  1 12  6 13  3 14  7 15
//  [5]  0 16  8 17  4 18(19) 2 20 10 21  5 22 11  1 12  6 13  3 14  7 15
//  [6]  0 16  8 17  4 18 19  2(24)20 10 21  5 22 11  1 12  6 13  3 14  7 15
//  [7]  0 16  8 17  4 18 19  2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
//
// The goal is to be the player with the highest score after the last marble is used up. Assuming
// the example above ends after the marble numbered 25, the winning score is 23+9=32 (because
// player 5 kept marble 23 and removed marble 9, while no other player got any points in this
// very short example game).
//
// Here are a few more examples:
//
// - 10 players; last marble is worth 1618 points: high score is 8317
// - 13 players; last marble is worth 7999 points: high score is 146373
// - 17 players; last marble is worth 1104 points: high score is 2764
// - 21 players; last marble is worth 6111 points: high score is 54718
// - 30 players; last marble is worth 5807 points: high score is 37305
//
// What is the winning Elf's score?

pub fn part1(num_players: usize, last_marble: usize) -> usize {
    let mut marbles: Vec<usize> = vec![0, 1];
    let mut players: Vec<usize> = vec![];
    for _ in 0..num_players {
        players.push(0);
    }

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

            players[cur_player] += marbles.remove(
                if cur_marble_idx == (marbles.len() - 1) as i32 {
                    0
                } else {
                    cur_marble_idx as usize
                }
            ) + cur_marble;

            cur_player += 1;
            cur_marble += 1;
            continue;
        }

        cur_marble_idx = add_to_marble_arr(
            &mut marbles,
            cur_marble_idx as usize,
            cur_marble,
        ) as i32;
        cur_marble += 1;
        cur_player += 1;
    }

    return *players.iter().max().unwrap();
}

// --- Part Two ---
// Amused by the speed of your answer, the Elves are curious:
//
// What would the new winning Elf's score be if the number of the last marble were 100 times larger?
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

    return *player_scores.iter().max().unwrap();
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
    return 1;
}