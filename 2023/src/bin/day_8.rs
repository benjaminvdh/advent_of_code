use std::collections::{HashMap, HashSet};

struct Solver;

#[derive(Debug, PartialEq)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    pub fn get_dest(&self, nodes: &HashMap<String, Node>, dirs: &[Dir]) -> String {
        dirs.iter().fold(self.name.clone(), |name, dir| match dir {
            Dir::Left => nodes.get(&name).unwrap().left.clone(),
            Dir::Right => nodes.get(&name).unwrap().right.clone(),
        })
    }
}

impl From<&str> for Node {
    fn from(line: &str) -> Self {
        let mut iter = line.chars();

        let name = string_from_iter(&mut iter);

        let mut iter = iter.skip(4);
        let left = string_from_iter(&mut iter);

        let mut iter = iter.skip(2);
        let right = string_from_iter(&mut iter);

        Self { name, left, right }
    }
}

fn string_from_iter(iter: &mut impl Iterator<Item = char>) -> String {
    String::from_iter(
        [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ]
        .iter(),
    )
}

impl aoc::Solver for Solver {
    type Input = (Vec<Dir>, HashMap<String, Node>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        let (dirs, nodes) = input.split_once("\n\n").unwrap();

        let dirs = dirs.chars().map(|c| c.into()).collect();
        let nodes = nodes
            .lines()
            .map(|line| {
                let node = Node::from(line);
                (node.name.clone(), node)
            })
            .collect();

        (dirs, nodes)
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (dirs, nodes) = input;

        let start_nodes = nodes.keys().filter(|node| node.as_str() == "AAA");
        get_num_steps(dirs, nodes, start_nodes)
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (dirs, nodes) = input;

        let start_nodes = nodes
            .keys()
            .filter(|node| node.chars().last().unwrap() == 'A');
        get_num_steps(dirs, nodes, start_nodes)
    }
}

fn get_num_steps<'a>(
    dirs: &[Dir],
    nodes: &'a HashMap<String, Node>,
    start_nodes: impl Iterator<Item = &'a String>,
) -> usize {
    dirs.len()
        * start_nodes
            .map(|name| {
                let mut current = name.to_string();
                let mut cycles = HashSet::new();

                while !cycles.contains(&current) {
                    cycles.insert(current.clone());
                    current = nodes.get(&current).unwrap().get_dest(&nodes, &dirs);
                }

                cycles.len() - 1
            })
            .product::<usize>()
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        (
            vec![Dir::Left, Dir::Left, Dir::Right],
            [
                Node {
                    name: "AAA".to_string(),
                    left: "BBB".to_string(),
                    right: "BBB".to_string(),
                },
                Node {
                    name: "BBB".to_string(),
                    left: "AAA".to_string(),
                    right: "ZZZ".to_string(),
                },
                Node {
                    name: "ZZZ".to_string(),
                    left: "ZZZ".to_string(),
                    right: "ZZZ".to_string(),
                },
            ]
            .into_iter()
            .map(|node| (node.name.clone(), node))
            .collect(),
        )
    }

    #[test]
    fn parsing() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 6);
    }

    fn get_input_2() -> <Solver as aoc::Solver>::Input {
        (
            vec![Dir::Left, Dir::Right],
            [
                Node {
                    name: "11A".to_string(),
                    left: "11B".to_string(),
                    right: "XXX".to_string(),
                },
                Node {
                    name: "11B".to_string(),
                    left: "XXX".to_string(),
                    right: "11Z".to_string(),
                },
                Node {
                    name: "11Z".to_string(),
                    left: "11B".to_string(),
                    right: "XXX".to_string(),
                },
                Node {
                    name: "22A".to_string(),
                    left: "22B".to_string(),
                    right: "XXX".to_string(),
                },
                Node {
                    name: "22B".to_string(),
                    left: "22C".to_string(),
                    right: "22C".to_string(),
                },
                Node {
                    name: "22C".to_string(),
                    left: "22Z".to_string(),
                    right: "22Z".to_string(),
                },
                Node {
                    name: "22Z".to_string(),
                    left: "22B".to_string(),
                    right: "22B".to_string(),
                },
                Node {
                    name: "XXX".to_string(),
                    left: "XXX".to_string(),
                    right: "XXX".to_string(),
                },
            ]
            .into_iter()
            .map(|node| (node.name.clone(), node))
            .collect(),
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input_2()), 6);
    }
}
