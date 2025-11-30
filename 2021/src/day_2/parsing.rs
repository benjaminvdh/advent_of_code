use std::num::IntErrorKind;
use std::str::FromStr;

use crate::day_2::command::Command;
use crate::parsing::{self, ParseError};

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = Command;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        let mut splits = line.split_whitespace();

        match (splits.next(), splits.next()) {
            (Some(command), Some(delta)) => parse_command(command, delta),
            _ => Err(ParseError::Incomplete(line.to_owned())),
        }
    }
}

fn parse_command(command: &str, delta: &str) -> Result<Command, ParseError> {
    let delta = parse_delta(delta)?;

    match command {
        "forward" => Ok(Command::Forward(delta)),
        "up" => Ok(Command::Up(delta)),
        "down" => Ok(Command::Down(delta)),
        _ => Err(ParseError::Invalid(command.to_owned())),
    }
}

fn parse_delta(delta: &str) -> Result<i32, ParseError> {
    i32::from_str(delta).map_err(|e| match e.kind() {
        IntErrorKind::Empty => ParseError::Incomplete(delta.to_owned()),
        _ => ParseError::Invalid(delta.to_owned()),
    })
}

#[cfg(test)]
mod tests {
    use crate::parsing::Parser;

    use super::*;

    #[test]
    fn parse_forward() {
        let parser = Parser;
        let input = "forward 5";

        assert!(matches!(parser.parse(input).unwrap(), Command::Forward(5)));
    }

    #[test]
    fn parse_up() {
        let parser = Parser;
        let input = "up 3";

        assert!(matches!(parser.parse(input).unwrap(), Command::Up(3)));
    }

    #[test]
    fn parse_down() {
        let parser = Parser;
        let input = "down 5";

        assert!(matches!(parser.parse(input).unwrap(), Command::Down(5)));
    }

    #[test]
    fn parse_negative_delta() {
        let parser = Parser;
        let input = "down -4";

        assert!(matches!(parser.parse(input).unwrap(), Command::Down(-4)));
    }

    #[test]
    fn parse_missing_delta() {
        let parser = Parser;
        let input = "down";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Incomplete(line) if &line == "down")
        )
    }

    #[test]
    fn parse_invalid_delta() {
        let parser = Parser;
        let input = "down alot";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Invalid(line) if &line == "alot")
        )
    }

    #[test]
    fn parse_unknown() {
        let parser = Parser;
        let input = "back 4";

        assert!(
            matches!(parser.parse(input).unwrap_err(), ParseError::Invalid(line) if &line == "back")
        )
    }
}
