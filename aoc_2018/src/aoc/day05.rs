// https://adventofcode.com/2018/day/5 
#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (usize, usize) {
    let line = &input[0];
    return (part1(line), part2(line));
}

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