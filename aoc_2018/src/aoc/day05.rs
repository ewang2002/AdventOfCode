#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (usize, usize) {
    let line = &input[0];
    return (part1(line), part2(line));
}

// https://adventofcode.com/2018/day/5
//
// --- Day 5: Alchemical Reduction ---
// You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent
// progress, but are still struggling with the suit's size reduction capabilities.
//
// While the very latest in 1518 alchemical technology might have solved their problem eventually,
// you can do better. You scan the chemical composition of the suit's material and discover that it
// is formed by extremely long polymers (one of which is available as your puzzle input).
//
// The polymer is formed by smaller units which, when triggered, react with each other such that
// two adjacent units of the same type and opposite polarity are destroyed. Units' types are
// represented by letters; units' polarity is represented by capitalization. For instance, r and R
// are units with the same type but opposite polarity, whereas r and s are entirely different types
// and do not react.
//
// For example:
//
//  In aA, a and A react, leaving nothing behind.
//  In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
//  In abAB, no two adjacent units are of the same type, and so nothing happens.
//  In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing
// happens.
// Now, consider a larger example, dabAcCaCBAcCcaDA:
//
//  dabAcCaCBAcCcaDA  The first 'cC' is removed.
//  dabAaCBAcCcaDA    This creates 'Aa', which is removed.
//  dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
//  dabCBAcaDA        No further actions can be taken.
//  After all possible reactions, the resulting polymer contains 10 units.
//
// How many units remain after fully reacting the polymer you scanned? (Note: in this puzzle and
// others, the input is large; if you copy/paste your input, make sure you get the whole thing.)

pub fn part1(line: &String) -> usize {
    // We aren't dealing with any weird characters.
    let mut char_arr: Vec<_> = line.as_bytes().to_vec();

    let mut idx = 0;
    // Simulating for (i = 0; i < char_arr.len() - 1; i += 1) {...}
    loop {
        if idx >= char_arr.len() - 1 {
            break;
        }

        if can_react(char_arr[idx], char_arr[idx + 1]) {
            // [..., a, b, X, a, A, x, ...]
            //                ^ (idx = n)
            char_arr.remove(idx);
            // [..., a, b, X, A, x, ...]
            //                ^ (idx = n)
            char_arr.remove(idx);
            // [..., a, b, X, x, ...]
            //                ^ (idx = n)
            idx -= if idx < 1 { 0 } else { 1 };
            // [..., a, b, X, x, ...]
            //             ^ (idx = n - 1)
            continue;
        }

        idx += 1;
    }

    return String::from_utf8(char_arr).expect("How did this hit?").len();
}

// --- Part Two ---
// Time to improve the polymer.
//
// One of the unit types is causing problems; it's preventing the polymer from collapsing as much
// as it should. Your goal is to figure out which unit type is causing the most problems, remove
// all instances of it (regardless of polarity), fully react the remaining polymer, and measure
// its length.
//
// For example, again using the polymer dabAcCaCBAcCcaDA from above:
//
//  Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which
// has length 6.
//  Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA,
// which has length 8.
//  Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which
// has length 4.
//  Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc,
// which has length 6.
// In this example, removing all C/c units was best, producing the answer 4.
//
// What is the length of the shortest polymer you can produce by removing all units of exactly one
// type and fully reacting the result?

pub fn part2(line: &String) -> usize {
    let all_important_letters = "abcdefghijklmnopqrstuvwxyz".split("");
    let mut len_of_shortest: usize = line.len();
    for letter in all_important_letters {
        let new_str = line
            .replace(&letter, "")
            .replace(&letter.to_uppercase(), "");
        let res = part1(&new_str);
        if len_of_shortest > res {
            len_of_shortest = res;
        }
    }

    return len_of_shortest;
}

fn can_react(a: u8, b: u8) -> bool {
    return if a < b { b - a == 32 } else { a - b == 32 };
}