use advent_of_code::day_14;

fn main() {
    match run() {
        Ok(quantity) => day_14::print_success_message(quantity),
        Err(e) => day_14::print_error_message(e),
    }
}

fn run() -> Result<usize, advent_of_code::InputError> {
    let input = advent_of_code::get_input(&day_14::Parser)?;
    Ok(day_14::part_2::apply_transformations(&input))
}
