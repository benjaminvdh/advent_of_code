use std::num::IntErrorKind;
use std::str::FromStr;

use crate::parsing::{self, ParseError};

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = i32;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        i32::from_str(line).map_err(|e| match e.kind() {
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
    fn parse_valid_input() {
        let parser = Parser;
        let input = "284";

        assert_eq!(parser.parse(input).unwrap(), 284);
    }

    #[test]
    fn parse_negative_input() {
        let parser = Parser;
        let input = "-931";

        assert_eq!(parser.parse(input).unwrap(), -931);
    }

    #[test]
    fn parse_empty_input() {
        let parser = Parser;
        let input = "";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Incomplete(line) if &line == "")
        );
    }

    #[test]
    fn parse_invalid_input() {
        let parser = Parser;
        let input = "three";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Invalid(line) if &line == "three")
        );
    }
}
