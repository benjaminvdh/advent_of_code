use advent_of_code::day_7;

fn main() {
    match run() {
        Ok(fuel) => day_7::print_success_message(fuel),
        Err(e) => day_7::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let fuel = advent_of_code::get_input(&day_7::Parser)?
        .first()
        .map(|crabs| day_7::part_1::get_least_fuel(crabs))
        .unwrap_or(0);

    Ok(fuel)
}
