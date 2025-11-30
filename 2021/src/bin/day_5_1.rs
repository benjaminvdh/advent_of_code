use advent_of_code::day_5;

fn main() {
    match run() {
        Ok(num_dangerous_tiles) => day_5::print_success_message(num_dangerous_tiles),
        Err(e) => day_5::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let input = advent_of_code::get_input(&day_5::Parser)?;
    let num_dangerous_tiles = day_5::part_1::count_dangerous_tiles(&input);

    Ok(num_dangerous_tiles)
}
