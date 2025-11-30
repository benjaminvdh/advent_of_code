use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::parsing::{ParseError, Parser};

#[derive(Debug)]
pub struct NoInputFile;

impl fmt::Display for NoInputFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Usage: <executable> <path to input>")
    }
}

#[derive(Debug)]
pub enum InputError {
    IncorrectUsage(NoInputFile),
    Io(io::Error),
    Parsing(ParseError),
}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> Self {
        InputError::Io(e)
    }
}

impl From<NoInputFile> for InputError {
    fn from(e: NoInputFile) -> Self {
        InputError::IncorrectUsage(e)
    }
}

impl From<ParseError> for InputError {
    fn from(e: ParseError) -> Self {
        InputError::Parsing(e)
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::IncorrectUsage(e) => write!(f, "{}", e),
            InputError::Io(e) => write!(f, "{}", e),
            InputError::Parsing(e) => write!(f, "{}", e),
        }
    }
}

pub fn get_input<P: Parser>(parser: &P) -> Result<Vec<P::Output>, InputError> {
    let filename = get_input_file_name(env::args())?;
    let contents = fs::read_to_string(&filename)?;
    let input = parse(parser, &contents)?;

    Ok(input)
}

fn get_input_file_name(mut args: impl Iterator<Item = String>) -> Result<PathBuf, NoInputFile> {
    args.nth(1)
        .map(|filename| PathBuf::from(filename))
        .ok_or(NoInputFile)
}

fn parse<P: Parser>(parser: &P, input: &str) -> Result<Vec<P::Output>, ParseError> {
    input.lines().map(|line| parser.parse(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_input_path() {
        let args = vec![String::from("./executable"), String::from("input_path")];

        assert_eq!(
            get_input_file_name(args.into_iter()).unwrap(),
            PathBuf::from("input_path")
        );
    }

    #[test]
    fn no_input_path() {
        let args = vec![String::from("./executable")];

        assert!(matches!(
            get_input_file_name(args.into_iter()).unwrap_err(),
            NoInputFile
        ));
    }

    #[test]
    fn no_arguments() {
        let args = vec![];

        assert!(matches!(
            get_input_file_name(args.into_iter()).unwrap_err(),
            NoInputFile
        ));
    }

    #[test]
    fn extra_arguments() {
        let args = vec![
            String::from("./executable"),
            String::from("input_path"),
            String::from("--run"),
        ];

        assert_eq!(
            get_input_file_name(args.into_iter()).unwrap(),
            PathBuf::from("input_path")
        );
    }
}
