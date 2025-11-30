use std::iter;

use crate::day_13::{Coord, Fold};

pub struct Grid {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        let width = max_x + 1;
        let height = max_y + 1;

        Self {
            cells: iter::repeat(iter::repeat(false).take(width).collect())
                .take(height)
                .collect(),
            width,
            height,
        }
    }

    pub fn get_number_of_dots(&self) -> usize {
        (0..self.height)
            .map(|y| (0..self.width).filter(|&x| self.is_set(x, y)).count())
            .sum()
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_set(x, y) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }

            println!("");
        }
    }

    pub fn fill(&mut self, coords: &[Coord]) {
        for coord in coords {
            self.set(coord.0, coord.1);
        }
    }

    pub fn is_set(&self, x: usize, y: usize) -> bool {
        self.cells[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.cells[y][x] = true;
    }

    pub fn fold(&mut self, instruction: &Fold) {
        match instruction {
            Fold::X(line) => self.fold_x(*line),
            Fold::Y(line) => self.fold_y(*line),
        }
    }

    fn fold_x(&mut self, line: usize) {
        for y in 0..self.height {
            for x in line..self.width {
                if self.is_set(x, y) {
                    self.set(mirror(x, line), y);
                }
            }
        }

        self.width = line + 1;
    }

    fn fold_y(&mut self, line: usize) {
        for y in line..self.height {
            for x in 0..self.width {
                if self.is_set(x, y) {
                    self.set(x, mirror(y, line));
                }
            }
        }

        self.height = line + 1;
    }
}

fn mirror(coord: usize, line: usize) -> usize {
    line - (coord - line)
}
