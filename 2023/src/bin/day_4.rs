use std::str::FromStr;

struct Solver;

#[derive(Debug, PartialEq)]
struct Card {
    winning_numbers: Vec<u32>,
    present_numbers: Vec<u32>,
}

impl Card {
    fn get_score(&self) -> u32 {
        let num_matches = self.get_num_matches();

        if num_matches >= 1 {
            2u32.pow(num_matches as u32 - 1)
        } else {
            0
        }
    }

    fn get_num_matches(&self) -> usize {
        self.present_numbers
            .iter()
            .filter(|present_number| {
                self.winning_numbers
                    .iter()
                    .any(|winning_number| winning_number == *present_number)
            })
            .count()
    }
}

impl aoc::Solver for Solver {
    type Input = Vec<Card>;
    type Output1 = u32;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|card| card.get_score()).sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut cards: Vec<(&Card, usize)> = input.into_iter().map(|card| (card, 1)).collect();

        for i in 0..cards.len() {
            let card = cards[i].0;
            let num_matches = card.get_num_matches();

            for j in 0..num_matches {
                cards[i + j + 1].1 += cards[i].1;
            }
        }

        cards.iter().map(|(_, num)| num).sum()
    }
}

fn parse_line(line: &str) -> Card {
    let line = &line[line.find(":").unwrap() + 2..];

    let mut splits = line.split(" | ");
    let winning_string = splits.next().unwrap();
    let present_string = splits.next().unwrap();

    Card {
        winning_numbers: parse_numbers(winning_string),
        present_numbers: parse_numbers(present_string),
    }
}

fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .split_whitespace()
        .map(|number| u32::from_str(number).unwrap())
        .collect()
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Card> {
        vec![
            Card {
                winning_numbers: vec![41, 48, 83, 86, 17],
                present_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                winning_numbers: vec![13, 32, 20, 16, 61],
                present_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                winning_numbers: vec![1, 21, 53, 59, 44],
                present_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Card {
                winning_numbers: vec![41, 92, 73, 84, 69],
                present_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Card {
                winning_numbers: vec![87, 83, 26, 28, 32],
                present_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Card {
                winning_numbers: vec![31, 18, 13, 56, 72],
                present_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 13);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 30);
    }
}
