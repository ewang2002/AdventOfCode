use std::collections::HashMap;

// https://adventofcode.com/2018/day/12
#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (i64, i64) {
    let initial_state = input[0].replace("initial state: ", "");
    let mut plant_mapping: HashMap<String, String> = HashMap::new();

    input.iter().skip(2).for_each(|line| {
        let plant_layout = line.split(" => ").collect::<Vec<_>>();
        plant_mapping.insert(plant_layout[0].to_string(), plant_layout[1].to_string());
    });

    return (
        part1(&initial_state, &plant_mapping),
        part2(&initial_state, &plant_mapping)
    );
}

pub fn part1(init_state: &String, plant_notes: &HashMap<String, String>) -> i64 {
    let mut curr_state = init_state.clone();
    // The plant that we're initially looking at
    let mut init_plant_idx = 0;
    for _ in 1..=20 {
        let gen_res = run_one_generation(curr_state, init_plant_idx, plant_notes);
        curr_state = gen_res.0;
        init_plant_idx = gen_res.1;
    }

    let mut sum = 0;
    let bytes = curr_state.bytes().collect::<Vec<_>>();
    for i in 0..bytes.len() {
        sum += if bytes[i] == b'#' { (i as i64) - init_plant_idx } else { 0 }
    }

    return sum;
}

pub fn part2(init_state: &String, plant_notes: &HashMap<String, String>) -> i64 {
    // From my observation, it appears that after a certain number of generation, the only change
    // between generations is that all plants (dead or alive) are shifted one to the right.
    //
    // Thus, the goal is to find the generation such that the next generation exhibits this
    // behavior, and then use this to our advantage.

    let mut curr_state = init_state.clone();
    let mut prev_state = String::from("#");
    // The plant that we're initially looking at
    let mut init_plant_idx: i64 = 0;
    let mut generation_count = 0;

    loop {
        generation_count += 1;
        let gen_res = run_one_generation(curr_state, init_plant_idx, plant_notes);
        curr_state = gen_res.0;
        init_plant_idx = gen_res.1;

        let curr_bytes = curr_state.bytes().collect::<Vec<_>>();
        let prev_bytes = prev_state.bytes().collect::<Vec<_>>();

        let (curr_first, curr_last) = find_first_last_char(
            &curr_bytes,
            b'#',
        );

        let (prev_first, prev_last) = find_first_last_char(
            &prev_bytes,
            b'#',
        );

        let curr_slice = &curr_bytes[(curr_first as usize)..=(curr_last as usize)];
        let prev_slice = &prev_bytes[(prev_first as usize)..=(prev_last as usize)];

        if curr_slice == prev_slice {
            break;
        }

        prev_state = curr_state.clone();
    }

    let mut sum = 0;
    let bytes = curr_state.bytes().collect::<Vec<_>>();
    for i in 0..bytes.len() {
        // Here, we note that 50_000_000_000 is the number of generations that we need to find the
        // sum of the numbers of the plants for. However, we've already gone through
        // `generation_count` generations (as reflected by `curr_state`). So, we need to subtract
        // `curr_state` from 50_000_000_000 to get the actual number of generations to offset our
        // numbers by.
        //
        // Put it another way, if we didn't subtract `generation_count` from 50 billion, we would
        // be offsetting each number by 50 billion + `generation_count` since `curr_state` already
        // accounts for `generation_count` generations.
        //
        // Put it another way, (50_000_000_000 - generation_count) is the number of times we would
        // have to shift `curr_states` right with '.' to get the right answer.
        //
        // Take this example. Suppose that the `init_plant_idx` (initial plant index) is 0 and we
        // wanted to find a more efficient way to calculate generation 12's value.
        //
        // GEN  curr_states         value
        // 1	..#.#.			    6
        // 2	.##..#			    5
        // 3	..##..#			    11
        // 4	...##..#		    14
        // 5	....##..#		    17
        // 6	.....##..#		    20
        // 7	......##..#		    23
        // 8	.......##..#		26
        // 9	........##..#		29
        // 10	.........##..#		32
        // 11	..........##..#		35
        // 12	...........##..#	38
        //
        // Generation 2 is the first generation such that every generation after would result in
        // a shifted repetition. Generation 3, then, is what `generation_count` is. If we added
        // (12 - 3) dots to the beginning of `curr_states` for generation 3, we would get exactly
        // `curr_states` for generation 12.
        //
        // Therefore:
        //      2 - 0 + (12 - 3) = 11
        //      3 - 0 + (12 - 3) = 12
        //      6 - 0 + (12 - 3) = 15
        //                   Sum = 38   ( = Generation 12's Value)
        //
        // We can apply the same idea here.
        sum += match bytes[i] {
            b'#' => (i as i64) - init_plant_idx + (50_000_000_000 - generation_count),
            _ => 0
        };
    }

    return sum;
}

/// Finds the first and last index of a byte (character).
///
/// # Parameters
/// - `bytes`: The vector of bytes to check.
/// - `byte_to_check`: The byte to look for.
///
/// # Returns
/// - A tuple contianing the starting index and the ending index. Both will either be a
/// non-negative integer (if found) or -1 (if not found).
fn find_first_last_char(bytes: &Vec<u8>, byte_to_check: u8) -> (i32, i32) {
    let first = bytes.iter().position(|&x| x == byte_to_check);
    let last = bytes.iter().rposition(|&x| x == byte_to_check);
    return match first {
        Some(idx) => (idx as i32, last.unwrap() as i32),
        None => (-1, -1)
    };
}

/// Runs one generation, simulating the growth or death of the plants.
///
/// # Parameters
/// - `init_state`: The initial state of the plants. Should only contain `#` (alive) and `.` (dead).
/// - `init_idx`: The initial index (where you start looking).
/// - `plant_notes`: The notes that describe how the plants live or die.
///
/// # Returns
/// - A tuple where the first element is the new state and the second element is the new initial
/// index.
fn run_one_generation(init_state: String, init_idx: i64,
                      plant_notes: &HashMap<String, String>) -> (String, i64) {
    // The plant that we're initially looking at
    let mut init_plant_idx = init_idx;

    let mut new_state = String::new();
    let mut curr_state_chars = init_state.bytes().collect::<Vec<_>>();

    let (first_idx, last_idx) = find_first_last_char(&curr_state_chars, b'#');

    // When inserting characters at index 0 of curr_state_chars:
    // - Adding 0 or 1 characters on the left side will subtract from the initial plant index.
    // - Adding 2 characters on left side will not change the initial plant index
    // - Adding x - 2 characters (where x > 2) will add (x - 2) to the initial plant index.
    let left_offset_inc = match first_idx {
        -1 => 2,
        idx => if idx < 6 { 6 } else { -2 }
    };

    init_plant_idx += left_offset_inc - 2;

    let right_offset_inc = match last_idx {
        -1 => 2,
        idx => if (idx as usize) > curr_state_chars.len() - 6 { 6 } else { 2 }
    };

    for _ in 0..left_offset_inc.abs() {
        if left_offset_inc < 0 {
            curr_state_chars.remove(0);
        } else {
            curr_state_chars.insert(0, b'.');
        }
    }

    for _ in 0..right_offset_inc {
        curr_state_chars.push(b'.');
    }

    curr_state_chars.windows(5).for_each(|window| {
        let window_str = String::from_utf8_lossy(window);
        match plant_notes.get(&*window_str) {
            Some(s) => new_state.push_str(s),
            None => new_state.push('.')
        }
    });

    assert_eq!(new_state.len(), curr_state_chars.len() - 4);
    return (new_state, init_plant_idx);
}