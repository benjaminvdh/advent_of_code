pub mod parsing;

pub mod part_1;
pub mod part_2;

pub use parsing::Parser;

fn window_is_increasing(window: &[i32]) -> u32 {
    match window {
        &[first, second, ..] if second > first => 1,
        _ => 0,
    }
}

pub fn print_success_message(num_increases: u32) {
    println!(
        "The depth measurement has increased {} times.",
        num_increases
    );
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod window_is_increasing {
        use super::*;

        #[test]
        fn increasing_window() {
            assert_eq!(window_is_increasing(&[1, 2]), 1);
        }

        #[test]
        fn increasing_window_trailing_increase() {
            assert_eq!(window_is_increasing(&[1, 2, 3]), 1);
        }

        #[test]
        fn increasing_window_trailing_decrease() {
            assert_eq!(window_is_increasing(&[1, 2, 1]), 1);
        }

        #[test]
        fn increasing_window_big() {
            assert_eq!(window_is_increasing(&[103, 827]), 1);
        }

        #[test]
        fn decreasing_window() {
            assert_eq!(window_is_increasing(&[2, 1]), 0);
        }

        #[test]
        fn decreasing_window_trailing_increase() {
            assert_eq!(window_is_increasing(&[2, 1, 3]), 0);
        }

        #[test]
        fn decreasing_window_trailing_decrease() {
            assert_eq!(window_is_increasing(&[2, 1, 0]), 0);
        }

        #[test]
        fn decreasing_window_big() {
            assert_eq!(window_is_increasing(&[827, 103]), 0);
        }

        #[test]
        fn too_small_window() {
            assert_eq!(window_is_increasing(&[3]), 0);
        }

        #[test]
        fn empty_window() {
            assert_eq!(window_is_increasing(&[]), 0);
        }
    }
}
