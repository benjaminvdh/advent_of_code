struct Step {
    label: String,
    operation: char,
    focal_length: usize,
}

impl From<&str> for Step {
    fn from(string: &str) -> Self {
        let operation_index = string.chars().position(|c| c == '-' || c == '=').unwrap();

        let label = string[0..operation_index].to_owned();

        let operation = string.chars().nth(operation_index).unwrap();

        let focal_length = if operation == '=' {
            string.chars().nth(operation_index + 1).unwrap() as usize - '0' as usize
        } else {
            0
        };

        Self {
            label,
            operation,
            focal_length,
        }
    }
}

struct Boxes(Vec<Vec<(String, usize)>>);

impl Boxes {
    pub fn new() -> Self {
        Self([vec![]].into_iter().cycle().take(256).collect())
    }

    pub fn apply(self, step: &Step) -> Self {
        match step.operation {
            '=' => self.add(step),
            '-' => self.remove(step),
            _ => panic!(),
        }
    }

    pub fn get_focusing_power(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(box_index, boxes)| {
                (box_index + 1)
                    * boxes
                        .iter()
                        .enumerate()
                        .map(|(lens_index, (_, focal_length))| (lens_index + 1) * focal_length)
                        .sum::<usize>()
            })
            .sum()
    }

    fn add(mut self, step: &Step) -> Self {
        let hash = run_hash(&step.label);

        if let Some(lens_box) = self.0[hash]
            .iter_mut()
            .find(|lens_box| lens_box.0 == step.label)
        {
            lens_box.1 = step.focal_length;
        } else {
            self.0[hash].push((step.label.to_owned(), step.focal_length))
        }

        self
    }

    fn remove(mut self, step: &Step) -> Self {
        let hash = run_hash(&step.label);

        if let Some(index) = self.0[hash]
            .iter()
            .position(|lens_box| lens_box.0 == step.label)
        {
            let _ = self.0[hash].remove(index);
        }

        self
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input
            .split(',')
            .map(|string| string.trim().to_owned())
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|seq| run_hash(seq)).sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|step| Step::from(step.as_str()))
            .fold(Boxes::new(), |boxes, step| boxes.apply(&step))
            .get_focusing_power()
    }
}

fn run_hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            String::from("rn=1"),
            String::from("cm-"),
            String::from("qp=3"),
            String::from("cm=2"),
            String::from("qp-"),
            String::from("pc=4"),
            String::from("ot=9"),
            String::from("ab=5"),
            String::from("pc-"),
            String::from("pc=6"),
            String::from("ot=7"),
        ]
    }

    #[test]
    fn parsing() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 1320);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 145);
    }
}
