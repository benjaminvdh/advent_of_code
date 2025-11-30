use advent_of_code::day_12;

fn main() {
    match run() {
        Ok(num_paths) => day_12::part_1::print_success_message(num_paths),
        Err(e) => day_12::print_error_message(e),
    }
}

fn run() -> Result<usize, advent_of_code::InputError> {
    let links = advent_of_code::get_input(&day_12::Parser)?;
    Ok(day_12::part_1::get_number_of_paths(links))
}
