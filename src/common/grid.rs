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
        let start_index = self.index_of(0, y)?;
        let end_index = self.index_of(x, y)?;

        for item in &self.content[start_index..end_index] {
            if !predicate(item) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn all_right_of(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<bool, GridOperationError> {
        if x == self.max_x() {
            return Ok(true);
        }

        let start_index = self.index_of(x + 1, y)?;
        let end_index = self.index_of(self.max_x(), y)?;

        for item in &self.content[start_index..=end_index] {
            if !predicate(item) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn all_above(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<bool, GridOperationError> {
        for ty in 0..y {
            if !predicate(self.get(x, ty)?) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn all_below(
        &self,
        x: usize,
        y: usize,
        predicate: impl Fn(&T) -> bool,
    ) -> Result<bool, GridOperationError> {
        for ty in y + 1..self.max_y() {
            if !predicate(self.get(x, ty)?) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| self.get(x, y).expect("This should never fail"))
        })
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
    grid.set(0, 1, true).unwrap();
    grid.set(0, 2, true).unwrap();
    assert!(grid.all_below(0, 0, |i| *i).unwrap());
    assert!(!grid.all_below(0, 0, |i| !*i).unwrap());
    assert!(grid.all_below(0, 2, |i| *i).unwrap());
}

#[test]
fn test_iter() {
    let mut grid = Grid::new(2, 3);
    grid.set(0, 0, 1).unwrap();
    grid.set(1, 0, 2).unwrap();
    grid.set(0, 1, 3).unwrap();
    grid.set(1, 1, 4).unwrap();
    grid.set(0, 2, 5).unwrap();
    grid.set(1, 2, 6).unwrap();

    assert_eq!(
        grid.iter().cloned().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
}
