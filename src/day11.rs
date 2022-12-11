use crate::day::{Day, DayResult, PartResult};
use std::collections::HashMap;
pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day for Day11 {
    fn run(&mut self) -> crate::day::Result {
        let mut monkeys = input();
        run_rounds(&mut monkeys, 20, false);
        let part1_result = monkey_business(&monkeys);
        let mut monkeys = input();
        run_rounds(&mut monkeys, 10000, true);
        let part2_result = monkey_business(&monkeys);
        Ok(DayResult::new(
            PartResult::Success(format!("Monkey business is {}", part1_result)),
            PartResult::Success(format!("Monkey business is {}", part2_result)),
        ))
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspects: usize,
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: Box<dyn Fn(u64) -> u64>,
        test: u64,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspects: 0,
        }
    }

    fn operation(&self, w: u64) -> u64 {
        (self.operation)(w)
    }

    fn test(&self, w: u64) -> bool {
        w % self.test == 0
    }

    fn add_item(&mut self, w: u64) {
        self.items.push(w)
    }

    fn inspect(&mut self) {
        self.inspects += 1;
    }

    fn times_inspected(&self) -> usize {
        self.inspects
    }
}

fn run_monkey(
    id: usize,
    monkeys: &mut HashMap<usize, Monkey>,
    worry_management_factor: Option<u64>,
) {
    // remove the monkey from the HashMap so that we can get a mutable borrow
    // later to update other monkeys
    let mut monkey = monkeys.remove(&id).expect("Expect valid monkey ID");
    let items = monkey.items.clone();
    for item in items {
        monkey.inspect();
        let item = monkey.operation(item);
        let item = if let Some(factor) = worry_management_factor {
            item % factor
        } else {
            item / 3
        };
        if monkey.test(item) {
            monkeys
                .get_mut(&monkey.if_true)
                .expect("Expect valid if_true")
                .add_item(item);
        } else {
            monkeys
                .get_mut(&monkey.if_false)
                .expect("Expect valid if_false")
                .add_item(item);
        }
    }
    monkey.items.clear();
    // make sure we put the monkey back in the HashMap!
    monkeys.insert(id, monkey);
}

fn run_monkeys(monkeys: &mut HashMap<usize, Monkey>, worry_management_factor: Option<u64>) {
    let mut ids: Vec<usize> = monkeys.keys().copied().collect();
    ids.sort();
    for id in ids {
        run_monkey(id, monkeys, worry_management_factor);
    }
}

fn run_rounds(monkeys: &mut HashMap<usize, Monkey>, rounds: usize, part2_logic: bool) {
    // To manage worry in part 2, we multiply all the monkey's test divisors together
    // and use the modulus of that with each item's worry level as an adjusted value after
    // each time the monkey considers an item. This ensures all the divisibility tests for
    // every monkey will still pass where they would if we were using arbitrarily-large integers
    // This problem is designed to make sure we figure out something like this, because
    // it will overflow even a u128 without some sort of adjustment.
    let worry_management_factor = if part2_logic {
        Some(monkeys.values().map(|m| m.test).product())
    } else {
        None
    };
    for _ in 0..rounds {
        run_monkeys(monkeys, worry_management_factor);
    }
}

fn most_active_monkeys<'a>(
    monkeys: impl Iterator<Item = &'a Monkey>,
) -> impl Iterator<Item = &'a Monkey> {
    let mut monkeys: Vec<&Monkey> = monkeys.collect();
    monkeys.sort_by_key(|m| m.times_inspected());
    monkeys.reverse();
    monkeys.into_iter()
}

fn monkey_business(monkeys: &HashMap<usize, Monkey>) -> usize {
    most_active_monkeys(monkeys.values())
        .take(2)
        .map(|m| m.times_inspected())
        .product()
}

macro_rules! monkey {
    ($items:expr, $observe:expr, $divisor:expr, $if_true:expr, $if_false: expr) => {
        Monkey::new($items, Box::new($observe), $divisor, $if_true, $if_false)
    };
}

