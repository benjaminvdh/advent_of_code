pub use crate::parsing;
pub use crate::parsing::ParseError;

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = Vec<char>;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        Ok(line.chars().collect())
    }
}
