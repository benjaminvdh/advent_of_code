use crate::day_10;

pub fn print_success_message(score: u64) {
    println!("The middle autocomplete score is {}.", score);
}

pub fn get_middle_autocomplete_score(lines: &[Vec<char>]) -> u64 {
    let mut scores: Vec<_> = lines
        .iter()
        .map(|line| get_autocomplete_score(&line))
        .filter(|score| *score != 0)
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

fn get_autocomplete_score(line: &[char]) -> u64 {
    if let Some((&first, rest)) = line.split_first() {
        match day_10::parse(first, rest, vec![]) {
            Ok((_, missing)) => get_total_score(&missing),
            Err(_) => 0,
        }
    } else {
        0
    }
}

fn get_total_score(missing: &[char]) -> u64 {
    missing
        .iter()
        .map(|c| get_incomplete_score(*c))
        .fold(0, |total, score| total * 5 + score)
}

fn get_incomplete_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_total_score() {
        let input = [
            vec![
                '[', '(', '{', '(', '<', '(', '(', ')', ')', '[', ']', '>', '[', '[', '{', '[',
                ']', '{', '<', '(', ')', '<', '>', '>',
            ],
            vec![
                '[', '(', '(', ')', '[', '<', '>', ']', ')', ']', '(', '{', '[', '<', '{', '<',
                '<', '[', ']', '>', '>', '(',
            ],
            vec![
                '{', '(', '[', '(', '<', '{', '}', '[', '<', '>', '[', ']', '}', '>', '{', '[',
                ']', '{', '[', '(', '<', '(', ')', '>',
            ],
            vec![
                '(', '(', '(', '(', '{', '<', '>', '}', '<', '{', '<', '{', '<', '>', '}', '{',
                '[', ']', '{', '[', ']', '{', '}',
            ],
            vec![
                '[', '[', '<', '[', '(', '[', ']', ')', ')', '<', '(', '[', '[', '{', '}', '[',
                '[', '(', ')', ']', ']', ']',
            ],
            vec![
                '[', '{', '[', '{', '(', '{', '}', ']', '{', '}', '}', '(', '[', '{', '[', '{',
                '{', '{', '}', '}', '(', '[', ']',
            ],
            vec![
                '{', '<', '[', '[', ']', ']', '>', '}', '<', '{', '[', '{', '[', '{', '[', ']',
                '{', '(', ')', '[', '[', '[', ']',
            ],
            vec![
                '[', '<', '(', '<', '(', '<', '(', '<', '{', '}', ')', ')', '>', '<', '(', '[',
                ']', '(', '[', ']', '(', ')',
            ],
            vec![
                '<', '{', '(', '[', '(', '[', '[', '(', '<', '>', '(', ')', ')', '{', '}', ']',
                '>', '(', '<', '<', '{', '{',
            ],
            vec![
                '<', '{', '(', '[', '{', '{', '}', '}', '[', '<', '[', '[', '[', '<', '>', '{',
                '}', ']', ']', ']', '>', '[', ']', ']',
            ],
        ];

        assert_eq!(get_middle_autocomplete_score(&input), 288957);
    }
}
