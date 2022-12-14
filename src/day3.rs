use crate::common::day;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day3 {
    input: &'static str,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            input: include_str!("inputs/day3.txt"),
        }
    }
}

impl day::Day for Day3 {
    fn run(&mut self) -> day::Result {
        let part1_result = duplicated_item_priority_sum(self.input);
        let part2_result = sum_of_group_badge_priorities(self.input);
        Ok((
            Some(format!(
                "Sum of duplicate item priorities is {}",
                part1_result
            )),
            Some(format!("Sum of group badge priorities is {}", part2_result)),
        ))
    }
}

fn split_rucksack(rucksack: &str) -> Option<(&str, &str)> {
    let len = rucksack.len();
    if len % 2 != 0 {
        return None;
    }
    Some((&rucksack[0..len / 2], &rucksack[len / 2..]))
}

fn find_duplicate(compartments: (&str, &str)) -> Option<char> {
    let first = compartments.0.chars().collect::<HashSet<_>>();
    let second = compartments.1.chars().collect::<HashSet<_>>();
    let overlap = first.intersection(&second);
    overlap.cloned().next()
}

fn duplicates_in_input(input: &str) -> impl Iterator<Item = char> + '_ {
    input
        .lines()
        .filter_map(|l| split_rucksack(l.trim()))
        .filter_map(find_duplicate)
}

fn item_priority(i: char) -> u8 {
    match i {
        x if ('a'..='z').contains(&x) => (i as u8 - b'a') + 1,
        x if ('A'..='Z').contains(&x) => (i as u8 - b'A') + 27,
        _ => 0,
    }
}

fn duplicated_item_priority_sum(input: &str) -> u32 {
    duplicates_in_input(input)
        .map(|i| item_priority(i) as u32)
        .sum()
}

fn groups(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| c.collect())
        .collect()
}

fn find_group_badge(group: &[&str]) -> Option<char> {
    let mut elves = group.iter().map(|g| g.chars().collect::<HashSet<char>>());
    let mut intersection = elves.next()?;
    for elf in elves {
        intersection = elf.intersection(&intersection).cloned().collect();
    }
    intersection.into_iter().next()
}

fn sum_of_group_badge_priorities(input: &str) -> u32 {
    groups(input)
        .into_iter()
        .filter_map(|group| find_group_badge(&group))
        .map(|badge| item_priority(badge) as u32)
        .sum()
}

#[test]
fn test_split_rucksack() {
    assert_eq!(split_rucksack("abcd"), Some(("ab", "cd")));
    assert_eq!(split_rucksack("abcde"), None);
}

#[test]
fn test_find_duplicate() {
    assert_eq!(find_duplicate(("aa", "bb")), None);
    assert_eq!(find_duplicate(("aa", "ab")), Some('a'));
}

#[test]
fn test_find_all_duplicates() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let duplicates = duplicates_in_input(input).collect::<Vec<_>>();
    assert_eq!(duplicates, vec!['p', 'L', 'P', 'v', 't', 's']);
}

#[test]
fn test_priority() {
    assert_eq!(item_priority('a'), 1);
    assert_eq!(item_priority('A'), 27);
    assert_eq!(item_priority('Z'), 52);
}

#[test]
fn test_part1() {
    assert_eq!(
        duplicated_item_priority_sum(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        ),
        157
    );
}

#[test]
fn test_find_group_badge() {
    let group = vec!["ab", "ac", "dDa"];
    assert_eq!(find_group_badge(&group), Some('a'));
}

#[test]
fn test_part2() {
    assert_eq!(
        sum_of_group_badge_priorities(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        ),
        70
    );
}
