use crate::parsing::{self, ParseError};

use crate::day_8::pattern::{Pattern, Segment};

pub struct Parser;

impl parsing::Parser for Parser {
    type Output = ([Pattern; 10], [Pattern; 4]);

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        let mut splits = line.split('|');

        match (splits.next(), splits.next()) {
            (Some(signals), Some(outputs)) => {
                Ok((parse_patterns(signals)?, parse_patterns(outputs)?))
            }
            _ => Err(ParseError::Incomplete(line.to_owned())),
        }
    }
}

fn parse_patterns<const N: usize>(patterns: &str) -> Result<[Pattern; N], ParseError> {
    patterns
        .split_whitespace()
        .map(|pattern| parse_pattern(pattern))
        .collect::<Result<Vec<_>, ParseError>>()?
        .try_into()
        .map_err(|_| ParseError::Incomplete(patterns.to_owned()))
}

fn parse_pattern(pattern: &str) -> Result<Pattern, ParseError> {
    let segments: Vec<Segment> = pattern
        .chars()
        .map(|c| parse_segment(c))
        .collect::<Result<_, _>>()?;

    Ok(segments
        .into_iter()
        .fold(Pattern::new(), |pattern, segment| pattern | segment))
}

fn parse_segment(segment: char) -> Result<Segment, ParseError> {
    match segment {
        'a' => Ok(Segment::A),
        'b' => Ok(Segment::B),
        'c' => Ok(Segment::C),
        'd' => Ok(Segment::D),
        'e' => Ok(Segment::E),
        'f' => Ok(Segment::F),
        'g' => Ok(Segment::G),
        _ => Err(ParseError::Invalid(String::from(segment))),
    }
}

#[cfg(test)]
mod tests {
    use crate::day_8::pattern::Segment as Seg;
    use crate::parsing::Parser;

    use super::*;

    #[test]
    fn parse() {
        let parser = Parser;
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";

        let (signals, outputs) = parser.parse(input).unwrap();

        let reference = [
            Seg::B | Seg::E,
            Seg::C | Seg::F | Seg::B | Seg::E | Seg::G | Seg::A | Seg::D,
            Seg::C | Seg::B | Seg::D | Seg::G | Seg::E | Seg::F,
            Seg::F | Seg::G | Seg::A | Seg::E | Seg::C | Seg::D,
            Seg::C | Seg::G | Seg::E | Seg::B,
            Seg::F | Seg::D | Seg::C | Seg::G | Seg::E,
            Seg::A | Seg::G | Seg::E | Seg::B | Seg::F | Seg::D,
            Seg::F | Seg::E | Seg::C | Seg::D | Seg::B,
            Seg::F | Seg::A | Seg::B | Seg::C | Seg::D,
            Seg::E | Seg::D | Seg::B,
        ];
        assert_eq!(&signals, &reference);

        let reference = [
            Seg::F | Seg::D | Seg::G | Seg::A | Seg::C | Seg::B | Seg::E,
            Seg::C | Seg::E | Seg::F | Seg::D | Seg::B,
            Seg::C | Seg::E | Seg::F | Seg::B | Seg::G | Seg::D,
            Seg::G | Seg::C | Seg::B | Seg::E,
        ];
        assert_eq!(&outputs, &reference);
    }
}
