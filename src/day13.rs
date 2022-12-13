use crate::day::{Day, DayResult, PartResult};

pub struct Day13 {
    input: &'static str,
}

impl Day13 {
    pub fn new() -> Self {
        Self {
            input: include_str!("inputs/day13.txt"),
        }
    }
}

impl Day for Day13 {
    fn run(&mut self) -> crate::day::Result {
        Ok(DayResult::new(
            PartResult::NotImplemented,
            PartResult::NotImplemented,
        ))
    }
}
