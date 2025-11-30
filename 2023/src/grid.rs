use std::fmt::{self, Display, Formatter};

use crate::Dir;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn apply(self, dir: Dir) -> Self {
        match dir {
            Dir::N => Self {
                y: self.y - 1,
                ..self
            },
            Dir::E => Self {
                x: self.x + 1,
                ..self
            },
            Dir::S => Self {
                y: self.y + 1,
                ..self
            },
            Dir::W => Self {
                x: self.x - 1,
                ..self
            },
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: PartialEq> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        let index = self.get_index(x, y);
        &self.grid[index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = self.get_index(x, y);
        &mut self.grid[index]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        y * self.width + x
    }

    pub fn get_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        let mut neighbors = vec![];

        if let Some(x) = pos.x.checked_sub(1) {
            neighbors.push(Pos { x, ..*pos });
        }

        if let Some(y) = pos.y.checked_sub(1) {
            neighbors.push(Pos { y, ..*pos });
        }

        if pos.x < self.width - 1 {
            neighbors.push(Pos {
                x: pos.x + 1,
                ..*pos
            });
        }

        if pos.y < self.height - 1 {
            neighbors.push(Pos {
                y: pos.y + 1,
                ..*pos
            });
        }

        neighbors
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, val: T) -> Self {
        Self {
            grid: [val].repeat(width * height),
            width,
            height,
        }
    }
}

impl<T, I: IntoIterator<Item = T>> FromIterator<I> for Grid<T> {
    fn from_iter<II: IntoIterator<Item = I>>(iter: II) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut grid = vec![];

        for iiter in iter {
            height += 1;
            width = 0;

            for item in iiter {
                width += 1;
                grid.push(item);
            }
        }

        Self {
            grid,
            width: width,
            height: height,
        }
    }
}

impl<T: Display + PartialEq> Display for Grid<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(fmt, "{}", self.get(x, y))?;
            }
            writeln!(fmt)?;
        }

        Ok(())
    }
}
