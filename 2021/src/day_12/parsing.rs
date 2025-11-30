use crate::parsing::{self, ParseError};

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = (String, String);

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        let mut iter = line.split('-');

        match (iter.next(), iter.next()) {
            (Some(from), Some(to)) => Ok((from.to_owned(), to.to_owned())),
            _ => Err(ParseError::Incomplete(line.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use parsing::Parser;

    use super::*;

    #[test]
    fn parse() {
        let parser = Parser;

        let input = ["start-A", "start-b", "A-end"];

        let reference = [
            (String::from("start"), String::from("A")),
            (String::from("start"), String::from("b")),
            (String::from("A"), String::from("end")),
        ];

        for (input, reference) in input.iter().zip(reference.iter()) {
            assert_eq!(&parser.parse(input).unwrap(), reference);
        }
    }
}
