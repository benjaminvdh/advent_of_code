mod hand;

use hand::Hand;

use std::cmp::Ordering;
use std::str::FromStr;

struct Solver;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Game {
    hand: Hand,
    bid: u64,
}

impl aoc::Solver for Solver {
    type Input = Vec<Game>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        get_total_winnings(input, |a: &Game, b: &Game| a.hand.cmp(&b.hand))
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        get_total_winnings(input, |a: &Game, b: &Game| a.hand.jcmp(&b.hand))
    }
}

fn get_total_winnings<F: Fn(&Game, &Game) -> Ordering>(
    input: &<Solver as aoc::Solver>::Input,
    comparison: F,
) -> u64 {
    let mut input = input.clone();
    input.sort_unstable_by(comparison);
    input
        .iter()
        .enumerate()
        .map(|(i, game)| (i as u64 + 1) * game.bid)
        .sum()
}

fn parse_line(line: &str) -> Game {
    let mut splits = line.split_whitespace();

    let hand = splits.next().unwrap();
    let cards: Vec<Card> = hand.chars().map(|c| Card::from(c)).collect();

    let bid = u64::from_str(splits.next().unwrap()).unwrap();

    Game {
        hand: Hand::from(<[Card; 5]>::try_from(cards).unwrap()),
        bid,
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            Game {
                hand: Hand::from([Card::C3, Card::C2, Card::T, Card::C3, Card::K]),
                bid: 765,
            },
            Game {
                hand: Hand::from([Card::T, Card::C5, Card::C5, Card::J, Card::C5]),
                bid: 684,
            },
            Game {
                hand: Hand::from([Card::K, Card::K, Card::C6, Card::C7, Card::C7]),
                bid: 28,
            },
            Game {
                hand: Hand::from([Card::K, Card::T, Card::J, Card::J, Card::T]),
                bid: 220,
            },
            Game {
                hand: Hand::from([Card::Q, Card::Q, Card::Q, Card::J, Card::A]),
                bid: 483,
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 6440);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 5905);
    }
}
