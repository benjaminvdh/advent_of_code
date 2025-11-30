use advent_of_code::day_11;

fn main() {
    match run() {
        Ok(first_synchronized_step) => {
            day_11::part_2::print_success_message(first_synchronized_step)
        }
        Err(e) => day_11::print_error_message(e),
    }
}

fn run() -> Result<usize, advent_of_code::InputError> {
    let charges = advent_of_code::get_input(&day_11::Parser)?;
    Ok(day_11::part_2::get_first_synchronized_step(charges)?)
}
