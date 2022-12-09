use std::{cmp::Ordering, collections::HashSet, error::Error, str::FromStr};

use crate::day::{Day, DayResult, PartResult};

pub struct Day9 {
    input: &'static str,
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            input: include_str!("inputs/day9.txt"),
        }
    }
}

impl Day for Day9 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let steps = parse_input(self.input)?;
        let visited = run_part1(steps.into_iter());
        Ok(DayResult::new(
            PartResult::Success(format!("{} locations were visited by the tail", visited)),
            PartResult::NotImplemented,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Step {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

impl Move {
    fn to_step(self) -> Step {
        match self {
            Move::Right(_) => Step::Right,
            Move::Left(_) => Step::Left,
            Move::Up(_) => Step::Up,
            Move::Down(_) => Step::Down,
        }
    }
    fn count(self) -> usize {
        match self {
            Move::Right(c) => c,
            Move::Left(c) => c,
            Move::Up(c) => c,
            Move::Down(c) => c,
        }
    }
    fn expand(&self) -> impl Iterator<Item = Step> {
        std::iter::repeat(self.to_step()).take(self.count())
    }
}

fn parse_input_line(line: &str) -> Result<Move, Box<dyn Error>> {
    if let Some(c) = line.strip_prefix("L ") {
        Ok(Move::Left(usize::from_str(c)?))
    } else if let Some(c) = line.strip_prefix("R ") {
        Ok(Move::Right(usize::from_str(c)?))
    } else if let Some(c) = line.strip_prefix("U ") {
        Ok(Move::Up(usize::from_str(c)?))
    } else if let Some(c) = line.strip_prefix("D ") {
        Ok(Move::Down(usize::from_str(c)?))
    } else {
        Err(format!("Input string '{}' was not recognised", line).into())
    }
}

fn parse_input(input: &str) -> Result<Vec<Step>, Box<dyn Error>> {
    let mut steps = Vec::new();
    for line in input.lines() {
        let m = parse_input_line(line)?;
        for s in m.expand() {
            steps.push(s);
        }
    }
    Ok(steps)
}

fn run_part1(steps: impl Iterator<Item = Step>) -> usize {
    let mut rope = Rope::new(0, 0, 0, 0);
    let mut tail_visited = HashSet::new();
    tail_visited.insert(rope.tail);

    for step in steps {
        rope.apply_step(step);
        tail_visited.insert(rope.tail);
    }

    tail_visited.len()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn expand_move(&self, m: Move) -> impl Iterator<Item = Self> {
        let mut loc = self.clone();
        m.expand().map(move |s| {
            loc = loc.apply_step(s);
            loc.clone()
        })
    }

    fn apply_step(&self, s: Step) -> Self {
        match s {
            Step::Left => Location {
                x: self.x - 1,
                y: self.y,
            },
            Step::Right => Location {
                x: self.x + 1,
                y: self.y,
            },
            Step::Up => Location {
                x: self.x,
                y: self.y - 1,
            },
            Step::Down => Location {
                x: self.x,
                y: self.y + 1,
            },
        }
    }

    fn touches(&self, other: &Self) -> bool {
        if self.x == other.x {
            if self.y == other.y {
                true
            } else {
                (self.y - other.y).abs() <= 1
            }
        } else if self.y == other.y {
            (self.x - other.x).abs() <= 1
        } else {
            (self.y - other.y).abs() <= 1 && (self.x - other.x).abs() <= 1
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rope {
    head: Location,
    tail: Location,
}

impl Rope {
    fn new(hx: isize, hy: isize, tx: isize, ty: isize) -> Self {
        Self {
            head: Location::new(hx, hy),
            tail: Location::new(tx, ty),
        }
    }

    fn apply_step(&mut self, step: Step) {
        self.head = self.head.apply_step(step);
        self.correct_tail();
    }

    fn correct_tail(&mut self) {
        if self.head.touches(&self.tail) {
            return;
        }
        match (self.head.x.cmp(&self.tail.x), self.head.y.cmp(&self.tail.y)) {
            (Ordering::Less, Ordering::Less) => {
                self.tail.x -= 1;
                self.tail.y -= 1;
            }
            (Ordering::Less, Ordering::Equal) => {
                if self.tail.x - self.head.x > 1 {
                    self.tail.x -= 1
                }
            }
            (Ordering::Less, Ordering::Greater) => {
                self.tail.x -= 1;
                self.tail.y += 1;
            }
            (Ordering::Equal, Ordering::Less) => self.tail.y -= 1,
            (Ordering::Equal, Ordering::Equal) => {}
            (Ordering::Equal, Ordering::Greater) => self.tail.y += 1,
            (Ordering::Greater, Ordering::Less) => {
                self.tail.x += 1;
                self.tail.y -= 1;
            }
            (Ordering::Greater, Ordering::Equal) => self.tail.x += 1,
            (Ordering::Greater, Ordering::Greater) => {
                self.tail.x += 1;
                self.tail.y += 1;
            }
        }
    }
}

#[test]
fn test_correct_tail() {
    let mut rope = Rope::new(2, 1, 1, 1);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(2, 1, 1, 1),
        "same y, x one apart, no change"
    );

    let mut rope = Rope::new(3, 1, 1, 1);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(3, 1, 2, 1),
        "same y, x two apart, moves to one apart"
    );

    let mut rope = Rope::new(3, 1, 2, 1);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(3, 1, 2, 1),
        "same y, x one apart, no change"
    );

    let mut rope = Rope::new(1, 2, 1, 1);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(1, 2, 1, 1),
        "same x, y one apart, no change"
    );

    let mut rope = Rope::new(1, 3, 1, 1);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(1, 3, 1, 2),
        "same x, y two apart, y changes"
    );

    let mut rope = Rope::new(2, 2, 1, 3);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(2, 2, 1, 3),
        "diagonally touching, no change"
    );

    let mut rope = Rope::new(2, 1, 1, 3);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(2, 1, 2, 2),
        "diagonally apart, tail moves X and Y"
    );

    let mut rope = Rope::new(3, 2, 1, 3);
    rope.correct_tail();
    assert_eq!(
        rope,
        Rope::new(3, 2, 2, 2),
        "diagonally apart, tail moves X and Y #2"
    );
}

#[test]
fn test_part1_sample() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let visited = run_part1(parse_input(input).unwrap().into_iter());
    assert_eq!(visited, 13);
}
