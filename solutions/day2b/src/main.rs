use std::{str::FromStr, string::ParseError};

static INPUT_TEXT: &str = include_str!("../input.txt");

type Score = u32;

#[derive(Debug)]
struct Round {
    enemy_move: Move,
    result: RoundResult
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.is_ascii());
        assert_eq!(s.len(), 3);
        let (enemy_str, round_str) = s.split_once(' ').unwrap();
        Ok(Round {
            enemy_move: Move::from(enemy_str.chars().nth(0).unwrap()),
            result: RoundResult::from(round_str.chars().nth(0).unwrap()),
        })
    }
}

impl Round {
    fn infer_my_move(&self) -> Move {
        match (&self.result, self.enemy_move) {
            (RoundResult::Tie, a) => a,
            (RoundResult::Win, Move::Scissors) | (RoundResult::Loss, Move::Paper) => Move::Rock,
            (RoundResult::Win, Move::Rock) | (RoundResult::Loss, Move::Scissors) => Move::Paper,
            (RoundResult::Win, Move::Paper) | (RoundResult::Loss, Move::Rock) => Move::Scissors,
        }
    }


    fn calculate_score(&self) -> Score {
        return self.result.round_result_value() + self.infer_my_move().inherent_value()
    }
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Tie,
    Loss
}

impl From<char> for RoundResult {
    fn from(c: char) -> Self {
        match c {
            'X' => RoundResult::Loss,
            'Y' => RoundResult::Tie,
            'Z' => RoundResult::Win,
            _ => panic!(),
        }
    }
}

impl RoundResult {
    fn round_result_value(&self) -> Score{
        match self {
            RoundResult::Win => 6,
            RoundResult::Tie => 3,
            RoundResult::Loss => 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn inherent_value(&self) -> Score {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!(),
        }
    }
}

fn main() {
    let solution: Score = INPUT_TEXT
                            .trim_end()
                            .split('\n')
                            .map(|line| line.parse::<Round>().unwrap())
                            .map(|round| round.calculate_score())
                            .sum();
    println!("Day 2A Solution: {solution}")
}
