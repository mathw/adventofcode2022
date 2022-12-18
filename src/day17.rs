use crate::common::day;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Add;

pub fn run() -> day::Result {
    let height = run_part1(include_str!("inputs/day17.txt"), 2022);
    Ok((Some(format!("Height is {}", height)), None))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn below(&self) -> Point {
        Point {
            y: self.y - 1,
            ..*self
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            ..*self
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            ..*self
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
struct Piece {
    elements: HashSet<Point>,
}

impl Piece {
    fn new<const T: usize>(elements: [(i32, i32); T]) -> Self {
        Self {
            elements: elements.into_iter().map(|(x, y)| Point { x, y }).collect(),
        }
    }
}

fn dash() -> Piece {
    Piece::new([(0, 0), (1, 0), (2, 0), (3, 0)])
}

fn plus() -> Piece {
    Piece::new([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)])
}

fn bent() -> Piece {
    Piece::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)])
}

fn long() -> Piece {
    Piece::new([(0, 0), (0, 1), (0, 2), (0, 3)])
}

fn square() -> Piece {
    Piece::new([(0, 0), (1, 0), (0, 1), (1, 1)])
}

fn pieces() -> impl Iterator<Item = Piece> {
    let pieces = vec![dash(), plus(), bent(), long(), square()];
    repeating(pieces.into_iter())
}

fn repeating<T: Clone>(sequence: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let mut current = 0;
    let sequence: Vec<T> = sequence.collect();

    std::iter::from_fn(move || {
        let item = sequence[current].clone();

        current = (current + 1) % sequence.len();

        Some(item)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Jet {
    Left,
    Right,
}

fn parse_jet(input: &str) -> impl Iterator<Item = Jet> + '_ {
    input.chars().filter_map(|c| match c {
        '<' => Some(Jet::Left),
        '>' => Some(Jet::Right),
        _ => None,
    })
}

struct Chamber {
    rocks: HashSet<Point>,
    falling_rocks: HashSet<Point>,
    width: usize,
}

impl Chamber {
    fn new() -> Self {
        Self {
            rocks: HashSet::new(),
            falling_rocks: HashSet::new(),
            width: 7,
        }
    }

    fn max_rock_height(&self) -> i32 {
        self.rocks.iter().map(|p| p.y).max().unwrap_or(-1)
    }

    /// Drop the currently falling rock one step down
    /// Returns true if the rock has come to rest
    fn drop_rocks(&mut self) -> bool {
        if self.rock_should_stop_here() {
            self.rocks.extend(self.falling_rocks.drain());
            true
        } else {
            self.falling_rocks = self.falling_rocks.iter().map(|p| p.below()).collect();
            false
        }
    }

    fn rock_should_stop_here(&self) -> bool {
        self.falling_rocks
            .iter()
            .any(|p| self.rocks.contains(&p.below()) || p.y == 0)
    }

    fn apply_jet(&mut self, jet: Jet) {
        let new_falling: HashSet<Point> = self
            .falling_rocks
            .iter()
            .map(|p| match jet {
                Jet::Left => p.left(),
                Jet::Right => p.right(),
            })
            .collect();

        // ensure that the new position of the rock doesn't intersect the walls or any fallen rocks
        if !new_falling
            .iter()
            .any(|p| p.x < 0 || p.x >= self.width as i32 || self.rocks.contains(p))
        {
            self.falling_rocks = new_falling;
        }
    }

    fn add_rock(&mut self, piece: &Piece) {
        let start_pos = Point {
            x: 2,
            y: self.max_rock_height() + 4,
        };
        self.falling_rocks
            .extend(piece.elements.iter().map(|p| *p + start_pos))
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_y = i32::max(
            self.max_rock_height(),
            self.falling_rocks.iter().map(|p| p.y).max().unwrap_or(0),
        );
        for y in (-1..=max_y).rev() {
            if y == -1 {
                writeln!(f, "+-------+")?;
            } else {
                write!(f, "|")?;
                for x in 0..(self.width as i32) {
                    if self.rocks.contains(&Point { x, y }) {
                        write!(f, "#")?;
                    } else if self.falling_rocks.contains(&Point { x, y }) {
                        write!(f, "@")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f, "|")?;
            }
        }
        Ok(())
    }
}

fn run_part1(input: &str, rocks: usize) -> usize {
    let mut jets = repeating(parse_jet(input));
    let mut chamber = Chamber::new();
    'rock: for piece in pieces().take(rocks) {
        chamber.add_rock(&piece);
        loop {
            chamber.apply_jet(jets.next().expect("This is an infinite iterator"));
            if chamber.drop_rocks() {
                // this rock is done, do the next
                continue 'rock;
            }
        }
    }
    (chamber.max_rock_height() + 1) as usize
}

#[test]
fn test_part1_sample() {
    assert_eq!(
        run_part1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022),
        3068
    );
}
