use std::str::FromStr;

struct Solver;

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn get_num_winning_strategies(&self) -> usize {
        (1..self.time)
            .map(|hold_time| hold_time * (self.time - hold_time))
            .filter(|distance| *distance > self.record)
            .count()
    }
}

impl From<(u64, u64)> for Race {
    fn from(tuple: (u64, u64)) -> Self {
        Self {
            time: tuple.0,
            record: tuple.1,
        }
    }
}

impl aoc::Solver for Solver {
    type Input = Vec<Race>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        let mut line_iter = input.lines();

        let times = parse_line(line_iter.next().unwrap());
        let records = parse_line(line_iter.next().unwrap());

        times
            .into_iter()
            .zip(records.into_iter())
            .map(|(time, record)| Race::from((time, record)))
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|race| race.get_num_winning_strategies())
            .product()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let total_time_string = input
            .iter()
            .fold(String::new(), |acc, race| acc + &race.time.to_string());
        let total_record_string = input
            .iter()
            .fold(String::new(), |acc, race| acc + &race.record.to_string());

        let mega_race = Race {
            time: u64::from_str(&total_time_string).unwrap(),
            record: u64::from_str(&total_record_string).unwrap(),
        };

        mega_race.get_num_winning_strategies()
    }
}

fn parse_line(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split_whitespace()
        .skip(1)
        .map(|number| u64::from_str(number).unwrap())
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            Race { time: 7, record: 9 },
            Race {
                time: 15,
                record: 40,
            },
            Race {
                time: 30,
                record: 200,
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 288);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 71503);
    }
}
