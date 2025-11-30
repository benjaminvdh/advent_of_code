use crate::day_3::binary_digit_count::{self, BinaryDigitCount};

pub fn print_success_message(product: u32) {
    println!("The product of gamma and epsilon is {}.", product);
}

pub fn compute_gamma_times_epsilon<const N: usize>(measurements: &[u32]) -> u32 {
    let binary_digit_count = binary_digit_count::count_binary_digits::<N>(measurements);

    compute_gamma(&binary_digit_count) * compute_epsilon(&binary_digit_count)
}

fn compute_gamma<const N: usize>(count: &BinaryDigitCount<N>) -> u32 {
    count
        .0
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_positive())
        .map(|(i, _)| 1 << i)
        .sum()
}

fn compute_epsilon<const N: usize>(count: &BinaryDigitCount<N>) -> u32 {
    !compute_gamma(count) & ((1 << N as u32) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASUREMENTS: [u32; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    #[test]
    fn product() {
        assert_eq!(compute_gamma_times_epsilon::<5>(&MEASUREMENTS), 198);
    }

    fn get_binary_count() -> BinaryDigitCount<5> {
        binary_digit_count::count_binary_digits(&MEASUREMENTS)
    }

    #[test]
    fn gamma() {
        let count = get_binary_count();

        assert_eq!(compute_gamma(&count), 22);
    }

    #[test]
    fn epsilon() {
        let count = get_binary_count();

        assert_eq!(compute_epsilon(&count), 9);
    }
}
