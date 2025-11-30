use std::ops::Add;

pub fn count_binary_digits<const N: usize>(measurements: &[u32]) -> BinaryDigitCount<N> {
    measurements
        .iter()
        .fold(BinaryDigitCount::new(), |counter, measurement| {
            counter + *measurement
        })
}

pub struct BinaryDigitCount<const N: usize>(pub [i32; N]);

impl<const N: usize> BinaryDigitCount<N> {
    fn new() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Add<u32> for BinaryDigitCount<N> {
    type Output = Self;

    fn add(self, number: u32) -> Self::Output {
        let BinaryDigitCount(mut occurrences) = self;

        for (index, total_digits) in occurrences.iter_mut().enumerate() {
            if number >> index & 1 == 1 {
                *total_digits += 1;
            } else {
                *total_digits -= 1;
            }
        }

        BinaryDigitCount(occurrences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_digits() {
        let measurements = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let binary_digit_count = count_binary_digits::<5>(&measurements);

        assert_eq!(&binary_digit_count.0, &[-2, 2, 4, -2, 2]);
    }
}
