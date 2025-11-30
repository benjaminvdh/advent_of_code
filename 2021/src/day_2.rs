pub mod parsing;

pub mod part_1;
pub mod part_2;

pub use parsing::Parser;

mod command;

pub fn print_success_message(product: i32) {
    println!("The final product is {}.", product);
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}
