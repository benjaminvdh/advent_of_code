use crate::day_8::pattern::Pattern;

pub fn print_success_message(sum: u32) {
    println!("The sum of all output signals is {}.", sum);
}

pub fn compute_sum(input: &[([Pattern; 10], [Pattern; 4])]) -> u32 {
    input
        .iter()
        .map(|(signals, outputs)| determine_output_number(signals, outputs).unwrap_or(0))
        .sum()
}

fn determine_output_number(signals: &[Pattern; 10], outputs: &[Pattern; 4]) -> Option<u32> {
    let ordered_signals = order_signals(signals)?;

    let sum = outputs
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &output)| 10u32.pow(i as u32) * get_digit(&ordered_signals, output))
        .sum();

    Some(sum)
}

fn get_digit(ordered_signals: &[Pattern; 10], output: Pattern) -> u32 {
    ordered_signals
        .iter()
        .enumerate()
        .find(|(_, &signal)| signal == output)
        .map(|(i, _)| i as u32)
        .unwrap_or(0)
}

fn order_signals(signals: &[Pattern; 10]) -> Option<[Pattern; 10]> {
    let mut signals: Vec<_> = signals.into_iter().copied().collect();

    let is_one = |signal: &Pattern| signal.is_one();
    let one = get_number(&mut signals, is_one)?;

    let is_four = |signal: &Pattern| signal.is_four();
    let four = get_number(&mut signals, is_four)?;

    let is_seven = |signal: &Pattern| signal.is_seven();
    let seven = get_number(&mut signals, is_seven)?;

    let is_eight = |signal: &Pattern| signal.is_eight();
    let eight = get_number(&mut signals, is_eight)?;

    let is_three = |signal: &Pattern| signal.num_segments() == 5 && *signal & one == one;
    let three = get_number(&mut signals, is_three)?;

    let is_five =
        |signal: &Pattern| signal.num_segments() == 5 && *signal & (four & !one) == (four & !one);
    let five = get_number(&mut signals, is_five)?;

    let is_two = |signal: &Pattern| signal.num_segments() == 5;
    let two = get_number(&mut signals, is_two)?;

    let is_nine = |signal: &Pattern| *signal & (seven | four) == (seven | four);
    let nine = get_number(&mut signals, is_nine)?;

    let is_zero = |signal: &Pattern| *signal & one == one;
    let zero = get_number(&mut signals, is_zero)?;

    let six = signals.swap_remove(0);

    Some([zero, one, two, three, four, five, six, seven, eight, nine])
}

