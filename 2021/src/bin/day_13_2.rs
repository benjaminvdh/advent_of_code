use advent_of_code::day_13;

fn main() {
    if let Err(e) = run() {
        day_13::print_error_message(e);
    }
}

fn run() -> Result<(), advent_of_code::InputError> {
    let input = advent_of_code::get_input(&day_13::Parser)?;
    Ok(day_13::part_2::print_instructions(&input))
}
