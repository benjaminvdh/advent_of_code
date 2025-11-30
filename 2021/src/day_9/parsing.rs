use std::str::FromStr;

pub use crate::parsing;
pub use crate::parsing::ParseError;

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = Vec<u32>;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        line.chars()
            .map(|number| u32::from_str(&String::from(number)).map_err(|e| ParseError::from(e)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::Parser;

    use super::*;

    #[test]
    fn parse() {
        let parser = Parser;

        assert_eq!(parser.parse("12345").unwrap(), vec![1, 2, 3, 4, 5]);
    }
}
