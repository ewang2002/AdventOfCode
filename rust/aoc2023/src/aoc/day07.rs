use std::{cmp::Ordering, collections::HashMap};

use common::problem::day::{AoCProblem, Solution};

const PART1_ORDERING: [(char, u32); 13] = [
    ('A', 100),
    ('K', 99),
    ('Q', 98),
    ('J', 97),
    ('T', 96),
    ('9', 95),
    ('8', 94),
    ('7', 93),
    ('6', 92),
    ('5', 91),
    ('4', 90),
    ('3', 89),
    ('2', 88),
];

const PART2_ORDERING: [(char, u32); 13] = [
    ('A', 100),
    ('K', 99),
    ('Q', 98),
    ('T', 96),
    ('9', 95),
    ('8', 94),
    ('7', 93),
    ('6', 92),
    ('5', 91),
    ('4', 90),
    ('3', 89),
    ('2', 88),
    ('J', 0),
];

pub struct Day07 {
    hands: Vec<Hand>,
}

impl AoCProblem for Day07 {
    fn prepare(input: String) -> Self {
        Self {
            hands: input
                .lines()
                .map(|d| d.split_once(' ').unwrap())
                .map(|(raw_cards, bid)| Hand::new(raw_cards, bid.parse().unwrap()))
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let ordering: HashMap<char, u32> = HashMap::from_iter(PART1_ORDERING);
        let mut all_hands: Vec<_> = get_hand_type(&self.hands, false);
        all_hands.sort_by(|h1, h2| compare_hand_and_types(h1, h2, &ordering));
        all_hands
            .iter()
            .enumerate()
            .map(|(idx, (hand, _))| (idx + 1) * hand.bid)
            .sum::<usize>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        let ordering: HashMap<char, u32> = HashMap::from_iter(PART2_ORDERING);
        let mut all_hands: Vec<_> = get_hand_type(&self.hands, true);
        all_hands.sort_by(|h1, h2| compare_hand_and_types(h1, h2, &ordering));
        all_hands
            .iter()
            .enumerate()
            .map(|(idx, (hand, _))| (idx + 1) * hand.bid)
            .sum::<usize>()
            .into()
    }

    fn day() -> u32 {
        7
    }

    fn year() -> u32 {
        2023
    }
}

pub struct Hand {
    /// The cards associated with this hand.
    pub cards: Vec<char>,

