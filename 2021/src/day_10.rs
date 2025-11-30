pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}

fn parse(first: char, line: &[char], mut missing: Vec<char>) -> Result<(&[char], Vec<char>), u32> {
    if is_closing(first) {
        Err(get_score(first))
    } else if is_opening(first) {
        match line.split_first() {
            Some((second, rest)) => parse_remainder(first, *second, rest, missing),
            None => {
                missing.push(get_match(first));
                Ok((line, missing))
            }
        }
    } else {
        Err(0)
    }
}

fn parse_remainder(
    first: char,
    second: char,
    rest: &[char],
    missing: Vec<char>,
) -> Result<(&[char], Vec<char>), u32> {
    if is_opening(second) {
        parse(second, rest, missing).and_then(|(rest, missing)| parse(first, rest, missing))
    } else if is_closing(second) {
        if chars_match(first, second) {
            match rest.split_first() {
                Some((&third, next_substring)) => {
                    if is_opening(third) {
                        parse(third, next_substring, missing)
                    } else {
                        Ok((rest, missing))
                    }
                }
                None => Ok((rest, missing)),
            }
        } else {
            Err(get_score(second))
        }
    } else {
        Err(0)
    }
}

fn chars_match(a: char, b: char) -> bool {
    match (a, b) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn get_match(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => c,
    }
}

fn is_opening(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn is_closing(c: char) -> bool {
    match c {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false,
    }
}

fn get_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
