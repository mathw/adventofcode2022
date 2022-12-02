use std::{error::Error, num::ParseIntError, str::FromStr};

use crate::day::{Day, DayResult, PartResult};

pub struct Day2 {
    input: &'static str,
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            input: include_str!("inputs/day2.txt"),
        }
    }
}

impl Day for Day2 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let moves = parse_input_for_part1(self.input);

        let total_score: u32 = moves.iter().map(|m| m.score()).sum();
        let part1_result = format!("Total score is {}", total_score);

        let guide = transform_input_for_part2(&moves);
        let total_score: u32 = guide.iter().map(|g| g.score()).sum();
        let part2_result = format!("Total score with the corrected guide is {}", total_score);

        Ok(DayResult::new(
            PartResult::Success(part1_result),
            PartResult::Success(part2_result),
        ))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn winning_move(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn losing_move(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn draw_move(&self) -> Choice {
        *self
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(PartialEq, Eq, Debug)]
struct Move(Choice, Choice);

impl Move {
    fn score(&self) -> u32 {
        let outcome_score = match self.outcome() {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        };
        outcome_score + self.1.score()
    }

    fn outcome(&self) -> Outcome {
        match self {
            Move(a, b) if a == b => Outcome::Draw,
            Move(a, b) if *b == a.winning_move() => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

fn parse_input_for_part1(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter_map(parse_input_line_for_part1)
        .collect()
}

fn parse_input_line_for_part1(line: &str) -> Option<Move> {
    let mut chars = line.chars();
    let first_move = match chars.next() {
        Some('A') => Some(Choice::Rock),
        Some('B') => Some(Choice::Paper),
        Some('C') => Some(Choice::Scissors),
        _ => Option::<Choice>::default(),
    }?;
    chars.next()?;
    let second_move = match chars.next() {
        Some('X') => Some(Choice::Rock),
        Some('Y') => Some(Choice::Paper),
        Some('Z') => Some(Choice::Scissors),
        _ => Option::<Choice>::default(),
    }?;
    Some(Move(first_move, second_move))
}

#[derive(Debug, PartialEq, Eq)]
struct Guide(Choice, Outcome);

impl Guide {
    fn needed_choice(&self) -> Choice {
        match self.1 {
            Outcome::Win => self.0.winning_move(),
            Outcome::Lose => self.0.losing_move(),
            Outcome::Draw => self.0.draw_move(),
        }
    }

    fn score(&self) -> u32 {
        Move(self.0, self.needed_choice()).score()
    }
}

fn transform_input_for_part2(part1_input: &Vec<Move>) -> Vec<Guide> {
    part1_input
        .iter()
        .map(|m| {
            Guide(
                m.0,
                match m.1 {
                    Choice::Rock => Outcome::Lose,
                    Choice::Paper => Outcome::Draw,
                    Choice::Scissors => Outcome::Win,
                },
            )
        })
        .collect()
}

#[cfg(test)]
const SAMPLE_INPUT: &'static str = "A Y
B X
C Z";

#[test]
fn test_parse_input_line() {
    let line = "A Y";
    let result = parse_input_line_for_part1(line);
    assert_eq!(result, Some(Move(Choice::Rock, Choice::Paper)));
}

#[test]
fn test_parse_input() {
    let moves = parse_input_for_part1(SAMPLE_INPUT);
    assert_eq!(
        moves,
        vec![
            Move(Choice::Rock, Choice::Paper),
            Move(Choice::Paper, Choice::Rock),
            Move(Choice::Scissors, Choice::Scissors)
        ]
    );
}

#[test]
fn test_move_score() {
    assert_eq!(Move(Choice::Rock, Choice::Paper).score(), 8);
    assert_eq!(Move(Choice::Paper, Choice::Rock).score(), 1);
    assert_eq!(Move(Choice::Scissors, Choice::Scissors).score(), 6);
}
