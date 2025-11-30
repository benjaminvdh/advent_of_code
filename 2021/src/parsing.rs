use std::fmt;
use std::num::{IntErrorKind, ParseIntError};

#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    Incomplete(String),
    Invalid(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Incomplete(line) => write!(f, "Incomplete line `{}`", line),
            ParseError::Invalid(line) => write!(f, "Failed to parse `{}`", line),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        match e.kind() {
            IntErrorKind::Empty => ParseError::Incomplete(e.to_string()),
            _ => ParseError::Invalid(e.to_string()),
        }
    }
}

pub trait Parser {
    type Output;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError>;
}
