use crate::day_13;
use crate::day_13::parsing::Input;

pub fn print_instructions(input: &[Input]) {
    let (coords, folds) = day_13::split_coords_and_folds(input);

    let mut grid = day_13::create_grid(&coords);

    for fold in &folds {
        grid.fold(fold);
    }

    grid.print();
}
