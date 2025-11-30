use advent_of_code::day_8;

fn main() {
    match run() {
        Ok(num_unique_patterns) => day_8::part_1::print_success_message(num_unique_patterns),
        Err(e) => day_8::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let input = advent_of_code::get_input(&day_8::Parser)?;
    Ok(day_8::part_1::get_num_unique_patterns(&input))
}
