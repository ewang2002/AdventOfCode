use std::collections::HashMap;

use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day07 {
    // Key = directory
    // Value = HashMap where
    //      Key = file name
    //      Value = size
    dir_info: HashMap<String, HashMap<String, usize>>,
    sizes: HashMap<String, usize>,
}

impl AoCProblem for Day07 {
    fn prepare(input: &str) -> Self {
        let mut map: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut curr_path: Vec<&str> = vec![];

        let all_lines: Vec<&str> = input.lines().skip(1).collect();
        let mut i: usize = 0;
        while i < all_lines.len() {
            let line = all_lines[i];
            match &line[..4] {
                "$ cd" => {
                    match &line[5..] {
                        ".." => {
                            curr_path.pop();
                        }
                        "/" => {
                            curr_path = vec![];
                        }
                        _ => {
                            curr_path.push(&line[5..]);
                        }
                    }

                    i += 1;
                }
                "$ ls" => {
                    let path = curr_path.join("/");
                    map.entry(path.clone()).or_insert_with(HashMap::new);
                    i += 1;
                    while i < all_lines.len() && !all_lines[i].starts_with('$') {
                        let (type_or_size, name) = all_lines[i].split_once(' ').unwrap();
                        if type_or_size == "dir" {
                            let base_path = if curr_path.is_empty() {
                                "".to_owned()
                            } else {
                                curr_path.join("/") + "/"
                            };

                            map.entry(format!("{}{}", base_path, name))
                                .or_insert_with(HashMap::new);
                        } else {
                            map.get_mut(&path)
                                .unwrap()
                                .insert(name.to_owned(), type_or_size.parse().unwrap());
                        }

                        i += 1;
                    }
                }
                _ => unreachable!("assuming all cases start with $ only."),
            }
        }

        let sizes = compute_total_size(&map);
        Self {
            dir_info: map,
            sizes,
        }
    }

    fn part1(&mut self) -> Solution {
        self.sizes
            .values()
            .filter(|s| **s <= 100_000)
            .sum::<usize>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        let space_to_clear = 30_000_000 - (70_000_000 - *self.sizes.get("").unwrap());
        self.sizes
            .values()
            .filter(|x| **x >= space_to_clear)
            .min()
            .unwrap()
            .into()
    }
}

/// Computes the size of all directories.
///
/// # Parameters
/// - `dir_map`: The directory map.
///
/// # Returns
/// A map where the key is the directory and the value is the size of all children items.
fn compute_total_size(dir_map: &HashMap<String, HashMap<String, usize>>) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    compute_helper(dir_map, &mut map, String::new(), 0);
    map
}

/// Computes the size of a directory. This is a helper recursive function.
///
/// # Parameters
/// - `dir_map`: The directory map.
/// - `map`: The return map (for the function that called this function).
/// - `curr_path`: The current path to consider.
/// - `num_slashes`: The number of slashes to consider when computing the sizes.
///
/// # Returns
/// The total size of the given directory.
fn compute_helper(
    dir_map: &HashMap<String, HashMap<String, usize>>,
    map: &mut HashMap<String, usize>,
    curr_path: String,
    num_slashes: usize,
) -> usize {
    let dirs_to_consider: Vec<&str> = dir_map
        .keys()
        .filter(|d| {
            d.starts_with(&curr_path)
                && d.as_str() != curr_path
                && d.chars().filter(|x| *x == '/').count() == num_slashes
        })
        .map(|d| d.as_str())
        .collect();
    let all_files = dir_map.get(&curr_path).unwrap().values().sum::<usize>();

    // Base Case: no more directories to consider.
    if dirs_to_consider.is_empty() {
        map.insert(curr_path, all_files);
        return all_files;
    }

    // Otherwise, we need to consider the sum of all remaining directories
    let size_child_dirs = dirs_to_consider
        .iter()
        .map(|d| compute_helper(dir_map, map, d.to_string(), num_slashes + 1))
        .sum::<usize>();

    map.insert(curr_path, size_child_dirs + all_files);
    size_child_dirs + all_files
}
