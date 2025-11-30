use crate::day_13;
use crate::day_13::parsing::Input;

pub fn print_success_message(num_dots: usize) {
    println!(
        "After folding once, there are {} dots on the paper.",
        num_dots
    );
}

pub fn get_number_of_dots(input: &[Input]) -> usize {
    let (coords, folds) = day_13::split_coords_and_folds(input);

    let mut grid = day_13::create_grid(&coords);

    if let Some(fold) = folds.first() {
        grid.fold(fold);
    }

    grid.get_number_of_dots()
}

#[cfg(test)]
mod tests {
    use super::*;

    use day_13::grid::Grid;
    use day_13::{Coord, Fold};

    #[test]
    fn fold_once() {
        let coords = [
            Coord(6, 10),
            Coord(0, 14),
            Coord(9, 10),
            Coord(0, 3),
            Coord(10, 4),
            Coord(4, 11),
            Coord(6, 0),
            Coord(6, 12),
            Coord(4, 1),
            Coord(0, 13),
            Coord(10, 12),
            Coord(3, 4),
            Coord(3, 0),
            Coord(8, 4),
            Coord(1, 10),
            Coord(2, 14),
            Coord(8, 10),
            Coord(9, 0),
        ];

        let mut grid = Grid::new(11, 15);
        grid.fill(&coords);

        grid.fold(&Fold::Y(7));
        assert_eq!(grid.get_number_of_dots(), 17);
    }
}
