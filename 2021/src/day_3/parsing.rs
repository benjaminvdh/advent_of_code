use std::num::IntErrorKind;

use crate::parsing::{self, ParseError};

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = u32;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        Self::Output::from_str_radix(line, 2).map_err(|e| match e.kind() {
            IntErrorKind::Empty => ParseError::Incomplete(line.to_owned()),
            _ => ParseError::Invalid(line.to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::Parser;

    use super::*;

    #[test]
    fn valid_input() {
        let parser = Parser;
        let input = "101010";

        assert_eq!(parser.parse(input).unwrap(), 42);
    }

    #[test]
    fn invalid_input() {
        let parser = Parser;
        let input = "forty-two";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Invalid(line) if &line == "forty-two")
        );
    }

    #[test]
    fn invalid_digit() {
        let parser = Parser;
        let input = "101020";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Invalid(line) if &line == "101020")
        );
    }

    #[test]
    fn empty_input() {
        let parser = Parser;
        let input = "";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Incomplete(line) if &line == "")
        );
    }
}
