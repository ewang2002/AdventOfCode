#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (i32, String) {
    return (part1(input), part2(input));
}

// https://adventofcode.com/2018/day/2
// --- Day 2: Inventory Management System ---
//
// You stop falling through time, catch your breath, and check the screen on the device.
// "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10."
// You made it! Now, to find those anomalies.
//
// Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now
// that so many people have chimneys, maybe he could sneak in that way?" Another voice responds,
// "Actually, we've been working on a new kind of suit that would let him fit through tight spaces
// like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans,
// everything! Nobody on the team can even seem to remember important details of the project!"
//
// "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored
// together, so the box IDs should be similar. Too bad it would take forever to search the warehouse
// for two similar box IDs..." They walk too far away to hear any more.
//
// Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if
// you were discovered - and use your fancy wrist device to quickly scan every box and produce a
// list of the likely candidates (your puzzle input).
//
// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number
// that have an ID containing exactly two of any letter and then separately counting those with
// exactly three of any letter. You can multiply those two counts together to get a rudimentary
// checksum and compare it to what your device predicts.
//
// For example, if you see the following box IDs:
//
//  abcdef contains no letters that appear exactly two or three times.
//  bababc contains two a and three b, so it counts for both.
//  abbcde contains two b, but no letter appears exactly three times.
//  abcccd contains three c, but no letter appears exactly two times.
//  aabcdd contains two a and two d, but it only counts once.
//  abcdee contains two e.
//  ababab contains three a and three b, but it only counts once.
//
// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them
// contain a letter which appears exactly three times. Multiplying these together produces a
// checksum of 4 * 3 = 12.
//
// What is the checksum for your list of box IDs?

pub fn part1(input: &Vec<String>) -> i32 {
    let mut two = 0;
    let mut three = 0;
    for line in input {
        let mut two_temp = 0;
        let mut three_temp = 0;
        line.chars().for_each(|x| {
            match line.matches(x).count() {
                2 => two_temp = 1,
                3 => three_temp = 1,
                _ => {}
            }
        });

        two += two_temp;
        three += three_temp;
    }

    return two * three;
}

// Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype
// fabric.
//
// The boxes will have IDs which differ by exactly one character at the same position in both
// strings. For example, given the following box IDs:
//
//  abcde
//  fghij
//  klmno
//  pqrst
//  fguij
//  axcye
//  wvxyz
//
// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).
// However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those
// must be the correct boxes.
//
// What letters are common between the two correct box IDs? (In the example above, this is found by
// removing the differing character from either ID, producing fgij.)

pub fn part2(input: &Vec<String>) -> String {
    for a in input {
        for b in input {
            if let Some(s) = check_words(&a, &b) {
                return s;
            }
        }
    }

    return String::new();
}

fn check_words(w1: &String, w2: &String) -> Option<String> {
    let mut res_word = String::new();
    let mut mismatches = 0;
    for (a, b) in w1.chars().zip(w2.chars()) {
        if mismatches > 1 {
            break;
        }

        if a != b {
            mismatches += 1;
            continue;
        }

        res_word.push(a);
    }

    return if mismatches == 1 { Option::Some(res_word) } else { Option::None };
}


// Old AoC code
// pub fn part1(input: &Vec<String>) -> i32 {
//     // Find all general number of repeat chars in case part 2 requires it.
//     let mut num_letters: HashMap<i32, i32> = HashMap::new();
//
//     for line in input {
//         let mut temp: HashMap<char, i32> = HashMap::new();
//
//         for c in line.chars() {
//             match temp.get(&c) {
//                 None => {
//                     temp.insert(c, 1);
//                 }
//                 Some(_) => {
//                     *temp.get_mut(&c).unwrap() += 1;
//                 }
//             }
//         }
//
//         let mut entries_set: HashSet<i32> = HashSet::new();
//         for (_k, v) in temp {
//             if entries_set.contains(&v) {
//                 continue;
//             }
//
//             if num_letters.contains_key(&v) {
//                 *num_letters.get_mut(&v).unwrap() += 1;
//             }
//             else {
//                 num_letters.insert(v, 1);
//             }
//
//             entries_set.insert(v);
//         }
//     }
//
//     return num_letters.get(&2).unwrap() * num_letters.get(&3).unwrap();
// }