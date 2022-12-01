use std::error::Error;
use std::fmt::Display;

pub trait Day {
    fn run(&mut self) -> Result;
}

pub type Result = std::result::Result<DayResult, Box<dyn Error>>;

pub enum PartResult {
    Success(String),
    NotImplemented,
}

pub struct DayResult {
    part1: PartResult,
    part2: PartResult,
}

impl DayResult {
    pub fn new(part1: PartResult, part2: PartResult) -> DayResult {
        DayResult { part1, part2 }
    }
}

impl Default for DayResult {
    fn default() -> Self {
        DayResult {
            part1: PartResult::NotImplemented,
            part2: PartResult::NotImplemented,
        }
    }
}

impl Display for PartResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PartResult::NotImplemented => write!(f, "not implemented"),
            PartResult::Success(output) => write!(f, "SUCCESS\n{}", output),
        }
    }
}

impl Display for DayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "PART ONE: {}\n", self.part1)?;
        write!(f, "PART TWO: {}\n", self.part2)
    }
}