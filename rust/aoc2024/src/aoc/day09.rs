use std::collections::{HashMap, VecDeque};

use common::problem::day::{AoCProblem, Solution};

pub struct Day09 {
    disk_map: Vec<u32>,
}

impl AoCProblem for Day09 {
    fn prepare(input: String) -> Self {
        Self {
            disk_map: input.chars().map(|c| c.to_digit(10).unwrap()).collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut files = VecDeque::new();
        let mut free_spaces = VecDeque::new();
        let mut file_id = 0;
        for (idx, digit) in self.disk_map.iter().enumerate() {
            if idx % 2 == 0 {
                files.push_back((file_id, *digit));
                file_id += 1;
            } else {
                free_spaces.push_back(*digit);
            }
        }

        let mut compressed_map = vec![];
        while let Some((file_id, block_ct)) = files.pop_front() {
            for _ in 0..block_ct {
                compressed_map.push(file_id);
            }

            // Handle filling empty spaces
            let mut free_space_count = match free_spaces.pop_front() {
                Some(s) => s,
                None => break,
            };

            while free_space_count > 0 {
                let (back_file_id, back_block_ct) = match files.pop_back() {
                    Some(s) => s,
                    None => break,
                };

                if back_block_ct <= free_space_count {
                    compressed_map.extend((0..back_block_ct).map(|_| back_file_id));
                    free_space_count -= back_block_ct;
                } else {
                    compressed_map.extend((0..free_space_count).map(|_| back_file_id));
                    files.push_back((back_file_id, back_block_ct - free_space_count));
                    break;
                }
            }
        }

        compressed_map
            .into_iter()
            .enumerate()
            .map(|(i, id)| i * id)
            .sum::<usize>()
            .into()
    }

    // 8705230292234 = too high
    fn part2(&mut self) -> Solution {
        struct FileInfo {
            size: usize,
            loc: usize,
        }

        let mut file_size_location = HashMap::new();
        let mut compressed_map: Vec<Block> = vec![];
        let mut free_spaces: VecDeque<(usize, usize)> = VecDeque::new();
        let mut back_file_id: usize = 0;
        for (idx, digit) in self.disk_map.iter().enumerate() {
            if idx % 2 == 0 {
                file_size_location.insert(
                    back_file_id,
                    FileInfo {
                        size: *digit as usize,
                        loc: compressed_map.len(),
                    },
                );
                compressed_map.extend((0..*digit).map(|_| Block::Occupied(back_file_id)));
                back_file_id += 1;
            } else {
                free_spaces.push_back((compressed_map.len(), *digit as usize));
                compressed_map.extend((0..*digit).map(|_| Block::Free));
            }
        }

        let mut file_id = back_file_id - 1;
        loop {
            let mut to_add_back = VecDeque::new();
            for _ in 0..free_spaces.len() {
                let size = file_size_location[&file_id].size;
                let loc = file_size_location[&file_id].loc;

                match free_spaces.pop_front() {
                    // If we have free space to put this in, do so
                    Some((idx, free_space_ct)) if free_space_ct >= size => {
                        for i in 0..size {
                            compressed_map[idx + i] = Block::Occupied(file_id);
                        }

                        for block_to_free in compressed_map.iter_mut().skip(loc).take(size) {
                            *block_to_free = Block::Free;
                        }

                        if free_space_ct - size > 0 {
                            to_add_back.push_front((idx + size, free_space_ct - size));
                        }

                        break;
                    }
                    // Otherwise, if the index of this free space is BEYOND the location of the file, then break out
                    Some(p @ (idx, _)) if idx > loc => {
                        to_add_back.push_front(p);
                        break;
                    }
                    // Otherwise, try again
                    Some(p) => {
                        to_add_back.push_front(p);
                    }
                    None => {
                        continue;
                    }
                }
            }

            if file_id == 0 {
                break;
            }

            file_id -= 1;
            while let Some(p) = to_add_back.pop_front() {
                free_spaces.push_front(p);
            }
        }

        compressed_map
            .into_iter()
            .enumerate()
            .map(|(i, id)| {
                i * match id {
                    Block::Free => 0,
                    Block::Occupied(o) => o,
                }
            })
            .sum::<usize>()
            .into()
    }

    fn day() -> u32 {
        9
    }

    fn year() -> u32 {
        2024
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Free,
    Occupied(usize),
}
