use advent_of_code::day_9;

fn main() {
    match run() {
        Ok(risk_level) => day_9::part_1::print_success_message(risk_level),
        Err(e) => day_9::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let heightmap = advent_of_code::get_input(&day_9::Parser)?;
    Ok(day_9::part_1::compute_total_risk_level(&heightmap))
}
