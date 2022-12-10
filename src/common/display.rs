use super::grid::{self, Grid};
use std::fmt;

/// A simple wrapper around a Grid<bool> specialised to be a monochrome display screen
pub struct Display {
    grid: Grid<bool>,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: bool) -> Result<(), grid::GridOperationError> {
        self.grid.set(x, y, pixel)
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                write!(
                    f,
                    "{}",
                    if *self.grid.get(x, y).map_err(|_| fmt::Error {})? {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_display() {
    let mut display = Display::new(3, 2);
    let _ = display.set(1, 1, true);
    let _ = display.set(2, 1, true);

    assert_eq!(
        display.to_string(),
        "...
.##
"
        .to_owned()
    );
}
