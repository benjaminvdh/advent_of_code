use advent_of_code::day_11;

fn main() {
    match run() {
        Ok(num_flashes) => day_11::part_1::print_success_message(num_flashes),
        Err(e) => day_11::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let charges = advent_of_code::get_input(&day_11::Parser)?;
    Ok(day_11::part_1::get_number_of_flashes(charges)?)
}
