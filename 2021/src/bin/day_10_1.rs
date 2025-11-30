use advent_of_code::day_10;

fn main() {
    match run() {
        Ok(score) => day_10::part_1::print_success_message(score),
        Err(e) => day_10::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let chars = advent_of_code::get_input(&day_10::Parser)?;
    Ok(day_10::part_1::get_total_syntax_error_score(&chars))
}
