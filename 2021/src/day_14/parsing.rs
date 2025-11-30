use crate::parsing::{self, ParseError};

pub struct Parser;

#[derive(Debug, PartialEq)]
pub enum Polymer {
    Empty,
    Rule(String, String),
    Template(String),
}

impl parsing::Parser for Parser {
    type Output = Polymer;

    fn parse(&self, line: &str) -> Result<Self::Output, ParseError> {
        if line.contains("->") {
            parse_rule(line)
        } else if !line.is_empty() {
            Ok(parse_template(line))
        } else {
            Ok(Polymer::Empty)
        }
    }
}

fn parse_rule(line: &str) -> Result<Polymer, ParseError> {
    let mut iter = line.split(" -> ");

    match (iter.next(), iter.next()) {
        (Some(from), Some(to)) => {
            let mut combined = String::from(&from[0..from.chars().count() - 1]);
            combined.push_str(to);

            Ok(Polymer::Rule(from.to_owned(), combined))
        }
        _ => Err(ParseError::Incomplete(line.to_owned())),
    }
}

fn parse_template(line: &str) -> Polymer {
    Polymer::Template(line.to_owned())
}

#[cfg(test)]
mod tests {
    use parsing::Parser;

    use super::*;

    #[test]
    fn parse() {
        let parser = Parser;

        let input = [
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];

        let reference = [
            Polymer::Template(String::from("NNCB")),
            Polymer::Empty,
            Polymer::Rule(String::from("CH"), String::from("CB")),
            Polymer::Rule(String::from("HH"), String::from("HN")),
            Polymer::Rule(String::from("CB"), String::from("CH")),
            Polymer::Rule(String::from("NH"), String::from("NC")),
            Polymer::Rule(String::from("HB"), String::from("HC")),
            Polymer::Rule(String::from("HC"), String::from("HB")),
            Polymer::Rule(String::from("HN"), String::from("HC")),
            Polymer::Rule(String::from("NN"), String::from("NC")),
            Polymer::Rule(String::from("BH"), String::from("BH")),
            Polymer::Rule(String::from("NC"), String::from("NB")),
            Polymer::Rule(String::from("NB"), String::from("NB")),
            Polymer::Rule(String::from("BN"), String::from("BB")),
            Polymer::Rule(String::from("BB"), String::from("BN")),
            Polymer::Rule(String::from("BC"), String::from("BB")),
            Polymer::Rule(String::from("CC"), String::from("CN")),
            Polymer::Rule(String::from("CN"), String::from("CC")),
        ];

        for (input, reference) in input.iter().zip(reference.iter()) {
            assert_eq!(&parser.parse(input).unwrap(), reference);
        }
    }
}