#[cfg(not(test))]
fn input() -> HashMap<usize, Monkey> {
    HashMap::from([
        (
            0,
            monkey!(vec![89, 95, 92, 64, 87, 68], |old| old * 11, 2, 7, 4),
        ),
        (1, monkey!(vec![87, 67], |old| old + 1, 13, 3, 6)),
        (2, monkey!(vec![95, 79, 92, 82, 60], |old| old + 6, 3, 1, 6)),
        (3, monkey!(vec![67, 97, 56], |old| old * old, 17, 7, 0)),
        (
            4,
            monkey!(
                vec![80, 68, 87, 94, 61, 59, 50, 68],
                |old| old * 7,
                19,
                5,
                2
            ),
        ),
        (5, monkey!(vec![73, 51, 76, 59], |old| old + 8, 7, 2, 1)),
        (6, monkey!(vec![92], |old| old + 5, 11, 3, 0)),
        (
            7,
            monkey!(vec![99, 76, 78, 76, 79, 90, 89], |old| old + 7, 5, 4, 5),
        ),
    ])
}

#[cfg(test)]
fn input() -> HashMap<usize, Monkey> {
    HashMap::from([
        (0, monkey!(vec![79, 98], |old| old * 19, 23, 2, 3)),
        (1, monkey!(vec![54, 65, 75, 74], |old| old + 6, 19, 2, 0)),
        (2, monkey!(vec![79, 60, 97], |old| old * old, 13, 1, 3)),
        (3, monkey!(vec![74], |old| old + 3, 17, 0, 1)),
    ])
}

#[test]
fn test_run_monkey_0() {
    let mut monkeys = input();
    run_monkey(0, &mut monkeys, None);
    let monkey0 = monkeys
        .get(&0)
        .expect("Monkey 0 should be back in the HashMap");
    assert!(monkey0.items.is_empty(), "Monkey 0 should have no items");
    let monkey3 = monkeys.get(&3).expect("Monkey 3 should exist");
    assert_eq!(
        monkey3.items,
        vec![74, 500, 620],
        "Monkey 3 should have received two items"
    );
}

#[test]
fn test_round_1() {
    let mut monkeys = input();
    run_monkeys(&mut monkeys, None);
    assert_eq!(
        monkeys.get(&0).expect("Monkey 0 should exist").items,
        vec![20, 23, 27, 26],
        "Monkey 0 items should be correct"
    );
    assert_eq!(
        monkeys.get(&1).expect("Monkey 1 should exist").items,
        vec![2080, 25, 167, 207, 401, 1046],
        "Monkey 1 items should be correct"
    );
    assert!(
        monkeys
            .get(&2)
            .expect("Monkey 2 should exist")
            .items
            .is_empty(),
        "Monkey 2 should have no items"
    );
    assert!(
        monkeys
            .get(&3)
            .expect("Monkey 3 should exist")
            .items
            .is_empty(),
        "Monkey 3 should have no items"
    );
}

#[test]
fn test_part_1() {
    let mut monkeys = input();
    run_rounds(&mut monkeys, 20, false);
    assert_eq!(monkey_business(&monkeys), 10605);
}

#[test]
fn test_part_2() {
    let mut monkeys = input();
    run_rounds(&mut monkeys, 10000, true);
    assert_eq!(monkey_business(&monkeys), 2713310158);
}

#[test]
fn test_part2_one_round() {
    let mut monkeys = input();
    run_rounds(&mut monkeys, 1, true);
    assert_eq!(monkeys[&0].times_inspected(), 2);
    assert_eq!(monkeys[&1].times_inspected(), 4);
    assert_eq!(monkeys[&2].times_inspected(), 3);
    assert_eq!(monkeys[&3].times_inspected(), 6);
}

#[test]
fn test_part2_twenty_rounds() {
    let mut monkeys = input();
    run_rounds(&mut monkeys, 20, true);
    assert_eq!(monkeys[&0].times_inspected(), 99);
    assert_eq!(monkeys[&1].times_inspected(), 97);
    assert_eq!(monkeys[&2].times_inspected(), 8);
    assert_eq!(monkeys[&3].times_inspected(), 103);
}

#[test]
fn test_part2_ten_thousand_rounds() {
    let mut monkeys = input();
    run_rounds(&mut monkeys, 10000, true);
    assert_eq!(monkeys[&0].times_inspected(), 52166);
    assert_eq!(monkeys[&1].times_inspected(), 47830);
    assert_eq!(monkeys[&2].times_inspected(), 1938);
    assert_eq!(monkeys[&3].times_inspected(), 52013);
}
