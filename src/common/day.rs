use std::error::Error;

pub trait Day {
    fn run(&mut self) -> Result;
}

pub type Result = std::result::Result<(Option<String>, Option<String>), Box<dyn Error>>;
