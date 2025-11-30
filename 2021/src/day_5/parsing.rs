use std::str::FromStr;

use crate::parsing;
use crate::parsing::ParseError;

pub struct Parser;

use crate::day_5::line::Line;
use crate::day_5::point::Point;

impl parsing::Parser for Parser {
    type Output = Line;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        let mut iter = line.split_whitespace();

        match (iter.next(), iter.next(), iter.next()) {
            (Some(start), Some(_), Some(end)) => {
                Ok(Line::new(parse_point(start)?, parse_point(end)?))
            }
            _ => Err(ParseError::Incomplete(line.to_owned())),
        }
    }
}

fn parse_point(string: &str) -> Result<Point, ParseError> {
    let mut iter = string.split(',');

    match (iter.next(), iter.next()) {
        (Some(x), Some(y)) => Ok(Point::new(parse_coord(x)?, parse_coord(y)?)),
        _ => Err(ParseError::Incomplete(string.to_owned())),
    }
}

fn parse_coord(string: &str) -> Result<u32, ParseError> {
    let coord = u32::from_str(string)?;

    Ok(coord)
}

#[cfg(test)]
mod tests {
    use parsing::Parser;

    use super::*;

    #[test]
    fn parse() {
        let input = "0,9 -> 5,9";
        let parser = Parser;

        let reference = Line::new(Point::new(0, 9), Point::new(5, 9));

        assert_eq!(parser.parse(input).unwrap(), reference);
    }
}
