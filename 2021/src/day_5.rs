pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

mod detector;
mod grid;
mod line;
mod point;

pub fn print_success_message(num_dangerous_tiles: u32) {
    println!("There are {} dangerous tiles", num_dangerous_tiles);
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}
