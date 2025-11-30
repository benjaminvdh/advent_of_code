use advent_of_code::day_6;

fn main() {
    match run() {
        Ok(num_lanternfish) => {
            day_6::print_success_message(num_lanternfish, day_6::part_2::NUM_DAYS)
        }
        Err(e) => day_6::print_error_message(e),
    }
}

fn run() -> Result<u64, advent_of_code::InputError> {
    let num_lanternfish = advent_of_code::get_input(&day_6::Parser)?
        .into_iter()
        .map(|lanternfish| day_6::part_2::compute_num_lanternfish(lanternfish))
        .sum();

    Ok(num_lanternfish)
}
