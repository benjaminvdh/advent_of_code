pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

mod grid;

use crate::parsing::ParseError;

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}

fn create_grid(
    rows: Vec<[u8; grid::GRID_SIZE]>,
) -> Result<[[u8; grid::GRID_SIZE]; grid::GRID_SIZE], ParseError> {
    rows.try_into()
        .map_err(|_| ParseError::Invalid(String::from("Incorrect number of rows")))
}
