use crate::day::{Day, DayResult, PartResult};
use regex::Regex;
use std::{collections::HashMap, error::Error, str::FromStr};

pub struct Day5 {
    input: &'static str,
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            input: include_str!("inputs/day5.txt"),
        }
    }
}

impl Day for Day5 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let moves = parse_input_instructions(self.input)?;
        let stacks = stacks_input();
        let part1_answer = run_part1(&moves, stacks)?;
        let stacks = stacks_input();
        let part2_answer = run_part2(&moves, stacks)?;
        Ok(DayResult::new(
            PartResult::Success(part1_answer),
            PartResult::Success(part2_answer),
        ))
    }
}

fn run_part1(moves: &[Move], mut stacks: Stacks) -> Result<String, String> {
    for m in moves {
        if !stacks.run_move(m) {
            return Err(format!("Move {:?} failed!", m));
        }
    }
    Ok(stacks.read_tops())
}

fn run_part2(moves: &[Move], mut stacks: Stacks) -> Result<String, String> {
    for m in moves {
        stacks
            .run_move_cratemover_9001(m)
            .ok_or_else(|| "Move failed".to_string())?;
    }
    Ok(stacks.read_tops())
}

#[derive(PartialEq, Eq, Debug)]
struct Stack(Vec<char>);

impl Stack {
    fn new(items: impl IntoIterator<Item = char>) -> Self {
        Self(items.into_iter().collect())
    }

    fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    fn push(&mut self, item: char) {
        self.0.push(item)
    }

    fn pop_many(&mut self, count: u8) -> Vec<char> {
        self.0.split_off(self.0.len() - (count as usize))
    }

    fn push_many(&mut self, items: &[char]) {
        for i in items {
            self.0.push(*i)
        }
    }

    fn peek_top(&self) -> Option<char> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0[self.0.len() - 1])
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Move {
    from: u8,
    to: u8,
    count: u8,
}

impl Move {
    fn new(from: u8, to: u8, count: u8) -> Self {
        Self { from, to, count }
    }
}

impl FromStr for Move {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Move, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)")
                .expect("I should be able to write valid regexes at compile time");
        }

        let m = RE
            .captures(s)
            .ok_or_else(|| format!("Input string '{}' did not match regex", s))?;

        let count_str = &m[1];
        let from_str = &m[2];
        let to_str = &m[3];

        Ok(Move::new(
            u8::from_str(from_str)?,
            u8::from_str(to_str)?,
            u8::from_str(count_str)?,
        ))
    }
}

fn parse_input_instructions(instructions: &str) -> Result<Vec<Move>, Box<dyn Error>> {
    instructions
        .lines()
        .map(Move::from_str)
        .collect::<Result<Vec<Move>, _>>()
}

#[derive(PartialEq, Eq, Debug)]
struct Stacks(HashMap<u8, Stack>);

impl Stacks {
    fn new(stacks: impl IntoIterator<Item = Stack>) -> Self {
        Self(
            stacks
                .into_iter()
                .enumerate()
                .map(|(i, s)| ((i + 1) as u8, s))
                .collect(),
        )
    }

    fn run_move(&mut self, m: &Move) -> bool {
        for _ in 0..m.count {
            if self.run_move_step(m).is_none() {
                return false;
            }
        }
        true
    }

    fn run_move_step(&mut self, m: &Move) -> Option<()> {
        let removed = self.0.get_mut(&m.from)?.pop()?;
        self.0.get_mut(&m.to)?.push(removed);
        Some(())
    }

    fn run_move_cratemover_9001(&mut self, m: &Move) -> Option<()> {
        let removed = self.0.get_mut(&m.from)?.pop_many(m.count);
        self.0.get_mut(&m.to)?.push_many(&removed);
        Some(())
    }

    fn read_tops(&self) -> String {
        let mut indexes: Vec<u8> = self.0.keys().cloned().collect();
        indexes.sort();
        indexes
            .into_iter()
            .filter_map(|s| self.0[&s].peek_top())
            .collect()
    }
}

fn make_stack(s: &str) -> Stack {
    Stack::new(s.chars())
}

/// Hardcoded because parsing it as specified looks like a pain I don't need
fn stacks_input() -> Stacks {
    Stacks::new(vec![
        make_stack("NSDCVQT"),
        make_stack("MFV"),
        make_stack("FQWDPNHM"),
        make_stack("DQRTF"),
        make_stack("RFMNQHVB"),
        make_stack("CFGNPWQ"),
        make_stack("WFRLCT"),
        make_stack("TZNS"),
        make_stack("MSDJRQHN"),
    ])
}

#[cfg(test)]
fn stacks_test_input() -> Stacks {
    Stacks::new(vec![make_stack("ZN"), make_stack("MCD"), make_stack("P")])
}

#[test]
fn test_parse_move() {
    let input = "move 23 from 7 to 4";
    let parsed = Move::from_str(input).expect("This test expects the input to parse");
    assert_eq!(parsed, Move::new(7, 4, 23));
}

#[test]
fn test_run_move() {
    let stack1 = Stack::new(vec!['A', 'B', 'C']);
    let stack2 = Stack::new(vec!['D', 'F', 'E']);
    let r#move = Move::new(2, 1, 2); // move two items from 2 to 1
    let mut stacks = Stacks::new(vec![stack1, stack2]);

    stacks.run_move(&r#move);

    assert_eq!(stacks.0[&1].0, vec!('A', 'B', 'C', 'E', 'F'));
    assert_eq!(stacks.0[&2].0, vec!('D'));
}

#[test]
fn test_part1_sample() {
    let stacks = stacks_test_input();
    let moves = parse_input_instructions(
        "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    )
    .expect("These moves should parse");

    let result = run_part1(&moves, stacks).expect("I expect success");

    assert_eq!(&result, "CMZ");
}

#[test]
fn test_part2_sample() {
    let stacks = stacks_test_input();
    let moves = parse_input_instructions(
        "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    )
    .expect("These moves should parse");

    let result = run_part2(&moves, stacks).expect("I expect success");

    assert_eq!(&result, "MCD");
}
