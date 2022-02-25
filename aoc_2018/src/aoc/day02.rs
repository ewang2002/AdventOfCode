// https://adventofcode.com/2018/day/2
#[allow(dead_code)]
pub fn execute(input: &[String]) -> (i32, String) {
    (part1(input), part2(input))
}

pub fn part1(input: &[String]) -> i32 {
    let mut two = 0;
    let mut three = 0;
    for line in input {
        let mut two_temp = 0;
        let mut three_temp = 0;
        line.chars().for_each(|x| match line.matches(x).count() {
            2 => two_temp = 1,
            3 => three_temp = 1,
            _ => {}
        });

        two += two_temp;
        three += three_temp;
    }

    two * three
}

pub fn part2(input: &[String]) -> String {
    for a in input {
        for b in input {
            if let Some(s) = check_words(a, b) {
                return s;
            }
        }
    }

    String::new()
}

fn check_words(w1: &str, w2: &str) -> Option<String> {
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

    if mismatches == 1 {
        Option::Some(res_word)
    } else {
        Option::None
    }
}

// Old AoC code
// pub fn part1(input: &[String]) -> i32 {
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
