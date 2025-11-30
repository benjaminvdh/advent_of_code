use advent_of_code::day_2;

fn main() {
    match run() {
        Ok(product) => day_2::print_success_message(product),
        Err(e) => day_2::print_error_message(e),
    }
}

fn run() -> Result<i32, advent_of_code::InputError> {
    let commands = advent_of_code::get_input(&day_2::Parser)?;
    Ok(day_2::part_2::execute_commands(&commands))
}
