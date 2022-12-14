use chumsky::prelude::*;
use core::num;
use itertools::Itertools;

use std::error::Error;

use crate::common::{
    day,
    grid::{Grid, GridOperationError},
};

pub struct Day14 {
    input: &'static str,
}

impl Day14 {
    pub fn new() -> Self {
        Self {
            input: include_str!("inputs/day14.txt"),
        }
    }
}

impl day::Day for Day14 {
    fn run(&mut self) -> day::Result {
        let part1 = run_part1(self.input)?;
        let part2 = run_part2(self.input)?;
        Ok((
            Some(format!("{} sand have come to rest", part1)),
            Some(format!(
                "{} sand have come to rest and the source is blocked",
                part2
            )),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path(Vec<(usize, usize)>);

fn parse_path(input: &str) -> Result<Path, Box<dyn Error>> {
    let number = text::digits::<_, Simple<char>>(10)
        .from_str::<usize>()
        .unwrapped::<usize, num::ParseIntError>()
        .labelled("number");
    let pair = number
        .then_ignore(just(','))
        .chain(number)
        .map(|v| (v[0], v[1]))
        .labelled("pair");
    let arrow = just("->").labelled("arrow");
    let path = pair.padded().separated_by(arrow).labelled("path");

    path.parse(input)
        .map(Path)
        .map_err(|es| es.into_iter().map(|e| e.to_string()).join("\n").into())
}

#[test]
fn test_parse_path() {
    assert_eq!(
        parse_path("4,5 -> 7,6 -> 9,2").unwrap(),
        Path(vec![(4, 5), (7, 6), (9, 2)])
    );
}

fn parse_input(input: &str) -> Result<Vec<Path>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| parse_path(line.trim()))
        .collect::<Result<_, _>>()
}

impl Path {
    fn expand(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.0.iter().cloned().take(1).chain(
            self.0
                .iter()
                .tuple_windows()
                .flat_map(|(s, e)| expand_path_segment(s, e).skip(1)),
        )
    }

    fn max_x(&self) -> usize {
        self.0.iter().map(|(x, _)| *x).max().unwrap_or(0)
    }

    fn max_y(&self) -> usize {
        self.0.iter().map(|(_, y)| *y).max().unwrap_or(0)
    }
}

fn expand_path_segment<'a>(
    start: &'a (usize, usize),
    end: &'a (usize, usize),
) -> Box<dyn Iterator<Item = (usize, usize)> + 'a> {
    if start.0 < end.0 {
        // this path is horizontal and goes left to right
        Box::new((start.0..=end.0).map(|x| (x, start.1)))
    } else if end.0 < start.0 {
        // this path is horizontal and goes right to left
        Box::new((end.0..=start.0).map(|x| (x, start.1)).rev())
    } else if start.1 < end.1 {
        // this path is vertical and goes bottom to top
        Box::new((start.1..=end.1).map(|y| (start.0, y)))
    } else if end.1 < start.1 {
        Box::new((end.1..=start.1).map(|y| (start.0, y)).rev())
    } else {
        Box::new(std::iter::empty())
    }
}

#[test]
fn test_expand_path() {
    let path = Path(vec![(0, 0), (1, 0), (1, 4)]);
    let expanded = path.expand().collect::<Vec<_>>();
    assert_eq!(
        expanded,
        vec![(0, 0), (1, 0), (1, 1), (1, 2), (1, 3), (1, 4)]
    );

    let path = Path(vec![(498, 4), (498, 6), (496, 6)]);
    let expanded = path.expand().collect::<Vec<_>>();
    assert_eq!(
        expanded,
        vec![(498, 4), (498, 5), (498, 6), (497, 6), (496, 6)]
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Air,
    Rock,
    Sand,
}

impl Default for Space {
    fn default() -> Self {
        Space::Air
    }
}

use self::Space::*;

fn add_path_to_grid(path: &Path, grid: &mut Grid<Space>) -> Result<(), GridOperationError> {
    for (x, y) in path.expand() {
        grid.set(x, y, Rock)?;
    }
    Ok(())
}

fn build_grid(paths: &[Path], with_floor: bool) -> Result<Grid<Space>, GridOperationError> {
    let width =
        paths.iter().map(|p| p.max_x()).max().unwrap_or(0) + if with_floor { 100 } else { 1 };
    let height =
        paths.iter().map(|p| p.max_y()).max().unwrap_or(0) + if with_floor { 3 } else { 1 };

    let mut grid = Grid::new(width, height);

    for path in paths {
        add_path_to_grid(path, &mut grid)?
    }

    if with_floor {
        // I'm going to assume there are at least 400 x off to the left to play with
        // Sample input and my own seem to support this
        add_path_to_grid(
            &Path(vec![(0, height - 1), (width - 1, height - 1)]),
            &mut grid,
        )?;
    }

    Ok(grid)
}

#[cfg(test)]
fn render_grid(grid: &Grid<Space>) -> String {
    let mut lines: Vec<String> = Vec::new();
    for y in 0..grid.height() {
        let mut line = Vec::new();
        for x in 0..grid.width() {
            if x == 500 && y == 0 {
                line.push('+')
            } else {
                line.push(match *grid.get(x, y).unwrap_or(&Air) {
                    Air => '.',
                    Rock => '#',
                    Sand => 'o',
                })
            }
        }
        lines.push(line.into_iter().collect());
    }
    lines.into_iter().join("\n")
}

#[test]
fn test_grid_build() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let paths = parse_input(input).expect("Input should parse");
    let grid = build_grid(&paths, false).expect("Grid should build");
    let rendered = render_grid(&grid);
    let rendered = rendered.lines().map(|l| &l[494..]).join("\n");
    assert_eq!(
        rendered.as_str(),
        "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########."
    )
}

