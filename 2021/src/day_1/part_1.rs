use crate::day_1;

pub fn count_number_of_increases(measurements: &[i32]) -> u32 {
    measurements
        .windows(2)
        .map(|window| day_1::window_is_increasing(window))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_measurements() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_number_of_increases(&input), 7);
    }

    #[test]
    fn no_measurements() {
        let input = vec![];

        assert_eq!(count_number_of_increases(&input), 0);
    }
}
