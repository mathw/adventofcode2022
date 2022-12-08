use std::{error::Error, fmt::Display, iter};

pub struct Grid<T> {
    content: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum GridOperationError {
    IndexOutOfBounds(usize, usize, usize, usize),
}

impl Error for GridOperationError {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}

impl Display for GridOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridOperationError::IndexOutOfBounds(x, y, width, height) => write!(
                f,
                "The given coordinates {},{} are not inside the {}x{} grid",
                x, y, width, height
            )?,
        }
        Ok(())
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            content: iter::repeat(T::default()).take(width * height).collect(),
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), GridOperationError> {
        let index = self.index_of(x, y)?;
        self.content[index] = value;
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&T, GridOperationError> {
        let index = self.index_of(x, y)?;
        self.content
            .get(index)
            .ok_or(GridOperationError::IndexOutOfBounds(
                x,
                y,
                self.width,
                self.height,
            ))
    }

    pub fn all_left_of(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<bool, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_left_of(x, y).all(predicate))
        }
    }

    pub fn all_right_of(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<bool, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_right_of(x, y).all(predicate))
        }
    }

    pub fn all_above(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<bool, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_above(x, y).all(predicate))
        }
    }

    pub fn all_below(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<bool, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_below(x, y).all(predicate))
        }
    }

    pub fn count_left_of(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<usize, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_left_of(x, y).filter(|i| predicate(i)).count())
        }
    }
    pub fn count_right_of(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<usize, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_right_of(x, y).filter(|i| predicate(i)).count())
        }
    }
    pub fn count_above(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<usize, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_above(x, y).filter(|i| predicate(i)).count())
        }
    }
    pub fn count_below(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<usize, GridOperationError> {
        if self.out_of_bounds(x, y) {
            Err(self.index_out_of_bounds(x, y))
        } else {
            Ok(self.iter_below(x, y).filter(|i| predicate(i)).count())
        }
    }

    fn index_out_of_bounds(&self, x: usize, y: usize) -> GridOperationError {
        GridOperationError::IndexOutOfBounds(x, y, self.width, self.height)
    }

    fn out_of_bounds(&self, x: usize, y: usize) -> bool {
        x > self.max_x() || y > self.max_y()
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }

    pub fn iter_coords_left_of(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..x).rev().map(move |x| (x, y))
    }

    pub fn iter_coords_right_of(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (x..=self.max_x()).skip(1).map(move |x| (x, y))
    }

    pub fn iter_coords_above(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..y).rev().map(move |y| (x, y))
    }

    pub fn iter_coords_below(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (y..=self.max_y()).skip(1).map(move |y| (x, y))
    }

    pub fn iter_left_of(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        self.iter_coords_left_of(x, y)
            .map(|(x, y)| self.get(x, y).unwrap())
    }
    pub fn iter_right_of(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        self.iter_coords_right_of(x, y)
            .map(|(x, y)| self.get(x, y).unwrap())
    }
    pub fn iter_above(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        self.iter_coords_above(x, y)
            .map(|(x, y)| self.get(x, y).unwrap())
    }
    pub fn iter_below(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        self.iter_coords_below(x, y)
            .map(|(x, y)| self.get(x, y).unwrap())
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    fn max_x(&self) -> usize {
        self.width - 1
    }
    fn max_y(&self) -> usize {
        self.height - 1
    }

    fn index_of(&self, x: usize, y: usize) -> Result<usize, GridOperationError> {
        if x >= self.width || y >= self.height {
            Err(GridOperationError::IndexOutOfBounds(
                x,
                y,
                self.width,
                self.height,
            ))
        } else {
            Ok(y * self.width + x)
        }
    }
}

#[test]
fn test_all_left_of() {
    let mut grid = Grid::new(3, 3);
    grid.set(0, 0, true).unwrap();
    grid.set(1, 0, true).unwrap();
    assert!(grid.all_left_of(2, 0, |i| *i).unwrap());
    assert!(!grid.all_left_of(2, 0, |i| !*i).unwrap());
    assert!(grid.all_left_of(0, 0, |i| *i).unwrap());
}

#[test]
fn test_all_right_of() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 0, true).unwrap();
    grid.set(2, 0, true).unwrap();
    assert!(grid.all_right_of(0, 0, |i| *i).unwrap());
    assert!(!grid.all_right_of(0, 0, |i| !*i).unwrap());
    assert!(grid.all_right_of(2, 0, |i| *i).unwrap());
}

#[test]
fn test_all_above() {
    let mut grid = Grid::new(3, 3);
    grid.set(0, 0, true).unwrap();
    grid.set(0, 1, true).unwrap();
    assert!(grid.all_above(0, 2, |i| *i).unwrap());
    assert!(!grid.all_above(0, 2, |i| !*i).unwrap());
    assert!(grid.all_above(0, 0, |i| *i).unwrap());
}

#[test]
fn test_all_below() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, 1).unwrap();
    grid.set(1, 2, 2).unwrap();
    assert!(grid.all_below(1, 0, |t| *t >= 1).unwrap());
}

#[test]
fn test_iter_coords() {
    let grid: Grid<bool> = Grid::new(2, 3);
    assert_eq!(
        grid.iter_coords().collect::<Vec<_>>(),
        vec![(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
    );
}
