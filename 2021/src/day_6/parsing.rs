use std::str::FromStr;

pub use crate::parsing;
pub use crate::parsing::ParseError;

pub struct Parser;

use crate::day_6::LanternFish;

impl parsing::Parser for Parser {
    type Output = Vec<LanternFish>;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        line.split(',')
            .map(|number| lanternfish_from_string(number))
            .collect()
    }
}

fn lanternfish_from_string(string: &str) -> Result<LanternFish, ParseError> {
    let age = u8::from_str(string)?;

    Ok(LanternFish::from(age))
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
                LanternFish::from(1),
                LanternFish::from(2),
                LanternFish::from(3),
                LanternFish::from(4),
                LanternFish::from(5),
            ]
        );
    }
}
