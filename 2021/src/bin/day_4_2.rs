use advent_of_code::day_4;

fn main() {
    match run() {
        Ok(product) => day_4::print_success_message(product),
        Err(e) => day_4::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let input = advent_of_code::get_input(&day_4::Parser)?;
    let product = day_4::part_2::compute_score(&input);

    Ok(product)
}
