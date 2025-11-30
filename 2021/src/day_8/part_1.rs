use crate::day_8::pattern::Pattern;

pub fn print_success_message(num_unique_patterns: u32) {
    println!(
        "There are {} instances of 1, 4, 7 and 8",
        num_unique_patterns
    );
}

pub fn get_num_unique_patterns(input: &[([Pattern; 10], [Pattern; 4])]) -> u32 {
    input
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter(|pattern| {
                    pattern.is_one()
                        || pattern.is_four()
                        || pattern.is_seven()
                        || pattern.is_eight()
                })
                .count() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::day_8::pattern::Segment as Seg;

    #[test]
    fn num_unique_patterns() {
        let outputs = [
            [
                Seg::F | Seg::D | Seg::G | Seg::A | Seg::C | Seg::B | Seg::E,
                Seg::C | Seg::E | Seg::F | Seg::D | Seg::B,
                Seg::C | Seg::E | Seg::F | Seg::B | Seg::G | Seg::D,
                Seg::G | Seg::C | Seg::B | Seg::E,
            ],
            [
                Seg::F | Seg::C | Seg::G | Seg::E | Seg::D | Seg::B,
                Seg::C | Seg::G | Seg::B,
                Seg::D | Seg::G | Seg::E | Seg::B | Seg::A | Seg::C | Seg::F,
                Seg::G | Seg::C,
            ],
            [
                Seg::C | Seg::G,
                Seg::C | Seg::G,
                Seg::F | Seg::D | Seg::C | Seg::A | Seg::G | Seg::B,
                Seg::C | Seg::B | Seg::G,
            ],
            [
                Seg::E | Seg::F | Seg::A | Seg::B | Seg::C | Seg::D,
                Seg::C | Seg::E | Seg::D | Seg::B | Seg::A,
                Seg::G | Seg::A | Seg::D | Seg::F | Seg::E | Seg::C,
                Seg::C | Seg::B,
            ],
            [
                Seg::G | Seg::E | Seg::C | Seg::F,
                Seg::E | Seg::G | Seg::D | Seg::C | Seg::A | Seg::B | Seg::F,
                Seg::B | Seg::G | Seg::F,
                Seg::B | Seg::F | Seg::G | Seg::E | Seg::A,
            ],
            [
                Seg::G | Seg::E | Seg::B | Seg::D | Seg::C | Seg::F | Seg::A,
                Seg::E | Seg::C | Seg::B | Seg::A,
                Seg::C | Seg::A,
                Seg::F | Seg::A | Seg::D | Seg::E | Seg::G | Seg::C | Seg::B,
            ],
            [
                Seg::C | Seg::E | Seg::F | Seg::G,
                Seg::D | Seg::C | Seg::B | Seg::E | Seg::F,
                Seg::F | Seg::C | Seg::G | Seg::E,
                Seg::G | Seg::B | Seg::C | Seg::A | Seg::D | Seg::F | Seg::E,
            ],
            [
                Seg::E | Seg::D,
                Seg::B | Seg::C | Seg::G | Seg::A | Seg::F | Seg::E,
                Seg::C | Seg::D | Seg::G | Seg::B | Seg::A,
                Seg::C | Seg::B | Seg::G | Seg::E | Seg::F,
            ],
            [
                Seg::G | Seg::B | Seg::D | Seg::F | Seg::C | Seg::A | Seg::E,
                Seg::B | Seg::G | Seg::C,
                Seg::C | Seg::G,
                Seg::C | Seg::G | Seg::B,
            ],
            [
                Seg::F | Seg::G | Seg::A | Seg::E,
                Seg::C | Seg::F | Seg::G | Seg::A | Seg::B,
                Seg::F | Seg::G,
                Seg::B | Seg::A | Seg::G | Seg::C | Seg::E,
            ],
        ];

        let signals = [[Pattern::from(Seg::A); 10]; 10];

        let input: Vec<_> = signals.into_iter().zip(outputs.into_iter()).collect();
        assert_eq!(get_num_unique_patterns(&input), 26);
    }
}
