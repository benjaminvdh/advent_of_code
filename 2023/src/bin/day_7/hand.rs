use super::Card;

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn get_type(&self) -> Type {
        let mut cards = self.cards;
        cards.sort();
        let [a, b, c, d, e] = &cards;

        if a == b && b == c && c == d && d == e {
            Type::FiveOfAKind
        } else if (a == b && b == c && c == d) || (b == c && c == d && d == e) {
            Type::FourOfAKind
        } else if (a == b && b == c && d == e) || (a == b && c == d && d == e) {
            Type::FullHouse
        } else if (a == b && b == c) || (b == c && c == d) || (c == d && d == e) {
            Type::ThreeOfAKind
        } else if (a == b && c == d) || (a == b && d == e) || (b == c && d == e) {
            Type::TwoPair
        } else if a == b || b == c || c == d || d == e {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }

    pub fn get_type_with_jokers(&self) -> Type {
        let jokers = self.cards.iter().filter(|c| **c == Card::J).count();

        match self.get_type() {
            Type::FourOfAKind if jokers == 4 => Type::FiveOfAKind,
            Type::FourOfAKind if jokers == 1 => Type::FiveOfAKind,
            Type::FullHouse if jokers == 3 => Type::FiveOfAKind,
            Type::FullHouse if jokers == 2 => Type::FiveOfAKind,
            Type::ThreeOfAKind if jokers == 3 => Type::FourOfAKind,
            Type::ThreeOfAKind if jokers == 1 => Type::FourOfAKind,
            Type::TwoPair if jokers == 2 => Type::FourOfAKind,
            Type::TwoPair if jokers == 1 => Type::FullHouse,
            Type::OnePair if jokers == 2 => Type::ThreeOfAKind,
            Type::OnePair if jokers == 1 => Type::ThreeOfAKind,
            Type::HighCard if jokers == 1 => Type::OnePair,
            t => t,
        }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }

    pub fn jcmp(&self, other: &Hand) -> Ordering {
        match self
            .get_type_with_jokers()
            .cmp(&other.get_type_with_jokers())
        {
            Ordering::Equal => jcmp_hand(&self.cards, &other.cards),
            ordering => ordering,
        }
    }
}

fn jcmp_hand(a: &[Card], b: &[Card]) -> Ordering {
    match (a, b) {
        ([a], [b]) => jcmp(*a, *b),
        ([a, a_rest @ ..], [b, b_rest @ ..]) => match jcmp(*a, *b) {
            Ordering::Equal => jcmp_hand(a_rest, b_rest),
            ordering => ordering,
        },
        _ => panic!(),
    }
}

fn jcmp(a: Card, b: Card) -> Ordering {
    match (a, b) {
        (Card::J, Card::J) => Ordering::Equal,
        (Card::J, _) => Ordering::Less,
        (_, Card::J) => Ordering::Greater,
        (a, b) => a.cmp(&b),
    }
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        Self { cards }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_type_with_jokers() {
        // Four of a kind
        assert_eq!(
            Hand::from([Card::A, Card::A, Card::A, Card::A, Card::J]).get_type_with_jokers(),
            Type::FiveOfAKind
        );
        assert_eq!(
            Hand::from([Card::A, Card::J, Card::J, Card::J, Card::J]).get_type_with_jokers(),
            Type::FiveOfAKind
        );

        // Full house
        assert_eq!(
            Hand::from([Card::A, Card::A, Card::A, Card::J, Card::J]).get_type_with_jokers(),
            Type::FiveOfAKind
        );
        assert_eq!(
            Hand::from([Card::A, Card::A, Card::J, Card::J, Card::J]).get_type_with_jokers(),
            Type::FiveOfAKind
        );

        // Three of a kind
        assert_eq!(
            Hand::from([Card::A, Card::A, Card::A, Card::K, Card::J]).get_type_with_jokers(),
            Type::FourOfAKind
        );
        assert_eq!(
            Hand::from([Card::A, Card::K, Card::J, Card::J, Card::J]).get_type_with_jokers(),
            Type::FourOfAKind
        );

        // Two pair
        assert_eq!(
            Hand::from([Card::A, Card::A, Card::K, Card::K, Card::J]).get_type_with_jokers(),
            Type::FullHouse
        );
        assert_eq!(
            Hand::from([Card::A, Card::A, Card::K, Card::J, Card::J]).get_type_with_jokers(),
            Type::FourOfAKind
        );

        // One pair
        assert_eq!(
            Hand::from([Card::A, Card::A, Card::K, Card::Q, Card::J]).get_type_with_jokers(),
            Type::ThreeOfAKind
        );
        assert_eq!(
            Hand::from([Card::A, Card::K, Card::Q, Card::J, Card::J]).get_type_with_jokers(),
            Type::ThreeOfAKind
        );

        // High card
        assert_eq!(
            Hand::from([Card::A, Card::K, Card::Q, Card::T, Card::J]).get_type_with_jokers(),
            Type::OnePair
        );
    }
}
