use crate::day_11;
use crate::day_11::grid::{self, GRID_SIZE};

use crate::parsing::ParseError;

pub fn print_success_message(first_synchronized_step: usize) {
    println!(
        "All flashes were synchronized after {} steps.",
        first_synchronized_step
    );
}

pub fn get_first_synchronized_step(rows: Vec<[u8; GRID_SIZE]>) -> Result<usize, ParseError> {
    let mut grid = day_11::create_grid(rows)?;

    for i in 1.. {
        let _ = grid::update(&mut grid);

        if (0..GRID_SIZE).all(|y| (0..GRID_SIZE).all(|x| grid[y][x] == 0)) {
            return Ok(i);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_synchronized_step() {
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

        assert_eq!(get_first_synchronized_step(grid).unwrap(), 195);
    }
}
