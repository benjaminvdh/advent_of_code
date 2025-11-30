pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

mod crab;
mod propulsion;

use crab::Crab;
use propulsion::Propulsion;

pub fn print_success_message(fuel: u32) {
    println!(
        "The most efficient position requires {} units of fuel",
        fuel
    );
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}

fn get_least_fuel<P: Propulsion>(crabs: &[Crab], propulsion: &P) -> u32 {
    let min = crabs.iter().map(|crab| crab.pos).min().unwrap_or(0);
    let max = crabs.iter().map(|crab| crab.pos).max().unwrap_or(0);

    (min..=max)
        .map(|pos| get_total_fuel_for_position(crabs, propulsion, pos))
        .min()
        .unwrap_or(0)
}

fn get_total_fuel_for_position<P: Propulsion>(crabs: &[Crab], propulsion: &P, pos: u32) -> u32 {
    crabs
        .iter()
        .map(|crab| crab.get_fuel_for_position(propulsion, pos))
        .sum()
}
