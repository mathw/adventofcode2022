use super::types::Value;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Value, Error = Simple<char>> {
    recursive(|value| {
        let number = text::digits(10)
            .from_str()
            .unwrapped()
            .map(Value::Integer)
            .labelled("integer");

        let list = value
            .clone()
            .chain(just(',').ignore_then(value.clone()).repeated())
            .or_not()
            .flatten()
            .delimited_by(just('['), just(']'))
            .map(Value::List)
            .labelled("list");

        number.or(list)
    })
}

#[test]
fn test_single_element_list() {
    let input = "[1]";
    let (value, _) = parser().parse_recovery(input);
    assert_eq!(value, Some(Value::List(vec![Value::Integer(1)])))
}

#[test]
fn test_bigger_list() {
    let input = "[1,2]";
    let (value, _) = parser().parse_recovery(input);
    assert_eq!(
        value,
        Some(Value::List(vec![Value::Integer(1), Value::Integer(2)]))
    )
}

#[test]
fn test_nested_list() {
    let input = "[[1,1],2]";
    let (value, _) = parser().parse_recovery(input);
    assert_eq!(
        value,
        Some(Value::List(vec![
            Value::List(vec![Value::Integer(1), Value::Integer(1)]),
            Value::Integer(2)
        ]))
    )
}
