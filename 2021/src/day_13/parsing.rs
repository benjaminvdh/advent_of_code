use std::str::FromStr;

use crate::day_13::{Coord, Fold};
use crate::parsing::{self, ParseError};

pub struct Parser;

#[derive(Debug, PartialEq)]
pub enum Input {
    Coord(Coord),
    Empty,
    Instruction(Fold),
}

impl parsing::Parser for Parser {
    type Output = Input;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        if line.contains(',') {
            parse_coord(line)
        } else if line.contains('=') {
            parse_instruction(line)
        } else if line.is_empty() {
            Ok(Input::Empty)
        } else {
            Err(ParseError::Invalid(line.to_owned()))
        }
    }
}

fn parse_coord(line: &str) -> Result<Input, ParseError> {
    let mut iter = line.split(',');

    match (iter.next(), iter.next()) {
        (Some(x), Some(y)) => Ok(Input::Coord(Coord(
            usize::from_str(x)?,
            usize::from_str(y)?,
        ))),
        _ => Err(ParseError::Incomplete(line.to_owned())),
    }
}

fn parse_instruction(line: &str) -> Result<Input, ParseError> {
    let mut iter = line.split_whitespace();

    match (iter.next(), iter.next(), iter.next()) {
        (Some("fold"), Some("along"), Some(instruction)) => parse_fold_line(instruction),
        _ => Err(ParseError::Incomplete(line.to_owned())),
    }
}

fn parse_fold_line(instruction: &str) -> Result<Input, ParseError> {
    let mut iter = instruction.split('=');

    match (iter.next(), iter.next()) {
        (Some(axis), Some(coord)) => match axis {
            "x" => Ok(Input::Instruction(Fold::X(usize::from_str(coord)?))),
            "y" => Ok(Input::Instruction(Fold::Y(usize::from_str(coord)?))),
            _ => Err(ParseError::Invalid(instruction.to_owned())),
        },
        _ => Err(ParseError::Incomplete(instruction.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use parsing::Parser;

    use super::*;

    #[test]
    fn parse() {
        let parser = Parser;

        let input = ["6,10", "0,14", "fold along y=7", "fold along x=5"];

        let reference = [
            Input::Coord(Coord(6, 10)),
            Input::Coord(Coord(0, 14)),
            Input::Instruction(Fold::Y(7)),
            Input::Instruction(Fold::X(5)),
        ];

        for (input, reference) in input.iter().zip(reference.iter()) {
            assert_eq!(&parser.parse(input).unwrap(), reference);
        }
    }
}
