use advent_of_code::day_8;

fn main() {
    match run() {
        Ok(sum) => day_8::part_2::print_success_message(sum),
        Err(e) => day_8::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let input = advent_of_code::get_input(&day_8::Parser)?;
    Ok(day_8::part_2::compute_sum(&input))
}
