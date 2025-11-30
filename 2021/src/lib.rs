pub mod input;
pub mod parsing;

pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

pub use input::get_input;
pub use input::InputError;

pub fn abs_diff(a: u32, b: u32) -> u32 {
    a.checked_sub(b).unwrap_or_else(|| b - a)
}
