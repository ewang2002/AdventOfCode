use common::{
    constants::TWO_NEWLINE,
    day::{AoCProblem, Solution},
};

pub struct Day11 {
    notes: Vec<Note>,
}

impl AoCProblem for Day11 {
    fn prepare(input: String) -> Self {
        Self {
            notes: input
                .split(TWO_NEWLINE)
                .map(|note| {
                    let mut iterator = note.lines().skip(1);
                    let starting_items: Vec<_> = iterator
                        .next()
                        .unwrap()
                        .split_once(": ")
                        .unwrap()
                        .1
                        .split(", ")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect();
                    let operation_raw = iterator.next().unwrap().split_once("= old ").unwrap().1;
                    let test_div_by = iterator
                        .next()
                        .unwrap()
                        .split_once("by ")
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap();

                    let if_true = iterator
                        .next()
                        .unwrap()
                        .split_once("monkey ")
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap();
                    let if_false = iterator
                        .next()
                        .unwrap()
                        .split_once("monkey ")
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap();
                    Note {
                        starting_items,
                        operation: Operation {
                            is_add: operation_raw.starts_with('+'),
                            by: operation_raw[2..].parse::<usize>().ok(),
                        },
                        test_div: test_div_by,
                        throw_to_true: if_true,
                        throw_to_false: if_false,
                    }
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        simulate_monkey_game(&self.notes, 20, 3).into()
    }

    fn part2(&mut self) -> Solution {
        simulate_monkey_game(&self.notes, 10000, 1).into()
    }
}

/// Simulates the monkey game.
///
/// # Parameters
/// - `notes`: The notes (puzzle input).
/// - `rounds`:  The number of rounds to play.
/// - `div_worry_lvl_by`: The amount to divide the worry level by.
///
/// # Returns
/// The level of monkey business.
#[inline(always)]
fn simulate_monkey_game(notes: &[Note], rounds: usize, div_worry_lvl_by: usize) -> usize {
    let mut num_inspections: Vec<usize> = vec![0; notes.len()];
    let mut monkeys: Vec<Vec<usize>> = notes.iter().map(|x| x.starting_items.clone()).collect();
    // All of the numbers used when checking divisibility so happen to all be prime...
    let main_mod = notes.iter().fold(1, |old, new| old * new.test_div);

    for _ in 0..rounds {
        for (idx, note) in notes.iter().enumerate() {
            let curr_notes = monkeys[idx].clone();
            for item in curr_notes {
                let new_worry_lvl = (note.operation.to_fn()(item) / div_worry_lvl_by) % main_mod;

                if new_worry_lvl % note.test_div == 0 {
                    monkeys[note.throw_to_true].push(new_worry_lvl);
                } else {
                    monkeys[note.throw_to_false].push(new_worry_lvl);
                }

                monkeys[idx].remove(0);
                num_inspections[idx] += 1;
            }
        }
    }

    num_inspections.sort();
    num_inspections[num_inspections.len() - 2] * num_inspections[num_inspections.len() - 1]
}

#[derive(Clone)]
struct Note {
    starting_items: Vec<usize>,
    operation: Operation,
    test_div: usize,
    throw_to_true: usize,
    throw_to_false: usize,
}

#[derive(Clone)]
struct Operation {
    is_add: bool,
    by: Option<usize>,
}

impl Operation {
    /// Converts the operation into a function that can be called.
    ///
    /// # Returns
    /// The function.
    pub fn to_fn(&self) -> impl Fn(usize) -> usize + '_ {
        // Using Box<dyn Fn(usize) -> usize + '_> is slower by a bit due to
        // runtime overhead. We can do this because both closures return *one*
        // concrete type.
        |num| {
            if self.is_add {
                num + match self.by {
                    Some(u) => u,
                    None => num,
                }
            } else {
                num * match self.by {
                    Some(u) => u,
                    None => num,
                }
            }
        }
    }
}
