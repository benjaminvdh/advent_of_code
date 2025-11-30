use std::str::FromStr;

use crate::day_11::grid::GRID_SIZE;
use crate::parsing::{self, ParseError};

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = [u8; GRID_SIZE];

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        let charges = line
            .chars()
            .map(|c| u8::from_str(&String::from(c)).map_err(|e| ParseError::from(e)))
            .collect::<Result<Vec<_>, _>>()?;

        charges
            .try_into()
            .map_err(|_| ParseError::Invalid(line.to_owned()))
    }
}
