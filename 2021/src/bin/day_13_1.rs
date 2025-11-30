use advent_of_code::day_13;

fn main() {
    match run() {
        Ok(num_dots) => day_13::part_1::print_success_message(num_dots),
        Err(e) => day_13::print_error_message(e),
    }
}

fn run() -> Result<usize, advent_of_code::InputError> {
    let input = advent_of_code::get_input(&day_13::Parser)?;
    Ok(day_13::part_1::get_number_of_dots(&input))
}
