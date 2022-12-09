use std::error::Error;

use crate::day::{Day, DayResult, PartResult};

pub struct Day9 {
    input: &'static str,
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            input: include_str!("inputs/day9.txt"),
        }
    }
}

impl Day for Day9 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        Ok(DayResult::new(
            PartResult::NotImplemented,
            PartResult::NotImplemented,
        ))
    }
}
