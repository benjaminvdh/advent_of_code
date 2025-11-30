use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct PartNumber {
    x_start: usize,
    x_end: usize,
    y: usize,
    number: u32,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    x: usize,
    y: usize,
    gear: bool,
}

impl TryFrom<(usize, usize, char)> for Symbol {
    type Error = ();

    fn try_from(tuple: (usize, usize, char)) -> Result<Self, Self::Error> {
        let (x, y, c) = tuple;

        match c {
            c if !c.is_ascii_digit() && c != '.' => Ok(Self {
                x,
                y,
                gear: c == '*',
            }),
            _ => Err(()),
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = (Vec<PartNumber>, Vec<Symbol>);
    type Output1 = u32;
    type Output2 = u32;

    fn parse(input: &str) -> Self::Input {
        let part_numbers = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| parse_part_number_line(y, line))
            .collect();

        let symbols = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| parse_symbol_line(y, line))
            .collect();

        (part_numbers, symbols)
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (part_numbers, symbols) = input;

        part_numbers
            .iter()
            .filter(|part_number| is_next_to_symbols(part_number, &symbols))
            .map(|part_number| part_number.number)
            .sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (part_numbers, symbols) = input;

        symbols
            .iter()
            .filter_map(|symbol| get_gear_ratio(symbol, &part_numbers))
            .sum()
    }
}

fn parse_part_number_line(y: usize, line: &str) -> Vec<PartNumber> {
    let mut part_numbers = vec![];
    let mut start_index = 0;

    loop {
        let values: Vec<(usize, char)> = line
            .char_indices()
            .skip(start_index)
            .skip_while(|(_, c)| !c.is_ascii_digit())
            .take_while(|(_, c)| c.is_ascii_digit())
            .collect();

        match (values.first(), values.last()) {
            (Some((x_start, _)), Some((x_end, _))) => {
                let chars: String = values.iter().map(|(_, c)| c).collect();
                let number = u32::from_str(&chars).unwrap();

                part_numbers.push(PartNumber {
                    x_start: *x_start,
                    x_end: *x_end,
                    y,
                    number,
                });

                start_index = x_end + 1;
            }
            _ => break,
        }
    }

    part_numbers
}

fn parse_symbol_line(y: usize, line: &str) -> Vec<Symbol> {
    line.char_indices()
        .flat_map(|(x, c)| Symbol::try_from((x, y, c)))
        .collect()
}

fn is_next_to_symbols(part_number: &PartNumber, symbols: &[Symbol]) -> bool {
    symbols
        .iter()
        .any(|symbol| is_next_to_symbol(part_number, symbol))
}

fn is_next_to_symbol(part_number: &PartNumber, symbol: &Symbol) -> bool {
    part_number.y.abs_diff(symbol.y) <= 1
        && part_number.x_start.saturating_sub(1) <= symbol.x
        && symbol.x <= part_number.x_end + 1
}

fn get_gear_ratio(symbol: &Symbol, part_numbers: &[PartNumber]) -> Option<u32> {
    if !symbol.gear {
        return None;
    }

    let nearby_part_numbers: Vec<_> = part_numbers
        .iter()
        .filter(|part_number| is_next_to_symbol(part_number, symbol))
        .collect();

    if nearby_part_numbers.len() != 2 {
        return None;
    }

    Some(
        nearby_part_numbers
            .iter()
            .map(|part_number| part_number.number)
            .product(),
    )
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <super::Solver as aoc::Solver>::Input {
        let part_numbers = vec![
            PartNumber {
                x_start: 0,
                x_end: 2,
                y: 0,
                number: 467,
            },
            PartNumber {
                x_start: 5,
                x_end: 7,
                y: 0,
                number: 114,
            },
            PartNumber {
                x_start: 2,
                x_end: 3,
                y: 2,
                number: 35,
            },
            PartNumber {
                x_start: 6,
                x_end: 8,
                y: 2,
                number: 633,
            },
            PartNumber {
                x_start: 0,
                x_end: 2,
                y: 4,
                number: 617,
            },
            PartNumber {
                x_start: 7,
                x_end: 8,
                y: 5,
                number: 58,
            },
            PartNumber {
                x_start: 2,
                x_end: 4,
                y: 6,
                number: 592,
            },
            PartNumber {
                x_start: 6,
                x_end: 8,
                y: 7,
                number: 755,
            },
            PartNumber {
                x_start: 1,
                x_end: 3,
                y: 9,
                number: 664,
            },
            PartNumber {
                x_start: 5,
                x_end: 7,
                y: 9,
                number: 598,
            },
        ];

        let symbols = vec![
            Symbol {
                x: 3,
                y: 1,
                gear: true,
            },
            Symbol {
                x: 6,
                y: 3,
                gear: false,
            },
            Symbol {
                x: 3,
                y: 4,
                gear: true,
            },
            Symbol {
                x: 5,
                y: 5,
                gear: false,
            },
            Symbol {
                x: 3,
                y: 8,
                gear: false,
            },
            Symbol {
                x: 5,
                y: 8,
                gear: true,
            },
        ];

        (part_numbers, symbols)
    }

    #[test]
    fn parsing() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 4361);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 467835);
    }
}
