use crate::day::{Day, DayResult, PartResult};
use std::error::Error;

pub struct Day6 {
    input: &'static str,
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            input: include_str!("inputs/day6.txt"),
        }
    }
}

impl Day for Day6 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        Ok(DayResult::new(
            PartResult::NotImplemented,
            PartResult::NotImplemented,
        ))
    }
}
