mod parser;
mod types;

use std::cmp::Ordering;
use std::iter;

use chumsky::Parser;
use itertools::Itertools;

use self::parser::parser;
use self::types::Value;
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
        let pairs = parse_input_pairs(self.input)?;
        let part1 = run_part1(&pairs);
        let part2 = run_part2(pairs);
        Ok(DayResult::new(
            PartResult::Success(part1.to_string()),
            PartResult::Success(part2.to_string()),
        ))
    }
}
fn parse_input_pairs(input: &str) -> Result<Vec<(Value, Value)>, String> {
    let mut results = Vec::new();
    let mut current_left = String::new();
    let mut current_right = String::new();

    for line in input.lines() {
        if line.is_empty() {
            current_left.clear();
            current_right.clear();
        } else if current_left.is_empty() {
            current_left = line.into();
        } else {
            current_right = line.into();
            let left = parse_line(current_left.as_str())?;
            let right = parse_line(current_right.as_str())?;
            results.push((left, right));
        }
    }
    Ok(results)
}

fn parse_line(line: &str) -> Result<Value, String> {
    let (ov, errs) = parser().parse_recovery(line);
    if let Some(v) = ov {
        Ok(v)
    } else {
        Err(format!("{:?}", errs))
    }
}

fn run_part1(input: &[(Value, Value)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if left.cmp(right) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn run_part2(input: Vec<(Value, Value)>) -> usize {
    // flatten the values
    let divider1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let divider2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);
    let mut all_values: Vec<Value> = input
        .into_iter()
        .flat_map(|(left, right)| iter::once(left).chain(iter::once(right)))
        // add divider packets
        .chain(iter::once(divider1.clone()))
        .chain(iter::once(divider2.clone()))
        .collect();
    all_values.sort();
    let first_index = all_values
        .iter()
        .find_position(|v| **v == divider1)
        .unwrap()
        .0
        + 1;
    let second_index = all_values
        .iter()
        .find_position(|v| **v == divider2)
        .unwrap()
        .0
        + 1;

    first_index * second_index
}

#[test]
fn test_compare_values1() {
    order_test("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less);
}

#[test]
fn test_compare_values2() {
    order_test("[[1],[2,3,4]]", "[[1],4]", Ordering::Less);
}

#[test]
fn test_compare_values3() {
    order_test("[9]", "[[8,7,6]]", Ordering::Greater);
}
#[test]
fn test_compare_values4() {
    order_test("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less);
}
#[test]
fn test_compare_values5() {
    order_test("[7,7,7,7]", "[7,7,7]", Ordering::Greater);
}
#[test]
fn test_compare_values6() {
    order_test("[]", "[3]", Ordering::Less);
}
#[test]
fn test_compare_values7() {
    order_test("[[[]]]", "[[]]", Ordering::Greater);
}

#[test]
fn test_compare_values8() {
    order_test(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        Ordering::Greater,
    );
}

#[cfg(test)]
fn order_test(left: &str, right: &str, expected: Ordering) {
    let left = parser::parser().parse_recovery(left).0.unwrap();
    let right = parser::parser().parse_recovery(right).0.unwrap();

    assert_eq!(left.cmp(&right), expected);
}

#[test]
fn test_parse_input() {
    let input = "[1]
[2]

[3]
[4]";

    let result = parse_input_pairs(input).expect("This should parse");

    assert_eq!(
        result[0],
        (
            Value::List(vec![Value::Integer(1)]),
            Value::List(vec![Value::Integer(2)])
        )
    );
    assert_eq!(
        result[1],
        (
            Value::List(vec![Value::Integer(3)]),
            Value::List(vec![Value::Integer(4)])
        )
    );
}

#[cfg(test)]
const SAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[test]
fn test_sample_part1() {
    let pairs = parse_input_pairs(SAMPLE_INPUT).unwrap();
    let result = run_part1(&pairs);
    assert_eq!(result, 13);
}

#[test]
fn test_sample_part2() {
    let pairs = parse_input_pairs(SAMPLE_INPUT).unwrap();
    let result = run_part2(pairs);
    assert_eq!(result, 140);
}
