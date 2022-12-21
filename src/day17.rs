use crate::common::day;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::Add;

pub fn run() -> day::Result {
    let height = run_n_cycles(include_str!("inputs/day17.txt"), 2022);
    let really_big_height = run_n_cycles(include_str!("inputs/day17.txt"), 1000000000000);
    Ok((
        Some(format!("Height is {}", height)),
        Some(format!(
            "Height after waiting for ages is {}",
            really_big_height
        )),
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
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
    fn new<const T: usize>(elements: [(i64, i64); T]) -> Self {
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
    pieces.into_iter()
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

#[derive(Eq, PartialEq)]
struct Chamber<const W: usize> {
    rocks: Vec<[bool; W]>,
    falling_rocks: HashSet<Point>,
    max_rock_height: i64,
    purge_offset: i64,
}

impl<const W: usize> Chamber<W> {
    fn new() -> Self {
        Self {
            rocks: Vec::with_capacity(20000),
            falling_rocks: HashSet::new(),
            max_rock_height: -1,
            purge_offset: 0,
        }
    }

    /// Drop the currently falling rock one step down
    /// Returns true if the rock has come to rest
    fn drop_rocks(&mut self) -> bool {
        if self.rock_should_stop_here() {
            // drain into a Vec so we don't try to mutably borrow ourselves twice at once
            // the compiler can't tell that add_stopped_rock doesn't affect falling_rocks, because it
            // doesn't do lifetime analysis beyond a function boundary
            let rocks_to_add = self.falling_rocks.drain().collect::<Vec<_>>();
            for rock in rocks_to_add {
                self.add_stopped_rock(rock);
            }
            true
        } else {
            self.falling_rocks = self.falling_rocks.iter().map(|p| p.below()).collect();
            false
        }
    }

    fn add_stopped_rock(&mut self, rock: Point) {
        if self.rocks.len() <= rock.y as usize {
            // ensure enough rows are in the Vec
            for _ in self.rocks.len()..=(rock.y as usize + 1) {
                self.rocks.push([false; W]);
            }
        }

        // update appropriate value
        self.rocks[rock.y as usize][rock.x as usize] = true;

        self.max_rock_height = i64::max(self.max_rock_height, rock.y);
    }

    fn rock_should_stop_here(&self) -> bool {
        self.falling_rocks
            .iter()
            .any(|p| self.rock_at_point(&p.below()) || p.y == 0)
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
            .any(|p| p.x < 0 || p.x >= W as i64 || self.rock_at_point(p))
        {
            self.falling_rocks = new_falling;
        }
    }

    fn add_rock(&mut self, piece: &Piece) {
        let start_pos = Point {
            x: 2,
            y: self.max_rock_height + 4,
        };
        self.falling_rocks
            .extend(piece.elements.iter().map(|p| *p + start_pos))
    }

    /// Optimise the chamber by removing all irrelevant rocks
    /// This will allow membership tests on the rock set to be much faster
    /// after many iterations
    fn prune(&mut self) {
        // first find the highest y such that all x have a rock in
        let mut y = self.max_rock_height;
        loop {
            if self.is_full_at(y) {
                break;
            }
            y -= 1;
            if y <= 0 {
                // no pruning is currently possible
                return;
            }
        }

        // remove everything below that y which we haven't
        // previously removed
        self.rocks.drain(0..(y as usize));

        self.purge_offset += y;
        self.max_rock_height -= y;
    }

    fn is_full_at(&self, y: i64) -> bool {
        self.get_at_y(y) == &[true; W]
    }

    fn get_at_y(&self, y: i64) -> &[bool; W] {
        self.rocks.get(y as usize).unwrap_or(&[false; W])
    }

    fn rock_at(&self, x: usize, y: i64) -> bool {
        self.get_at_y(y)[x]
    }

    fn rock_at_point(&self, p: &Point) -> bool {
        self.rock_at(p.x as usize, p.y)
    }

    fn rock_height(&self) -> u64 {
        if self.max_rock_height == -1 {
            0
        } else {
            self.max_rock_height as u64 + 1 + self.purge_offset as u64
        }
    }
}

impl<const W: usize> Display for Chamber<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_y = i64::max(
            self.max_rock_height,
            self.falling_rocks.iter().map(|p| p.y).max().unwrap_or(0),
        );
        writeln!(f, "Offset: {}", self.purge_offset)?;
        for y in (-1..=max_y).rev() {
            if y == -1 {
                write!(f, "     +")?;
                for _ in 0..W {
                    write!(f, "-")?;
                }
                writeln!(f, "+")?;
            } else {
                write!(f, "{:04} |", y + self.purge_offset)?;
                for x in 0..(W) {
                    if self.rock_at(x, y) {
                        write!(f, "#")?;
                    } else if self.falling_rocks.contains(&Point { x: x as i64, y }) {
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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Memo<const W: usize> {
    piece_index: usize,
    jet_index: usize,
    rocks: Vec<[bool; W]>,
}

#[derive(Debug)]
struct MemoData {
    added_height: u64,
}

struct MemoisedChamber<const W: usize> {
    memory: HashMap<Memo<W>, MemoData>,
    memo_hits: HashMap<Memo<W>, usize>,
    chamber: Chamber<W>,
    pieces: Vec<Piece>,
    jets: Vec<Jet>,
    current_piece: usize,
    current_jet: usize,
    rock_height: u64,
    is_repeating: bool,
    is_repeating_repeating: bool,
}

impl<const W: usize> MemoisedChamber<W> {
    fn new(pieces: Vec<Piece>, jets: Vec<Jet>) -> Self {
        Self {
            memory: HashMap::new(),
            chamber: Chamber::new(),
            pieces,
            jets,
            current_piece: 0,
            current_jet: 0,
            rock_height: 0,
            is_repeating: false,
            is_repeating_repeating: false,
            memo_hits: HashMap::new(),
        }
    }

    fn get_next_piece(&mut self) -> Piece {
        let piece = self.pieces[self.current_piece].clone();
        self.current_piece = (self.current_piece + 1) % self.pieces.len();
        piece
    }

    fn get_next_jet(&mut self) -> Jet {
        let jet = self.jets[self.current_jet];
        self.current_jet = (self.current_jet + 1) % self.jets.len();
        jet
    }

    fn run_rock(&mut self) -> u64 {
        let key = self.memo_key();
        if let Some(data) = self.memory.get(&key) {
            self.rock_height += data.added_height;
            if !self.is_repeating {
                println!("Started repeating");
            }
            self.is_repeating = true;
        } else {
            if self.is_repeating {
                panic!("Repetition cycle is broken");
            }
            self.run_novel_rock();
        }

        self.rock_height
    }

    fn run_novel_rock(&mut self) {
        let start_piece = self.current_piece;
        let start_jet = self.current_jet;
        let start_height = self.chamber.rock_height();
        let start_rocks = self.chamber.rocks.clone();
        let piece = self.get_next_piece();
        self.chamber.add_rock(&piece);
        loop {
            let jet = self.get_next_jet();
            self.chamber.apply_jet(jet);
            if self.chamber.drop_rocks() {
                self.chamber.prune();
                let added_height = self.chamber.rock_height() - start_height;
                let e = self.memory.insert(
                    Memo {
                        piece_index: start_piece,
                        jet_index: start_jet,
                        rocks: start_rocks,
                    },
                    MemoData { added_height },
                );
                self.rock_height += added_height;
                break;
            }
        }
    }

    fn memo_key(&self) -> Memo<W> {
        Memo {
            jet_index: self.current_jet,
            piece_index: self.current_piece,
            rocks: self.chamber.rocks.clone(),
        }
    }
}

fn run_n_cycles(input: &str, cycles: usize) -> u64 {
    let jets: Vec<Jet> = parse_jet(input).collect();
    let pieces: Vec<Piece> = pieces().collect();
    let mut chamber: MemoisedChamber<7> = MemoisedChamber::new(pieces, jets);
    let mut current_height = 0;
    let start_time = std::time::Instant::now();
    for cycle_count in 1..=cycles {
        if cycle_count % 1000000 == 0 {
            let time_remaining = ((start_time.elapsed() / cycle_count as u32) * cycles as u32)
                - start_time.elapsed();
            println!(
                "Done {} in {}s, {}s estimated remaining",
                cycle_count,
                start_time.elapsed().as_secs(),
                time_remaining.as_secs()
            );
        }
        current_height = chamber.run_rock()
    }
    current_height
}

#[test]
fn test_part1_sample() {
    assert_eq!(
        run_n_cycles(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022),
        3068
    );
}