    /// The bid value for this hand.
    pub bid: usize,
}

impl Hand {
    /// Creates a new `Hand` instance from the given raw cards and bid value.
    ///
    /// # Parameters
    /// - `raw_cards`: A string containing the cards.
    /// - `bid_value`: The hand's bid.
    ///
    /// # Returns
    /// The hand.
    pub fn new(raw_cards: &str, bid_value: usize) -> Self {
        Self {
            cards: raw_cards.chars().collect(),
            bid: bid_value,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    /// Upgrades the hand type based on the number of jokers that are available. This
    /// is used in part 2.
    ///
    /// # Parameters
    /// - `num_jokers`: The number of jokers.
    ///
    /// # Returns
    /// The hand type representing the most powerful hand we could produce if the jokers
    /// were replaced.
    pub fn upgrade(&self, num_jokers: usize) -> HandType {
        match self {
            // FIVE OF A KIND (5)
            // Cannot upgrade.
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            // FOUR OF A KIND (4 / 1)
            // - If we have 1 J, then we get five of a kind (5)
            //      e.g., AAAAJ -> AAAAA
            // - If we have 4 J, then we get five of a kind (5)
            //      e.g., JJJJA -> AAAAA
            HandType::FourOfAKind if num_jokers >= 1 => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FourOfAKind,

            // FULL HOUSE (3 / 2)
            // - If we have 3 J, then we get five of a kind (5)
            //      e.g., JJJAA -> AAAAA
            // - If we have 2 J, then we get five of a kind (5)
            //      e.g., AAAJJ -> JJJJJ
            HandType::FullHouse if num_jokers >= 1 => HandType::FiveOfAKind,
            HandType::FullHouse => HandType::FullHouse,

            // THREE OF A KIND (3 / 1 / 1)
            // - If we have 3 J, then we get 4 of a kind (4 / 1)
            //      e.g., JJJAB -> AAAAB
            // - If we have 1 J, then we get 4 of a kind (4 / 1)
            //      e.g., AAAJB -> AAAAB
            HandType::ThreeOfAKind if num_jokers >= 1 => HandType::FourOfAKind,
            HandType::ThreeOfAKind => HandType::ThreeOfAKind,

            // TWO PAIR (2 / 2 / 1),
            // - If we have 2 J, then we get 4 of a kind (4 / 1)
            //      e.g., JJAAB -> JJJJA
            // - If we have 1 J, then we get full house (3 / 2)
            //      e.g., AABBJ -> AABBJ
            HandType::TwoPair if num_jokers == 2 => HandType::FourOfAKind,
            HandType::TwoPair if num_jokers == 1 => HandType::FullHouse,
            HandType::TwoPair => HandType::TwoPair,

            // ONE PAIR (2 / 1 / 1 / 1)
            // - If we have 2 J, we can get 3 of a kind (3 / 1 / 1)
            // - If we have 1 J, we can get 3 of a kind (3 / 1 / 1)
            HandType::OnePair if num_jokers >= 1 => HandType::ThreeOfAKind,
            HandType::OnePair => HandType::OnePair,

            // HIGH CARD (1 / 1 / 1 / 1 / 1)
            // - If we have 1 J, then we can get one pair (2 / 1 / 1 / 1)
            HandType::HighCard if num_jokers == 1 => HandType::OnePair,
            HandType::HighCard => HandType::HighCard,
        }
    }
}

/// A function that can be used to compare two hands to get their ordering.
///
/// # Parameters
/// - The first and second parameter is a tuple where the first element is a hand and the second element is its type.
/// - `ordering`: The designated ordering of each card, used to break any ties.
///
/// # Returns
/// The ordering.
#[inline(always)]
#[allow(clippy::comparison_chain)]
fn compare_hand_and_types(
    (h1, h1_type): &(&Hand, HandType),
    (h2, h2_type): &(&Hand, HandType),
    ordering: &HashMap<char, u32>,
) -> Ordering {
    if h1_type > h2_type {
        Ordering::Greater
    } else if h1_type < h2_type {
        Ordering::Less
    } else {
        for (c1, c2) in h1.cards.iter().zip(&h2.cards) {
            let ord1 = *ordering.get(c1).unwrap();
            let ord2 = *ordering.get(c2).unwrap();
            if ord1 > ord2 {
                return Ordering::Greater;
            } else if ord1 < ord2 {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

/// Gets a list of hands with their associated hand type.
///
/// # Parameters
/// - `hands`: The hands (input).
/// - `should_upgrade`: Whether jokers should be upgraded.
///
/// # Returns
/// A vector of tuples where each tuple has a reference to a hand and its type.
#[inline(always)]
fn get_hand_type(hands: &[Hand], should_upgrade: bool) -> Vec<(&Hand, HandType)> {
    hands
        .iter()
        .map(|hand| {
            let mut card_count: HashMap<char, u32> = HashMap::new();
            for card in &hand.cards {
                *card_count.entry(*card).or_default() += 1;
            }

            let vals: Vec<_> = card_count.values().copied().collect();

            let hand_type = match vals.len() {
                1 => HandType::FiveOfAKind,
                2 if vals[0] == 4 || vals[1] == 4 => HandType::FourOfAKind,
                2 => HandType::FullHouse,
                3 if vals[0] == 3 || vals[1] == 3 || vals[2] == 3 => HandType::ThreeOfAKind,
                3 => HandType::TwoPair,
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => unreachable!(),
            };

            let num_jokers = hand.cards.iter().filter(|c| **c == 'J').count();
            if should_upgrade && hand.cards.iter().any(|c| *c == 'J') {
                (hand, hand_type.upgrade(num_jokers))
            } else {
                (hand, hand_type)
            }
        })
        .collect()
}
