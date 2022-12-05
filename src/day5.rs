use crate::day::{Day, DayResult, PartResult};
use std::error::Error;

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
        Ok(DayResult::new(
            PartResult::NotImplemented,
            PartResult::NotImplemented,
        ))
    }
}