enum StepResult {
    Try(usize, usize),
    Stop,
    Void,
}

fn fill_sand(grid: &mut Grid<Space>) -> Result<usize, GridOperationError> {
    let mut sand_quantity = count_sand(grid);
    loop {
        if grid.get(500, 0)? == &Sand {
            // cave is full
            return Ok(sand_quantity);
        }

        drop_sand((500, 0), grid)?;
        let new_sand_quantity = count_sand(grid);
        if new_sand_quantity == sand_quantity {
            return Ok(new_sand_quantity);
        }
        sand_quantity = new_sand_quantity;
    }
}

fn count_sand(grid: &Grid<Space>) -> usize {
    grid.iter_values().filter(|v| v == &&Sand).count()
}

fn drop_sand(sand: (usize, usize), grid: &mut Grid<Space>) -> Result<(), GridOperationError> {
    match step_sand(sand, grid)? {
        StepResult::Try(x, y) => drop_sand((x, y), grid),
        StepResult::Stop => {
            grid.set(sand.0, sand.1, Sand)?;
            Ok(())
        }
        StepResult::Void => Ok(()),
    }
}

fn step_sand(sand: (usize, usize), grid: &Grid<Space>) -> Result<StepResult, GridOperationError> {
    use StepResult::*;
    let below = (sand.0, sand.1 + 1);
    if below.1 == grid.height() {
        return Ok(Void);
    }

    match grid.get(below.0, below.1)? {
        Air => Ok(Try(below.0, below.1)),
        _ => {
            // try to go left first
            if below.0 == 0 {
                // falls into the void
                Ok(Void)
            } else {
                let below_left = (below.0 - 1, below.1);
                if grid.get(below_left.0, below_left.1)? == &Air {
                    Ok(Try(below_left.0, below_left.1))
                } else {
                    let below_right = (below.0 + 1, below.1);
                    if below_right.0 == grid.width() {
                        Ok(Void)
                    } else if grid.get(below_right.0, below_right.1)? == &Air {
                        Ok(Try(below_right.0, below_right.1))
                    } else {
                        Ok(Stop)
                    }
                }
            }
        }
    }
}

fn run_part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut grid = build_grid(&parse_input(input)?, false)?;
    let sand = fill_sand(&mut grid)?;
    Ok(sand)
}

fn run_part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut grid = build_grid(&parse_input(input)?, true)?;
    let sand = fill_sand(&mut grid)?;
    Ok(sand)
}

#[test]
fn test_part1_sample() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let sand = run_part1(input).expect("No errors");
    assert_eq!(sand, 24);
}

#[test]
fn test_drop_five_sand() {
    let rendered = drop_n_sand(5);
    assert_eq!(
        rendered.as_str(),
        "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########."
    )
}

#[test]
fn test_drop_22_sand() {
    let rendered = drop_n_sand(22);
    assert_eq!(
        rendered.as_str(),
        "......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########."
    )
}

#[test]
fn test_drop_24_sand() {
    let rendered = drop_n_sand(24);
    assert_eq!(
        rendered.as_str(),
        "......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########."
    )
}

#[cfg(test)]
fn drop_n_sand(n: usize) -> String {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let paths = parse_input(input).expect("Input should parse");
    let mut grid = build_grid(&paths, false).expect("Grid should build");
    for _ in 0..n {
        drop_sand((500, 0), &mut grid).expect("No errors");
    }
    let rendered = render_grid(&grid);
    let rendered = rendered.lines().map(|l| &l[494..]).join("\n");
    rendered
}
