use std::error::Error;

use crate::day::{Day, DayResult, PartResult};

pub struct Day7 {
    input: &'static str,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            input: include_str!("inputs/day7.txt"),
        }
    }
}

impl Day for Day7 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        Ok(DayResult::new(
            PartResult::NotImplemented,
            PartResult::NotImplemented,
        ))
    }
}
