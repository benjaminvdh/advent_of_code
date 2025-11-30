use crate::day_3::binary_digit_count::{self, BinaryDigitCount};

pub fn print_success_message(product: u32) {
    println!(
        "The product of the oxygen generator rating and the CO₂ scrubber rating is {}.",
        product
    );
}

pub fn compute_oxygen_times_co2<const N: usize>(measurements: &[u32]) -> u32 {
    compute_oxygen_generator_rating(measurements, N) * compute_co2_scrubber_rating(measurements, N)
}

fn compute_oxygen_generator_rating(measurements: &[u32], index: usize) -> u32 {
    let target_digit_func = |count: &BinaryDigitCount<1>| if count.0[0] >= 0 { 1 } else { 0 };

    match filter_measurements(measurements, index, target_digit_func).as_slice() {
        &[value] => value,
        values => compute_oxygen_generator_rating(values, index - 1),
    }
}

fn compute_co2_scrubber_rating(measurements: &[u32], index: usize) -> u32 {
    let target_digit_func = |count: &BinaryDigitCount<1>| if count.0[0] >= 0 { 0 } else { 1 };

    match filter_measurements(measurements, index, target_digit_func).as_slice() {
        &[value] => value,
        values => compute_co2_scrubber_rating(values, index - 1),
    }
}

fn filter_measurements<F>(measurements: &[u32], index: usize, target_digit_func: F) -> Vec<u32>
where
    F: Fn(&BinaryDigitCount<1>) -> u32,
{
    let only_first_digits: Vec<_> = measurements
        .iter()
        .map(|measurement| measurement >> (index - 1) & 1)
        .collect();

    let count = binary_digit_count::count_binary_digits::<1>(&only_first_digits);

    measurements
        .iter()
        .filter(|measurement| *measurement >> index - 1 & 1 == target_digit_func(&count))
        .copied()
        .collect()
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
        assert_eq!(compute_oxygen_times_co2::<5>(&MEASUREMENTS), 230);
    }

    #[test]
    fn oxygen() {
        assert_eq!(compute_oxygen_generator_rating(&MEASUREMENTS, 5), 23);
    }

    #[test]
    fn co2() {
        assert_eq!(compute_co2_scrubber_rating(&MEASUREMENTS, 5), 10);
    }
}
