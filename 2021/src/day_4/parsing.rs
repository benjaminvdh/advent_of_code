use std::num::IntErrorKind;
use std::str::FromStr;

use crate::parsing::{self, ParseError};

pub struct Parser;

#[derive(Debug, PartialEq)]
pub enum BingoInput {
    Empty,
    Draws(Vec<u8>),
    Row(Vec<u8>),
}

impl parsing::Parser for Parser {
    type Output = BingoInput;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        if line.find(|c| c == ',').is_some() {
            let draws = parse_draws(line)?;

            Ok(BingoInput::Draws(draws))
        } else if line.find(|c| c == ' ').is_some() {
            let fields = parse_row(line)?;

            Ok(BingoInput::Row(fields))
        } else {
            Ok(BingoInput::Empty)
        }
    }
}

fn parse_draws(line: &str) -> Result<Vec<u8>, ParseError> {
    line.split(",").map(|draw| parse_draw(draw)).collect()
}

fn parse_draw(draw: &str) -> Result<u8, ParseError> {
    u8::from_str(draw).map_err(|e| match e.kind() {
        IntErrorKind::Empty => ParseError::Incomplete(draw.to_owned()),
        _ => ParseError::Invalid(draw.to_owned()),
    })
}

fn parse_row(line: &str) -> Result<Vec<u8>, ParseError> {
    line.split_whitespace()
        .map(|field| parse_field(field))
        .collect()
}

fn parse_field(field: &str) -> Result<u8, ParseError> {
    u8::from_str(field.trim()).map_err(|e| match e.kind() {
        IntErrorKind::Empty => ParseError::Incomplete(field.to_owned()),
        _ => ParseError::Invalid(field.to_owned()),
    })
}

#[cfg(test)]
mod tests {
    use parsing::Parser;

    use super::*;

    #[test]
    fn read_draws() {
        let line = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
        let parser = Parser;

        let reference = &[
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        assert!(
            matches!(parser.parse(line).unwrap(), BingoInput::Draws(draws) if &draws == reference)
        );
    }

    #[test]
    fn read_board() {
        let input = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

";
        let references = [
            BingoInput::Row(vec![22, 13, 17, 11, 0]),
            BingoInput::Row(vec![8, 2, 23, 4, 24]),
            BingoInput::Row(vec![21, 9, 14, 16, 7]),
            BingoInput::Row(vec![6, 10, 3, 18, 5]),
            BingoInput::Row(vec![1, 12, 20, 15, 19]),
            BingoInput::Empty,
        ];

        let parser = Parser;

        for (line, reference) in input.lines().zip(references.iter()) {
            let parsed = parser.parse(line);

            dbg!(&line);
            dbg!(reference);
            dbg!(&parsed);

            assert!(matches!(parsed.unwrap(), result if &result == reference));
        }
    }
}
