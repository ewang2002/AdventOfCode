use std::collections::HashMap;

pub struct Day10 {
    nav_subsystem: Vec<String>,
    incomplete: Vec<String>,
}

// https://adventofcode.com/2021/day/10
// Remark: I'm not using the AoCProblem trait since using it would cause a significant
// inconvenience.
impl Day10 {
    pub fn prepare(input: Vec<String>) -> Self {
        return Day10 {
            nav_subsystem: input,
            incomplete: vec![],
        };
    }

    pub fn part1(&mut self) -> usize {
        let score_map: HashMap<char, usize> = HashMap::from([
            (')', 3),
            (']', 57),
            ('}', 1197),
            ('>', 25137)
        ]);

        let mut total_score: usize = 0;
        for sub in &self.nav_subsystem {
            let mut stack: Vec<char> = vec![];
            let mut is_incomplete = true;
            for c in sub.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        let poss_match = stack.pop();
                        match poss_match {
                            Some('(') => {
                                if c != ')' {
                                    total_score += score_map.get(&c).unwrap();
                                    is_incomplete = false;
                                    break;
                                }
                            }
                            Some('{') => {
                                if c != '}' {
                                    total_score += score_map.get(&c).unwrap();
                                    is_incomplete = false;
                                    break;
                                }
                            }
                            Some('[') => {
                                if c != ']' {
                                    total_score += score_map.get(&c).unwrap();
                                    is_incomplete = false;
                                    break;
                                }
                            }
                            Some('<') => {
                                if c != '>' {
                                    total_score += score_map.get(&c).unwrap();
                                    is_incomplete = false;
                                    break;
                                }
                            }
                            Some(x) => panic!("unexpected character: {}", x),
                            None => {}
                        };
                    }
                    _ => panic!("Invalid char: {}", c)
                };
            }

            if is_incomplete {
                self.incomplete.push(sub.clone());
            }
        }

        return total_score;
    }

    pub fn part2(&self) -> usize {
        let mut all_scores: Vec<usize> = vec![];
        for str in &self.incomplete {
            let mut stack: Vec<char> = vec![];
            for c in str.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        let poss_match = stack.pop();
                        // If no more, then we can complete the rest of the pattern.
                        if poss_match.is_none() {
                            break;
                        }
                    }
                    _ => panic!("Invalid char: {}", c)
                };
            }

            let mut score: usize = 0;
            for c in stack.iter().rev() {
                score *= 5;
                match *c {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!("Invalid symbol: {}", *c)
                };
            }

            all_scores.push(score);
        }

        all_scores.sort();
        return all_scores[all_scores.len() / 2];
    }
}