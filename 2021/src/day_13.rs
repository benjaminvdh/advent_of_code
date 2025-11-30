pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

mod grid;

use grid::Grid;
use parsing::Input;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord(usize, usize);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Fold {
    X(usize),
    Y(usize),
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}

fn create_grid(coords: &[Coord]) -> Grid {
    let (max_x, max_y) = get_max_values(&coords);
    let mut grid = Grid::new(max_x, max_y);

    grid.fill(&coords);

    grid
}

fn split_coords_and_folds(input: &[Input]) -> (Vec<Coord>, Vec<Fold>) {
    let mut coords = vec![];
    let mut folds = vec![];

    for input in input {
        match input {
            Input::Coord(coord) => coords.push(*coord),
            Input::Instruction(fold) => folds.push(*fold),
            _ => (),
        }
    }

    (coords, folds)
}

fn get_max_values(coords: &[Coord]) -> (usize, usize) {
    (
        coords.iter().map(|coord| coord.0).max().unwrap_or_default(),
        coords.iter().map(|coord| coord.1).max().unwrap_or_default(),
    )
}
