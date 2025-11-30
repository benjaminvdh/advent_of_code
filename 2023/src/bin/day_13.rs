struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<Vec<Vec<char>>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input
            .split("\n\n")
            .map(|split| split.lines().map(|line| line.chars().collect()).collect())
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|pattern| get_pattern_score(pattern)).sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|pattern| get_smudge_pattern_score(pattern))
            .sum()
    }
}

fn get_pattern_score(pattern: &[Vec<char>]) -> usize {
    let y_score = (1..pattern.len())
        .filter_map(|split| rows_reflect_at(pattern, split))
        .max()
        .unwrap_or(0);

    let x_score = (1..pattern[0].len())
        .filter_map(|split| cols_reflect_at(pattern, split))
        .max()
        .unwrap_or(0);

    y_score * 100 + x_score
}

fn cols_reflect_at(lines: &[Vec<char>], vertical_split: usize) -> Option<usize> {
    let mut forward = vertical_split;
    let mut backward = vertical_split - 1;

    loop {
        if lines.iter().any(|line| line[forward] != line[backward]) {
            return None;
        }

        if forward == lines[0].len() - 1 || backward == 0 {
            break;
        }

        forward += 1;
        backward -= 1;
    }

    Some(vertical_split)
}

fn rows_reflect_at(lines: &[Vec<char>], horizontal_split: usize) -> Option<usize> {
    let mut forward = horizontal_split;
    let mut backward = horizontal_split - 1;

    loop {
        if lines[forward] != lines[backward] {
            return None;
        }

        if forward == lines.len() - 1 || backward == 0 {
            break;
        }

        forward += 1;
        backward -= 1;
    }

    Some(horizontal_split)
}

fn get_smudge_pattern_score(pattern: &[Vec<char>]) -> usize {
    let y_score = (1..pattern.len())
        .filter_map(|split| rows_smudge_reflect_at(pattern, split))
        .max()
        .unwrap_or(0);

    let x_score = (1..pattern[0].len())
        .filter_map(|split| cols_smudge_reflect_at(pattern, split))
        .max()
        .unwrap_or(0);

    y_score * 100 + x_score
}

fn cols_smudge_reflect_at(lines: &[Vec<char>], vertical_split: usize) -> Option<usize> {
    let mut forward = vertical_split;
    let mut backward = vertical_split - 1;
    let mut num_mismatch = 0;

    loop {
        num_mismatch += lines
            .iter()
            .filter(|line| line[forward] != line[backward])
            .count();

        if num_mismatch > 1 || forward == lines[0].len() - 1 || backward == 0 {
            break;
        }

        forward += 1;
        backward -= 1;
    }

    if num_mismatch == 1 {
        Some(vertical_split)
    } else {
        None
    }
}

fn rows_smudge_reflect_at(lines: &[Vec<char>], horizontal_split: usize) -> Option<usize> {
    let mut forward = horizontal_split;
    let mut backward = horizontal_split - 1;
    let mut num_mismatch = 0;

    loop {
        num_mismatch += lines[forward]
            .iter()
            .zip(lines[backward].iter())
            .filter(|(a, b)| a != b)
            .count();

        if num_mismatch > 1 || forward == lines.len() - 1 || backward == 0 {
            break;
        }

        forward += 1;
        backward -= 1;
    }
    if num_mismatch == 1 {
        Some(horizontal_split)
    } else {
        None
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            vec![
                vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['#', '.', '#', '.', '#', '#', '.', '#', '.'],
            ],
            vec![
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
                vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
            ],
        ]
    }

    #[test]
    fn parsing() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 405);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 400);
    }
}