fn get_number<F>(signals: &mut Vec<Pattern>, is_number: F) -> Option<Pattern>
where
    F: Fn(&Pattern) -> bool,
{
    let index = signals.iter().position(is_number)?;
    let number = signals.swap_remove(index);

    Some(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::day_8::pattern::Segment as Seg;

    #[test]
    fn sum() {
        let signals = [
            [
                Seg::B | Seg::E,
                Seg::C | Seg::F | Seg::B | Seg::E | Seg::G | Seg::A | Seg::D,
                Seg::C | Seg::B | Seg::D | Seg::G | Seg::E | Seg::F,
                Seg::F | Seg::G | Seg::A | Seg::E | Seg::C | Seg::D,
                Seg::C | Seg::G | Seg::E | Seg::B,
                Seg::F | Seg::D | Seg::C | Seg::G | Seg::E,
                Seg::A | Seg::G | Seg::E | Seg::B | Seg::F | Seg::D,
                Seg::F | Seg::E | Seg::C | Seg::D | Seg::B,
                Seg::F | Seg::A | Seg::B | Seg::C | Seg::D,
                Seg::E | Seg::D | Seg::B,
            ],
            [
                Seg::E | Seg::D | Seg::B | Seg::F | Seg::G | Seg::A,
                Seg::B | Seg::E | Seg::G | Seg::C | Seg::D,
                Seg::C | Seg::B | Seg::G,
                Seg::G | Seg::C,
                Seg::G | Seg::C | Seg::A | Seg::D | Seg::E | Seg::B | Seg::F,
                Seg::F | Seg::B | Seg::G | Seg::D | Seg::E,
                Seg::A | Seg::C | Seg::B | Seg::G | Seg::F | Seg::D,
                Seg::A | Seg::B | Seg::C | Seg::D | Seg::E,
                Seg::G | Seg::F | Seg::C | Seg::B | Seg::E | Seg::D,
                Seg::G | Seg::F | Seg::E | Seg::C,
            ],
            [
                Seg::F | Seg::G | Seg::A | Seg::E | Seg::B | Seg::D,
                Seg::C | Seg::G,
                Seg::B | Seg::D | Seg::A | Seg::E | Seg::C,
                Seg::G | Seg::D | Seg::A | Seg::F | Seg::B,
                Seg::A | Seg::G | Seg::B | Seg::C | Seg::F | Seg::D,
                Seg::G | Seg::D | Seg::C | Seg::B | Seg::E | Seg::F,
                Seg::B | Seg::G | Seg::C | Seg::A | Seg::D,
                Seg::G | Seg::F | Seg::A | Seg::C,
                Seg::G | Seg::C | Seg::B,
                Seg::C | Seg::D | Seg::G | Seg::A | Seg::B | Seg::E | Seg::F,
            ],
            [
                Seg::F | Seg::B | Seg::E | Seg::G | Seg::C | Seg::D,
                Seg::C | Seg::B | Seg::D,
                Seg::A | Seg::D | Seg::C | Seg::E | Seg::F | Seg::B,
                Seg::D | Seg::A | Seg::G | Seg::E | Seg::B,
                Seg::A | Seg::F | Seg::C | Seg::B,
                Seg::B | Seg::C,
                Seg::A | Seg::E | Seg::F | Seg::D | Seg::C,
                Seg::E | Seg::C | Seg::D | Seg::A | Seg::B,
                Seg::F | Seg::G | Seg::D | Seg::E | Seg::C | Seg::A,
                Seg::F | Seg::C | Seg::D | Seg::B | Seg::E | Seg::G | Seg::A,
            ],
            [
                Seg::A | Seg::E | Seg::C | Seg::B | Seg::F | Seg::D | Seg::G,
                Seg::F | Seg::B | Seg::G,
                Seg::G | Seg::F,
                Seg::B | Seg::A | Seg::F | Seg::E | Seg::G,
                Seg::D | Seg::B | Seg::E | Seg::F | Seg::A,
                Seg::F | Seg::C | Seg::G | Seg::E,
                Seg::G | Seg::C | Seg::B | Seg::E | Seg::A,
                Seg::F | Seg::C | Seg::A | Seg::E | Seg::G | Seg::B,
                Seg::D | Seg::G | Seg::C | Seg::E | Seg::A | Seg::B,
                Seg::F | Seg::C | Seg::B | Seg::D | Seg::G | Seg::A,
            ],
            [
                Seg::F | Seg::G | Seg::E | Seg::A | Seg::B,
                Seg::C | Seg::A,
                Seg::A | Seg::F | Seg::C | Seg::E | Seg::B | Seg::G,
                Seg::B | Seg::D | Seg::A | Seg::C | Seg::F | Seg::E | Seg::G,
                Seg::C | Seg::F | Seg::A | Seg::E | Seg::D | Seg::G,
                Seg::G | Seg::C | Seg::F | Seg::D | Seg::B,
                Seg::B | Seg::A | Seg::E | Seg::C,
                Seg::B | Seg::F | Seg::A | Seg::D | Seg::E | Seg::G,
                Seg::B | Seg::A | Seg::F | Seg::G | Seg::C,
                Seg::A | Seg::C | Seg::F,
            ],
            [
                Seg::D | Seg::B | Seg::C | Seg::F | Seg::G,
                Seg::F | Seg::G | Seg::D,
                Seg::B | Seg::D | Seg::E | Seg::G | Seg::C | Seg::A | Seg::F,
                Seg::F | Seg::G | Seg::E | Seg::C,
                Seg::A | Seg::E | Seg::G | Seg::B | Seg::D | Seg::F,
                Seg::E | Seg::C | Seg::D | Seg::F | Seg::A | Seg::B,
                Seg::F | Seg::B | Seg::E | Seg::D | Seg::C,
                Seg::D | Seg::A | Seg::C | Seg::G | Seg::B,
                Seg::G | Seg::D | Seg::C | Seg::E | Seg::B | Seg::F,
                Seg::G | Seg::F,
            ],
            [
                Seg::B | Seg::D | Seg::F | Seg::E | Seg::G | Seg::C,
                Seg::C | Seg::B | Seg::E | Seg::G | Seg::A | Seg::F,
                Seg::G | Seg::E | Seg::C | Seg::B | Seg::F,
                Seg::D | Seg::F | Seg::C | Seg::A | Seg::G | Seg::E,
                Seg::B | Seg::D | Seg::A | Seg::C | Seg::G,
                Seg::E | Seg::D,
                Seg::B | Seg::E | Seg::D | Seg::F,
                Seg::C | Seg::E | Seg::D,
                Seg::A | Seg::D | Seg::C | Seg::B | Seg::E | Seg::F | Seg::G,
                Seg::G | Seg::E | Seg::B | Seg::C | Seg::D,
            ],
            [
                Seg::E | Seg::G | Seg::A | Seg::D | Seg::F | Seg::B,
                Seg::C | Seg::D | Seg::B | Seg::F | Seg::E | Seg::G,
                Seg::C | Seg::E | Seg::G | Seg::D,
                Seg::F | Seg::E | Seg::C | Seg::A | Seg::B,
                Seg::C | Seg::G | Seg::B,
                Seg::G | Seg::B | Seg::D | Seg::E | Seg::F | Seg::C | Seg::A,
                Seg::C | Seg::G,
                Seg::F | Seg::G | Seg::C | Seg::D | Seg::A | Seg::B,
                Seg::E | Seg::G | Seg::F | Seg::D | Seg::B,
                Seg::B | Seg::F | Seg::C | Seg::E | Seg::G,
            ],
            [
                Seg::G | Seg::C | Seg::A | Seg::F | Seg::B,
                Seg::G | Seg::C | Seg::F,
                Seg::D | Seg::C | Seg::A | Seg::E | Seg::B | Seg::F | Seg::G,
                Seg::E | Seg::C | Seg::A | Seg::G | Seg::B,
                Seg::G | Seg::F,
                Seg::A | Seg::B | Seg::C | Seg::D | Seg::E | Seg::G,
                Seg::G | Seg::A | Seg::E | Seg::F,
                Seg::C | Seg::A | Seg::F | Seg::B | Seg::G | Seg::E,
                Seg::F | Seg::D | Seg::B | Seg::A | Seg::C,
                Seg::F | Seg::E | Seg::G | Seg::B | Seg::D | Seg::C,
            ],
        ];

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

        let input: Vec<_> = signals.into_iter().zip(outputs.into_iter()).collect();
        assert_eq!(compute_sum(&input), 61229);
    }
}
