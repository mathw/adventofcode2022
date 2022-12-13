mod parser;
mod types;

use std::cmp::Ordering;

use chumsky::Parser;

use self::parser::parser;
use self::types::{Order, Value};
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
        Ok(DayResult::new(
            PartResult::Success(part1.to_string()),
            PartResult::NotImplemented,
        ))
    }
}

fn compare_integers(left: u32, right: u32) -> Option<Order> {
    match left.cmp(&right) {
        Ordering::Less => Some(Order::Correct),
        Ordering::Equal => None,
        Ordering::Greater => Some(Order::Incorrect),
    }
}

fn compare_lists<'a>(
    left: &mut impl Iterator<Item = &'a Value>,
    right: &mut impl Iterator<Item = &'a Value>,
) -> Option<Order> {
    match (left.next(), right.next()) {
        (Some(_), None) => Some(Order::Incorrect),
        (None, Some(_)) => Some(Order::Correct),
        (Some(lv), Some(rv)) => match compare_values(lv, rv) {
            None => compare_lists(left, right),
            o => o,
        },
        (None, None) => None,
    }
}

fn compare_values(left: &Value, right: &Value) -> Option<Order> {
    match (left, right) {
        (Value::List(l), Value::List(r)) => compare_lists(&mut l.iter(), &mut r.iter()),
        (Value::List(l), Value::Integer(r)) => {
            compare_lists(&mut l.iter(), &mut vec![Value::Integer(*r)].iter())
        }
        (Value::Integer(l), Value::List(r)) => {
            compare_lists(&mut vec![Value::Integer(*l)].iter(), &mut r.iter())
        }
        (Value::Integer(l), Value::Integer(r)) => compare_integers(*l, *r),
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
            if compare_values(left, right) == Some(Order::Correct) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_compare_values1() {
    order_test("[1,1,3,1,1]", "[1,1,5,1,1]", Some(Order::Correct));
}

#[test]
fn test_compare_values2() {
    order_test("[[1],[2,3,4]]", "[[1],4]", Some(Order::Correct));
}

#[test]
fn test_compare_values3() {
    order_test("[9]", "[[8,7,6]]", Some(Order::Incorrect));
}
#[test]
fn test_compare_values4() {
    order_test("[[4,4],4,4]", "[[4,4],4,4,4]", Some(Order::Correct));
}
#[test]
fn test_compare_values5() {
    order_test("[7,7,7,7]", "[7,7,7]", Some(Order::Incorrect));
}
#[test]
fn test_compare_values6() {
    order_test("[]", "[3]", Some(Order::Correct));
}
#[test]
fn test_compare_values7() {
    order_test("[[[]]]", "[[]]", Some(Order::Incorrect));
}

#[test]
fn test_compare_values8() {
    order_test(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        Some(Order::Incorrect),
    );
}

#[cfg(test)]
fn order_test(left: &str, right: &str, expected: Option<Order>) {
    let left = parser::parser().parse_recovery(left).0.unwrap();
    let right = parser::parser().parse_recovery(right).0.unwrap();

    assert_eq!(compare_values(&left, &right), expected);
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
