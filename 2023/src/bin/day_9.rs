use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Sequence(Vec<i64>);

impl Sequence {
    pub fn extend(&self) -> Self {
        if self.is_zero() {
            let mut sequence = self.0.clone();
            sequence.push(0);
            Self(sequence)
        } else {
            let child = Self(self.get_differences());
            let extended_child = child.extend();
            let mut sequence = self.0.clone();
            sequence.push(*sequence.last().unwrap() + extended_child.last());
            Self(sequence)
        }
    }

    pub fn pre_extend(&self) -> Self {
        if self.is_zero() {
            let mut sequence = self.0.clone();
            sequence.insert(0, 0);
            Self(sequence)
        } else {
            let child = Self(self.get_differences());
            let extended_child = child.pre_extend();
            let mut sequence = self.0.clone();
            sequence.insert(0, *sequence.first().unwrap() - extended_child.first());
            Self(sequence)
        }
    }

    pub fn last(&self) -> i64 {
        *self.0.last().unwrap()
    }

    pub fn first(&self) -> i64 {
        *self.0.first().unwrap()
    }

    fn get_differences(&self) -> Vec<i64> {
        self.0
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect()
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|val| *val == 0)
    }
}

impl From<&str> for Sequence {
    fn from(line: &str) -> Self {
        Self(
            line.split_whitespace()
                .map(|number| i64::from_str(number).unwrap())
                .collect(),
        )
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for val in self.0.iter() {
            write!(f, "{val:8} ")?;
        }

        writeln!(f)?;

        if !self.is_zero() {
            let mut indentation = 0;
            let mut child_sequence = Self(self.get_differences());

            loop {
                indentation += 4;
                write!(f, "{:indentation$}", ' ')?;

                for val in child_sequence.0.iter() {
                    write!(f, "{val:8} ")?;
                }

                writeln!(f)?;

                if child_sequence.is_zero() {
                    break;
                }

                child_sequence = Self(child_sequence.get_differences());
            }
        }

        Ok(())
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<Sequence>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| Sequence::from(line)).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|sequence| sequence.extend())
            .map(|sequence| sequence.last())
            .sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|sequence| sequence.pre_extend())
            .map(|sequence| sequence.first())
            .sum()
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
            Sequence(vec![0, 3, 6, 9, 12, 15]),
            Sequence(vec![1, 3, 6, 10, 15, 21]),
            Sequence(vec![10, 13, 16, 21, 30, 45]),
        ]
    }

    #[test]
    fn parsing() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 114);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 2);
    }
}
