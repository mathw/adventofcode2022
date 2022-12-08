use std::error::Error;

use crate::common::grid::Grid;
use crate::day::{Day, DayResult, PartResult};

pub struct Day8 {
    input: &'static str,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            input: include_str!("inputs/day8.txt"),
        }
    }
}

impl Day for Day8 {
    fn run(&mut self) -> Result<DayResult, Box<dyn Error>> {
        let plantation = parse_input(self.input)?;
        let part1_visible_trees = count_visible_trees(&plantation);
        let part2_most_scenic_score = find_most_scenic_tree(&plantation)?;
        Ok(DayResult::new(
            PartResult::Success(format!("{} trees are visible", part1_visible_trees)),
            PartResult::Success(format!(
                "Most scenic tree score is {}",
                part2_most_scenic_score
            )),
        ))
    }
}

fn parse_input(input: &str) -> Result<Grid<u8>, Box<dyn Error>> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_height = chars.len();
    let grid_width = chars
        .get(0)
        .ok_or_else(|| "No content in input".to_owned())?
        .len();
    let mut grid = Grid::new(grid_width, grid_height);
    for y in 0..grid_height {
        for x in 0..grid_width {
            grid.set(
                x,
                y,
                u8_from_digit(chars.get(y).and_then(|line| line.get(x)).ok_or_else(|| {
                    format!("Input might not have been square: no info at {},{}", x, y)
                })?)?,
            )?
        }
    }
    Ok(grid)
}

fn u8_from_digit(c: &char) -> Result<u8, String> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(format!("{} is not a digit", c)),
    }
}

fn count_visible_trees(plantation: &Grid<u8>) -> usize {
    plantation
        .iter_coords()
        .filter(|(x, y)| tree_is_visible(plantation, *x, *y))
        .count()
}

fn tree_is_visible(plantation: &Grid<u8>, x: usize, y: usize) -> bool {
    let this_tree_height = plantation
        .get(x, y)
        .expect("If we got bad coords from iter_coords there is a bug in the Grid");
    let all_above = plantation
        .all_above(x, y, |t| t < this_tree_height)
        .expect("bug in grid");
    let all_below = plantation
        .all_below(x, y, |t| t < this_tree_height)
        .expect("bug in grid");
    let all_right_of = plantation
        .all_right_of(x, y, |t| t < this_tree_height)
        .expect("bug in grid");
    let all_left_of = plantation
        .all_left_of(x, y, |t| t < this_tree_height)
        .expect("bug in grid");

    all_above || all_below || all_right_of || all_left_of
}

fn scenic_score_of(plantation: &Grid<u8>, x: usize, y: usize) -> Result<u32, Box<dyn Error>> {
    let this_tree_height = plantation.get(x, y)?;
    let mut visible_left = plantation
        .iter_left_of(x, y)
        .take_while(|t| *t < this_tree_height)
        .count() as u32;
    if x as u32 - visible_left >= 1 {
        visible_left += 1;
    }

    let mut visible_right = plantation
        .iter_right_of(x, y)
        .take_while(|t| *t < this_tree_height)
        .count() as u32;
    if (plantation.width() - 1) as u32 - visible_right > x as u32 {
        visible_right += 1;
    }

    let mut visible_above = plantation
        .iter_above(x, y)
        .take_while(|t| *t < this_tree_height)
        .count() as u32;
    if y as u32 - visible_above >= 1 {
        visible_above += 1;
    }

    let mut visible_below = plantation
        .iter_below(x, y)
        .take_while(|t| *t < this_tree_height)
        .count() as u32;
    if (plantation.height() - 1) as u32 - visible_below > y as u32 {
        visible_below += 1;
    }

    Ok(visible_above * visible_below * visible_left * visible_right)
}

fn find_most_scenic_tree(plantation: &Grid<u8>) -> Result<u32, Box<dyn Error>> {
    Ok(plantation
        .iter_coords()
        .map(|(x, y)| scenic_score_of(plantation, x, y))
        .collect::<Result<Vec<u32>, _>>()?
        .into_iter()
        .max()
        .ok_or_else(|| "No trees".to_owned())?)
}

#[cfg(test)]
const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

#[test]
fn test_parse_input() {
    let input = "01
23";
    let parsed = parse_input(input).unwrap();
    assert_eq!(*parsed.get(0, 0).unwrap(), 0);
    assert_eq!(*parsed.get(1, 0).unwrap(), 1);
    assert_eq!(*parsed.get(0, 1).unwrap(), 2);
    assert_eq!(*parsed.get(1, 1).unwrap(), 3);
}

#[test]
fn test_visible_edges() {
    let grid = parse_input(
        "11
11",
    )
    .unwrap();
    assert_eq!(count_visible_trees(&grid), 4);
}

#[test]
fn test_part1_sample() {
    let grid = parse_input(TEST_INPUT).unwrap();
    let visible = count_visible_trees(&grid);
    assert_eq!(visible, 21);
}

#[test]
fn test_scenic_score() {
    let grid = parse_input(TEST_INPUT).unwrap();
    let score = scenic_score_of(&grid, 2, 3).unwrap();
    assert_eq!(score, 8);

    let score = scenic_score_of(&grid, 2, 1).unwrap();
    assert_eq!(score, 4);
}

#[test]
fn test_find_most_scenic() {
    let grid = parse_input(TEST_INPUT).unwrap();
    let most_scenic_score = find_most_scenic_tree(&grid).unwrap();
    assert_eq!(most_scenic_score, 8);
}
