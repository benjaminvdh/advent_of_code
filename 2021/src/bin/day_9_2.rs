use advent_of_code::day_9;

fn main() {
    match run() {
        Ok(product) => day_9::part_2::print_success_message(product),
        Err(e) => day_9::print_error_message(e),
    }
}

fn run() -> Result<u32, advent_of_code::InputError> {
    let heightmap = advent_of_code::get_input(&day_9::Parser)?;
    Ok(day_9::part_2::compute_basin_size_product(heightmap))
}
