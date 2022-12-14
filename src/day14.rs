use crate::common::day;

pub struct Day14 {
    input: &'static str,
}

impl Day14 {
    pub fn new() -> Self {
        Self {
            input: include_str!("inputs/day14.txt"),
        }
    }
}

impl day::Day for Day14 {
    fn run(&mut self) -> day::Result {
        Ok((None, None))
    }
}
