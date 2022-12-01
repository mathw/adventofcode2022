use std::{error::Error, num::ParseIntError, str::FromStr};

use crate::day::{Day, DayResult, PartResult};

pub struct Day1 {
    input: &'static str
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {input: include_str!("inputs/day1.txt")}
    }
}

impl Day for Day1 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let elves = parse_input(self.input)?;
        let most_calorific_elf = most_calorific_elf(&elves).ok_or_else(|| "There may not have been any elves".to_string())?;
        Ok(DayResult::new(PartResult::Success(format!("Most calorific elf has {} calories", most_calorific_elf)), PartResult::NotImplemented))
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    let mut elves = Vec::new();

    let mut current_elf = Vec::new();

    for line in input.lines().map(|l| l.trim()) {
        if !line.is_empty() {
            let n = u32::from_str(line)?;
            current_elf.push(n);
        }
        else {
            elves.push(current_elf);
            current_elf = Vec::new();
        }
    }
    if !current_elf.is_empty() {
        elves.push(current_elf);
    }

    Ok(elves)
}

fn most_calorific_elf(elves: &[Vec<u32>]) -> Option<u32> {
    elves.iter().map(|elf| elf.iter().sum()).max()
}

#[test]
fn test_parse_input() {
    let input = "45
88
    
22";
    let result = parse_input(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![vec![45, 88], vec![22]]);
}

#[test]
fn test_most_calorific_elf() {
    let elves = vec![vec![45, 88], vec![22]];
    assert_eq!(most_calorific_elf(&elves), Some(133));
}