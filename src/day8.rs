use std::error::Error;

use crate::day::{Day, DayResult, PartResult};

pub struct Day8 {
    input: &'static str,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            input: include_str!("inputs/day7.txt"),
        }
    }
}

impl Day for Day8 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        Ok(DayResult::new(
            PartResult::NotImplemented,
            PartResult::NotImplemented,
        ))
    }
}
