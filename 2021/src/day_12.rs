pub mod part_1;
pub mod part_2;

pub mod cave;
pub mod parsing;

pub use parsing::Parser;

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}
