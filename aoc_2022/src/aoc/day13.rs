use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day13 {
    pairs: Vec<(Vec<PacketComponent>, Vec<PacketComponent>)>,
    all_packets: Vec<Vec<PacketComponent>>,
}

impl AoCProblem for Day13 {
    fn prepare(input: &str) -> Self {
        Self {
            pairs: input
                .split("\n\n")
                .map(|p| {
                    let mut iterator = p.lines();
                    (
                        parse_line(iterator.next().unwrap()),
                        parse_line(iterator.next().unwrap()),
                    )
                })
                .collect(),
            all_packets: input
                .lines()
                .filter(|l| !l.is_empty())
                .map(parse_line)
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut num_valid_idx = 0;

        for (idx, (p1, p2)) in self.pairs.iter().enumerate() {
            num_valid_idx += match evaluate_pair(p1, p2) {
                Decision::Correct => idx + 1,
                Decision::Incorrect => 0,
                Decision::None => 0,
            };
        }

        num_valid_idx.into()
    }

    fn part2(&mut self) -> Solution {
        0.into()
    }
}

fn evaluate_pair(p1: &[PacketComponent], p2: &[PacketComponent]) -> Decision {
    let mut idx1 = 0;
    let mut idx2 = 0;
    while idx1 < p1.len() && idx2 < p2.len() {
        match (&p1[idx1], &p2[idx2]) {
            (PacketComponent::Int(i1), PacketComponent::Int(i2)) => {
                if i1 == i2 {
                    idx1 += 1;
                    idx2 += 1;
                    continue;
                }

                return if i1 < i2 {
                    Decision::Correct
                } else {
                    Decision::Incorrect
                };
            }
            (PacketComponent::List(l1), PacketComponent::List(l2)) => {
                let res = evaluate_pair(l1, l2);
                match res {
                    Decision::Correct | Decision::Incorrect => {
                        return res;
                    }
                    _ => {
                        idx1 += 1;
                        idx2 += 1;
                        continue;
                    }
                };
            }
            (PacketComponent::List(l1), PacketComponent::Int(i2)) => {
                let res = evaluate_pair(l1, &[PacketComponent::Int(*i2)]);
                match res {
                    Decision::Correct | Decision::Incorrect => {
                        return res;
                    }
                    _ => {
                        idx1 += 1;
                        idx2 += 1;
                        continue;
                    }
                };
            }
            (PacketComponent::Int(i1), PacketComponent::List(l2)) => {
                let res = evaluate_pair(&[PacketComponent::Int(*i1)], l2);
                match res {
                    Decision::Correct | Decision::Incorrect => {
                        return res;
                    }
                    _ => {
                        idx1 += 1;
                        idx2 += 1;
                        continue;
                    }
                }
            }
        }
    }

    if idx1 == p1.len() && idx2 == p2.len() {
        Decision::None
    } else if idx1 == p1.len() {
        Decision::Correct
    } else {
        Decision::Incorrect
    }
}

#[derive(Debug)]
enum Decision {
    Correct,
    Incorrect,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PacketComponent {
    Int(usize),
    List(Vec<PacketComponent>),
}

pub fn parse_line(packet: &str) -> Vec<PacketComponent> {
    match parse_helper(&packet.chars().collect::<Vec<_>>(), &mut 1) {
        PacketComponent::List(l) => Vec::from_iter(l.into_iter()),
        otherwise => vec![otherwise],
    }
}

fn parse_helper(packet: &[char], idx: &mut usize) -> PacketComponent {
    let mut final_vec: Vec<PacketComponent> = vec![];
    let mut val = 0;
    let mut is_processing = false;
    while *idx < packet.len() - 1 {
        match packet[*idx] {
            '[' => {
                *idx += 1;
                let res = parse_helper(packet, idx);
                final_vec.push(res);
            }
            ']' => {
                *idx += 1;
                if is_processing {
                    final_vec.push(PacketComponent::Int(val));
                }
                return PacketComponent::List(final_vec);
            }
            ',' => {
                if is_processing {
                    final_vec.push(PacketComponent::Int(val));
                    val = 0;
                    is_processing = false;
                }
                *idx += 1;
            }
            c if c.is_ascii_digit() => {
                is_processing = true;
                val = val * 10 + (c.to_digit(10).unwrap() as usize);
                *idx += 1;
            }
            _ => unreachable!("should not eb reachable"),
        }
    }

    if is_processing {
        final_vec.push(PacketComponent::Int(val));
    }
    PacketComponent::List(final_vec)
}

#[cfg(test)]
mod tests {
    use super::{parse_line, PacketComponent};

    #[test]
    fn test_basic() {
        assert_eq!(
            vec![
                PacketComponent::Int(1),
                PacketComponent::Int(1),
                PacketComponent::Int(3),
                PacketComponent::Int(1),
                PacketComponent::Int(1),
            ],
            parse_line("[1,1,3,1,1]")
        )
    }

    #[test]
    fn test_nest_once() {
        assert_eq!(
            vec![
                PacketComponent::List(vec![PacketComponent::Int(1)]),
                PacketComponent::List(vec![
                    PacketComponent::Int(2),
                    PacketComponent::Int(3),
                    PacketComponent::Int(4)
                ]),
            ],
            parse_line("[[1],[2,3,4]]")
        )
    }

    #[test]
    fn test_nest_mixed() {
        assert_eq!(
            vec![
                PacketComponent::List(vec![PacketComponent::Int(1)]),
                PacketComponent::Int(4),
            ],
            parse_line("[[1],4]")
        )
    }

    #[test]
    fn test_nest_mixed_again() {
        assert_eq!(
            vec![
                PacketComponent::List(vec![PacketComponent::Int(4), PacketComponent::Int(4)]),
                PacketComponent::Int(4),
                PacketComponent::Int(4),
                PacketComponent::Int(4)
            ],
            parse_line("[[4,4],4,4,4]")
        )
    }

    #[test]
    fn test_empty() {
        assert_eq!(Vec::<PacketComponent>::new(), parse_line("[]"))
    }

    #[test]
    fn test_empty_nested() {
        assert_eq!(
            vec![PacketComponent::List(vec![PacketComponent::List(vec![])])],
            parse_line("[[[]]]")
        )
    }

    #[test]
    fn test_complex_nesting() {
        assert_eq!(
            vec![
                PacketComponent::Int(1),
                PacketComponent::List(vec![
                    PacketComponent::Int(2),
                    PacketComponent::List(vec![
                        PacketComponent::Int(3),
                        PacketComponent::List(vec![
                            PacketComponent::Int(4),
                            PacketComponent::List(vec![
                                PacketComponent::Int(5),
                                PacketComponent::Int(6),
                                PacketComponent::Int(7),
                            ])
                        ])
                    ])
                ]),
                PacketComponent::Int(8),
                PacketComponent::Int(9),
            ],
            parse_line("[1,[2,[3,[4,[5,6,7]]]],8,9]")
        )
    }
}
