use crate::day_11;
use crate::day_11::grid::{self, GRID_SIZE};

use crate::parsing::ParseError;

pub fn print_success_message(num_flashes: u32) {
    println!("There were {} flashes after 100 cycles", num_flashes);
}

pub fn get_number_of_flashes(rows: Vec<[u8; GRID_SIZE]>) -> Result<u32, ParseError> {
    let mut grid = day_11::create_grid(rows)?;

    Ok((0..100).map(|_| grid::update(&mut grid)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_flashes() {
        let grid = vec![
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(get_number_of_flashes(grid).unwrap(), 1656);
    }
}
