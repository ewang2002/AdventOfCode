use crate::aoc::aoc_problem::AoCProblem;

const O_ROCK: char = 'A';
const O_PAPER: char = 'B';
const O_SCISSORS: char = 'C';

const Y_ROCK: char = 'X';
const Y_PAPER: char = 'Y';
const Y_SCISSORS: char = 'Z';

const S_LOSE: char = 'X';
const S_TIE: char = 'Y';
const S_WIN: char = 'Z';

const P_LOSE: usize = 0;
const P_TIE: usize = 3;
const P_WIN: usize = 6;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;

pub struct Day02 {
    guide: Vec<Vec<char>>,
}

impl AoCProblem<usize, usize> for Day02 {
    fn prepare(input: &str) -> Self {
        Self {
            guide: input
                .lines()
                .map(|x| x.chars().filter(|y| *y != ' ').collect())
                .collect(),
        }
    }

    fn part1(&mut self) -> usize {
        let mut points = 0;
        for guide in &self.guide {
            // (Opponent Picks, You Pick)
            points += match (guide[0], guide[1]) {
                // All possible winning cases first
                (O_SCISSORS, Y_ROCK) => ROCK + P_WIN,
                (O_ROCK, Y_PAPER) => PAPER + P_WIN,
                (O_PAPER, Y_SCISSORS) => SCISSORS + P_WIN,
                // Ties
                (O_ROCK, Y_ROCK) => ROCK + P_TIE,
                (O_PAPER, Y_PAPER) => PAPER + P_TIE,
                (O_SCISSORS, Y_SCISSORS) => SCISSORS + P_TIE,
                // Loses
                (_, Y_ROCK) => ROCK + P_LOSE,
                (_, Y_PAPER) => PAPER + P_LOSE,
                (_, Y_SCISSORS) => SCISSORS + P_LOSE,
                (a, b) => panic!("What is {} and {}?", a, b),
            };
        }

        points
    }

    fn part2(&mut self) -> usize {
        let mut points = 0;
        for guide in &self.guide {
            // (Opponent Picks, Desired Outcome)
            points += match (guide[0], guide[1]) {
                // Wins
                (O_ROCK, S_WIN) => PAPER + P_WIN,
                (O_PAPER, S_WIN) => SCISSORS + P_WIN,
                (O_SCISSORS, S_WIN) => ROCK + P_WIN,
                // Ties
                (O_ROCK, S_TIE) => ROCK + P_TIE,
                (O_PAPER, S_TIE) => PAPER + P_TIE,
                (O_SCISSORS, S_TIE) => SCISSORS + P_TIE,
                // Loses
                (O_ROCK, _) => SCISSORS + P_LOSE,
                (O_PAPER, _) => ROCK + P_LOSE,
                (O_SCISSORS, _) => PAPER + P_LOSE,
                _ => panic!("What is {} and {}?", guide[0], guide[1]),
            };
        }

        points
    }
}
