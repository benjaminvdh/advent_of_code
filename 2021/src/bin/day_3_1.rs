use advent_of_code::day_3;

fn main() {
    match run() {
        Ok(product) => day_3::part_1::print_success_message(product),
        Err(e) => day_3::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let measurements = advent_of_code::get_input(&day_3::Parser)?;
    let product = day_3::part_1::compute_gamma_times_epsilon::<12>(&measurements);

    Ok(product)
}
