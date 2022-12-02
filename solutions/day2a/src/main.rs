use std::{str::FromStr, string::ParseError};

static INPUT_TEXT: &str = include_str!("../input.txt");

type Score = u32;

#[derive(Debug)]
struct Round {
    my_move: Move,
    enemy_move: Move
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.is_ascii());
        assert_eq!(s.len(), 3);
        let (enemy_str, my_str) = s.split_once(' ').unwrap();
        Ok(Round {
            my_move: Move::from(my_str.chars().nth(0).unwrap()),
            enemy_move: Move::from(enemy_str.chars().nth(0).unwrap()),
        })
    }
}

impl Round {
    fn get_round_result(&self) -> RoundResult{
        return self.my_move.fight(&self.enemy_move)
    }

    fn calculate_score(&self) -> Score {
        return self.get_round_result().round_result_value() + self.my_move.inherent_value()
    }
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Tie,
    Loss
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

#[derive(Debug, PartialEq)]
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
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!(),
        }
    }
}

impl Move {
    fn fight(&self, other: &Self) -> RoundResult {
        match (self, other) {
            (Move::Rock, Move::Rock) | (Move::Paper, Move::Paper) | (Move::Scissors, Move::Scissors) => RoundResult::Tie,
            (Move::Rock, Move::Paper) => RoundResult::Loss,
            (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) => RoundResult::Win,
            (Move::Paper, Move::Scissors) => RoundResult::Loss,
            (Move::Scissors, Move::Rock) => RoundResult::Loss,
            (Move::Scissors, Move::Paper) => RoundResult::Win,
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
