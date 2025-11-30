pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

mod lanternfish;

use lanternfish::LanternFish;

pub fn print_success_message(num_lanternfish: u64, num_days: u32) {
    println!(
        "There are {} lanternfish after {} days.",
        num_lanternfish, num_days
    );
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}
