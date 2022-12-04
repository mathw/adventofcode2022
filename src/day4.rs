use crate::day::{Day, DayResult, PartResult};
use std::{error::Error, fmt::Debug, str::FromStr};

pub struct Day4 {
    input: &'static str,
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            input: include_str!("inputs/day4.txt"),
        }
    }
}

impl Day for Day4 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let contained_pairs = all_contained_pairs(self.input).expect("Expected parseable input");
        let part1_result = format!(
            "There are {} pairs with a fully contained assignment",
            contained_pairs.count()
        );
        Ok(DayResult::new(
            PartResult::Success(part1_result),
            PartResult::NotImplemented,
        ))
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Assignment {
    lower: u32,
    upper: u32,
}

impl Debug for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.lower, self.upper)
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let lower_str = parts
            .next()
            .ok_or_else(|| format!("Input string {} didn't contain a first part", s))?;
        let upper_str = parts
            .next()
            .ok_or_else(|| format!("Input string {} didn't contain a second part", s))?;
        let lower =
            u32::from_str(lower_str).map_err(|_| format!("Lower bound {} not a u32", lower_str))?;
        let upper =
            u32::from_str(upper_str).map_err(|_| format!("Upper bound {} not a u32", upper_str))?;
        Ok(Assignment { lower, upper })
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Pair {
    left: Assignment,
    right: Assignment,
}

impl Pair {
    fn one_fully_contains_other(&self) -> bool {
        self.left_fully_contains_right() || self.right_fully_contains_left()
    }

    fn left_fully_contains_right(&self) -> bool {
        self.left.lower <= self.right.lower && self.left.upper >= self.right.upper
    }

    fn right_fully_contains_left(&self) -> bool {
        self.right.lower <= self.left.lower && self.right.upper >= self.left.upper
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?},{:?}", self.left, self.right)
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let left_str = parts
            .next()
            .ok_or_else(|| format!("Input string {} did not have a left assignment", s))?;
        let right_str = parts
            .next()
            .ok_or_else(|| format!("Input string {} did not have a right assignment", s))?;
        let left = Assignment::from_str(left_str)?;
        let right = Assignment::from_str(right_str)?;
        Ok(Pair { left, right })
    }
}

fn all_contained_pairs(input: &str) -> Result<impl Iterator<Item = Pair>, <Pair as FromStr>::Err> {
    let pairs = input
        .lines()
        .map(Pair::from_str)
        .collect::<Result<Vec<Pair>, _>>()?;
    Ok(pairs.into_iter().filter(|p| p.one_fully_contains_other()))
}

#[cfg(test)]
const SAMPLE_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn test_parse_pair() {
    let input = "2-4,6-88";
    assert_eq!(
        Pair::from_str(input).unwrap(),
        Pair {
            left: Assignment { lower: 2, upper: 4 },
            right: Assignment {
                lower: 6,
                upper: 88
            }
        }
    );
}

#[test]
fn test_containment() {
    let pair = Pair::from_str("2-8,3-7").unwrap();
    assert!(pair.one_fully_contains_other());

    let pair = Pair::from_str("3-7,2-8").unwrap();
    assert!(pair.one_fully_contains_other());

    let pair = Pair::from_str("1-2,3-4").unwrap();
    assert_eq!(pair.one_fully_contains_other(), false);
}

#[test]
fn test_find_contained_pairs() {
    let contained_pairs = all_contained_pairs(SAMPLE_INPUT)
        .unwrap()
        .collect::<Vec<Pair>>();
    assert_eq!(contained_pairs.len(), 2);
    assert_eq!(contained_pairs[0], Pair::from_str("2-8,3-7").unwrap());
    assert_eq!(contained_pairs[1], Pair::from_str("6-6,4-6").unwrap());
}
