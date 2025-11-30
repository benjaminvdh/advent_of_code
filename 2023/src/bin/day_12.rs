use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Status {
    fn from(c: char) -> Self {
        match c {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Record {
    springs: Vec<Status>,
    groups: Vec<usize>,
}

impl Record {
    pub fn get_num_arrangements(&self) -> usize {
        if !self.is_valid() {
            0
        } else if self.is_complete() {
            1
        } else if let Some(first_index) = self
            .springs
            .iter()
            .position(|status| matches!(status, Status::Unknown))
        {
            let mut clone_a = self.clone();
            clone_a.springs[first_index] = Status::Operational;

            let mut clone_b = self.clone();
            clone_b.springs[first_index] = Status::Damaged;

            clone_a.get_num_arrangements() + clone_b.get_num_arrangements()
        } else {
            0
        }
    }

    fn is_valid(&self) -> bool {
        if self
            .springs
            .iter()
            .any(|status| !matches!(status, Status::Unknown))
        {
            return true;
        }

        let splits: Vec<_> = self
            .springs
            .split(|status| !matches!(status, Status::Damaged))
            .filter(|slice| slice.len() != 0)
            .collect();
        if splits.len() > self.groups.len() {
            false
        } else {
            splits
                .iter()
                .zip(self.groups.iter())
                .all(|(split, group)| split.len() <= *group)
        }
    }

    fn is_complete(&self) -> bool {
        if self
            .springs
            .iter()
            .any(|status| matches!(status, Status::Unknown))
        {
            return false;
        }

        let splits: Vec<_> = self
            .springs
            .split(|status| !matches!(status, Status::Damaged))
            .filter(|slice| slice.len() != 0)
            .collect();
        if splits.len() != self.groups.len() {
            false
        } else {
            splits
                .iter()
                .zip(self.groups.iter())
                .all(|(split, group)| split.len() == *group)
        }
    }
}

impl From<&str> for Record {
    fn from(line: &str) -> Self {
        let (springs, groups) = line.split_once(' ').unwrap();

        Self {
            springs: springs.chars().map(|c| c.into()).collect(),
            groups: groups
                .split(',')
                .map(|number| usize::from_str(number).unwrap())
                .collect(),
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<Record>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| line.into()).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|record| record.get_num_arrangements())
            .sum()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
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
            Record {
                springs: vec![
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Operational,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                ],
                groups: vec![1, 1, 3],
            },
            Record {
                springs: vec![
                    Status::Operational,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Operational,
                    Status::Operational,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Operational,
                    Status::Operational,
                    Status::Operational,
                    Status::Unknown,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Operational,
                ],
                groups: vec![1, 1, 3],
            },
            Record {
                springs: vec![
                    Status::Unknown,
                    Status::Damaged,
                    Status::Unknown,
                    Status::Damaged,
                    Status::Unknown,
                    Status::Damaged,
                    Status::Unknown,
                    Status::Damaged,
                    Status::Unknown,
                    Status::Damaged,
                    Status::Unknown,
                    Status::Damaged,
                    Status::Unknown,
                    Status::Damaged,
                    Status::Unknown,
                ],
                groups: vec![1, 3, 1, 6],
            },
            Record {
                springs: vec![
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Operational,
                    Status::Damaged,
                    Status::Operational,
                    Status::Operational,
                    Status::Operational,
                    Status::Damaged,
                    Status::Operational,
                    Status::Operational,
                    Status::Operational,
                ],
                groups: vec![4, 1, 1],
            },
            Record {
                springs: vec![
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Operational,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Operational,
                    Status::Operational,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Operational,
                ],
                groups: vec![1, 6, 5],
            },
            Record {
                springs: vec![
                    Status::Unknown,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Damaged,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                    Status::Unknown,
                ],
                groups: vec![3, 2, 1],
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 21);
    }

    #[test]
    #[ignore = "not yet implemented"]
    #[allow(unreachable_code)]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
