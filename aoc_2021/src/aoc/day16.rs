use crate::aoc::aoc_problem::{AoCProblem, Solution};
use std::cmp::min;
use std::collections::HashMap;

pub struct Day16 {
    orig_transmission: Vec<char>,
    transmission_binary: Vec<char>,
}

// https://adventofcode.com/2021/day/16
impl AoCProblem for Day16 {
    fn prepare(input: String) -> Self {
        let orig_transmission: Vec<char> = input.lines().nth(0).unwrap().chars().collect();
        let hex_to_bin_map: HashMap<char, &str> = HashMap::from([
            ('0', "0000"),
            ('1', "0001"),
            ('2', "0010"),
            ('3', "0011"),
            ('4', "0100"),
            ('5', "0101"),
            ('6', "0110"),
            ('7', "0111"),
            ('8', "1000"),
            ('9', "1001"),
            ('A', "1010"),
            ('B', "1011"),
            ('C', "1100"),
            ('D', "1101"),
            ('E', "1110"),
            ('F', "1111"),
        ]);

        let mut transmission_binary: Vec<char> = vec![];
        for c in &orig_transmission {
            hex_to_bin_map[c]
                .chars()
                .for_each(|bit| transmission_binary.push(bit));
        }

        Self {
            orig_transmission,
            transmission_binary,
        }
    }

    fn part1(&mut self) -> Solution {
        process_packet(
            &self.transmission_binary,
            &mut 0,
            self.transmission_binary.len(),
        )
        .all_packet_ids
        .iter()
        .sum::<usize>()
        .into()
    }

    fn part2(&mut self) -> Solution {
        process_packet(
            &self.transmission_binary,
            &mut 0,
            self.transmission_binary.len(),
        )
        .literal_packets[0]
            .into()
    }
}

/// Checks if there is another (sub)packet that needs to be processed.
///
/// # Parameters
/// - `transmission`: The transmission.
/// - `i`: The current index.
/// - `to`: The end index.
///
/// # Returns
/// `true` if there is another sub-packet that needs to be processed and `false` otherwise.
fn has_another_packet(transmission: &[char], i: usize, to: usize) -> bool {
    // 3 for the version
    // 3 for the type ID
    // Should have at least a 1 somewhere in there (no zero packet, assumption)
    (min(to, transmission.len()) as i64) - (i as i64) >= 6
        && transmission[i..].iter().any(|x| *x == '1')
}

/// Processes the entire packet.
///
/// # Parameters
/// - `transmission`: The transmission.
/// - `i`: The current index.
/// - `to`: The end index.
///
/// # Returns
/// The packet result.
fn process_packet(transmission: &[char], i: &mut usize, to: usize) -> PacketResult {
    let mut res: PacketResult = PacketResult {
        literal_packets: vec![],
        all_packet_ids: vec![],
    };

    while has_another_packet(transmission, *i, to) {
        let p = process_one_packet(transmission, i, to);
        p.literal_packets
            .into_iter()
            .for_each(|x| res.literal_packets.push(x));
        p.all_packet_ids
            .into_iter()
            .for_each(|x| res.all_packet_ids.push(x));
    }

    res
}

/// Processes one packet.
///
/// # Parameters
/// - `transmission`: The transmission.
/// - `i`: The current index.
/// - `to`: The end index.
///
/// # Returns
/// The packet result.
fn process_one_packet(transmission: &[char], i: &mut usize, to: usize) -> PacketResult {
    let mut res: PacketResult = PacketResult {
        literal_packets: vec![],
        all_packet_ids: vec![],
    };

    let version = extract_number(transmission, i, 3);
    res.all_packet_ids.push(version);

    let type_id = extract_number(transmission, i, 3);
    // Literal packet
    if type_id == 4 {
        res.literal_packets
            .push(process_literal_packet(transmission, i));
        return res;
    }

    // Otherwise, this must be some special packet
    let length_id = transmission[*i];
    *i += 1;

    match length_id {
        '0' => {
            let total_length = extract_number(transmission, i, 15);
            let p = process_packet(transmission, i, *i + total_length);
            p.all_packet_ids
                .into_iter()
                .for_each(|x| res.all_packet_ids.push(x));

            res.literal_packets
                .push(calculate_packet(&p.literal_packets, type_id));
        }
        '1' => {
            let mut num_sub_packets = extract_number(transmission, i, 11);
            let mut literal_packets: Vec<usize> = vec![];
            while num_sub_packets > 0 {
                let p = process_one_packet(transmission, i, to);
                p.all_packet_ids
                    .into_iter()
                    .for_each(|x| res.all_packet_ids.push(x));
                p.literal_packets
                    .into_iter()
                    .for_each(|x| literal_packets.push(x));
                num_sub_packets -= 1;
            }

            res.literal_packets
                .push(calculate_packet(&literal_packets, type_id));
        }
        _ => panic!("Unknown character: {}", length_id),
    }

    res
}

/// Calculates the value of a set of packets.
///
/// # Parameters
/// - `values`: The packets.
/// - `type_id`: The operator ID.
///
/// # Returns
/// The resulting packet value.
fn calculate_packet(values: &[usize], type_id: usize) -> usize {
    match type_id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknown operator: {}", type_id),
    }
}

/// Extracts a number from a transmission.
///
/// # Parameters
/// - `transmission`: The transmission.
/// - `i`: The current index.
/// - `extract_num`: The number of bits to process.
///
/// # Returns
/// The number.
fn extract_number(transmissions: &[char], i: &mut usize, extract_num: usize) -> usize {
    let mut processor = String::new();
    for _ in 0..extract_num {
        processor.push(transmissions[*i]);
        *i += 1;
    }

    usize::from_str_radix(&processor, 2).unwrap()
}

/// Processes the content of a literal packet. This will parse the literal value from this packet.
///
/// # Parameters
/// - `transmissions`: The original transmission information.
/// - `i`: The index pointing to the first character that represents a literal value.
///
/// # Returns
/// The literal value presented by this packet.
fn process_literal_packet(transmissions: &[char], i: &mut usize) -> usize {
    let mut str_processor = String::new();
    loop {
        let is_last_group = transmissions[*i] == '0';
        *i += 1;
        for _ in 0..4 {
            str_processor.push(transmissions[*i]);
            *i += 1;
        }

        if is_last_group {
            break;
        }
    }

    usize::from_str_radix(&str_processor, 2).unwrap()
}

#[derive(Debug)]
struct PacketResult {
    all_packet_ids: Vec<usize>,
    literal_packets: Vec<usize>,
}
