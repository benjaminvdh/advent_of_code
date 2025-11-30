use std::str::FromStr;

pub use crate::parsing;
pub use crate::parsing::ParseError;

pub struct Parser;

use crate::day_7::Crab;

impl parsing::Parser for Parser {
    type Output = Vec<Crab>;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        line.split(',')
            .map(|number| crab_from_string(number))
            .collect()
    }
}

fn crab_from_string(string: &str) -> Result<Crab, ParseError> {
    let position = u32::from_str(string)?;

    Ok(Crab::from(position))
}

#[cfg(test)]
mod tests {
    use crate::parsing::Parser;

    use super::*;

    #[test]
    fn parse() {
        let parser = Parser;

        assert_eq!(
            parser.parse("1,2,3,4,5").unwrap(),
            vec![
                Crab::from(1),
                Crab::from(2),
                Crab::from(3),
                Crab::from(4),
                Crab::from(5),
            ]
        );
    }
}
