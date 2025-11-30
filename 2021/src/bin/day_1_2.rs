use advent_of_code::day_1;

fn main() {
    match run() {
        Ok(num_increases) => day_1::print_success_message(num_increases),
        Err(e) => day_1::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let measurements = advent_of_code::get_input(&day_1::Parser)?;
    Ok(day_1::part_2::count_number_of_increases(&measurements))
}
