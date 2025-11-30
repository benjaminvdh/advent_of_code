use std::str::FromStr;

use aoc::Dir;

#[derive(Debug, PartialEq)]
struct Instruction {
    dir: Dir,
    steps: usize,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let mut iter = line.split_whitespace();

        let dir = match iter.next().unwrap() {
            "U" => Dir::N,
            "R" => Dir::E,
            "D" => Dir::S,
            "L" => Dir::W,
            _ => panic!(),
        };

        let steps = usize::from_str(iter.next().unwrap()).unwrap();

        Self { dir, steps }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Ext,
    Int,
    Edge,
}

type Grid = aoc::Grid<State>;

struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<Instruction>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| line.into()).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let positions = trace_instructions(input);
        let (min_x, min_y, max_x, max_y) = get_grid_size(&positions);

        let mut grid = Grid::new(
            (max_x - min_x) as usize + 1,
            (max_y - min_y) as usize + 1,
            State::Int,
        );

        for position in translate_positions(positions, min_x, min_y) {
            *grid.get_mut(position.0, position.1) = State::Edge;
        }

        for x in 0..grid.width() {
            let left = grid.get_mut(x, 0);
            if *left == State::Int {
                *left = State::Ext;
            }

            let right = grid.get_mut(x, grid.height() - 1);
            if *right == State::Int {
                *right = State::Ext;
            }
        }

        for y in 0..grid.height() {
            let top = grid.get_mut(0, y);
            if *top == State::Int {
                *top = State::Ext;
            }

            let bot = grid.get_mut(grid.width() - 1, y);
            if *bot == State::Int {
                *bot = State::Ext;
            }
        }

        loop {
            let mut changed = false;

            for y in 1..grid.height() - 1 {
                for x in 1..grid.width() - 1 {
                    if *grid.get(x, y) != State::Int {
                        continue;
                    }

                    if *grid.get(x, y - 1) == State::Ext
                        || *grid.get(x + 1, y) == State::Ext
                        || *grid.get(x, y + 1) == State::Ext
                        || *grid.get(x - 1, y) == State::Ext
                    {
                        changed = true;
                        *grid.get_mut(x, y) = State::Ext;
                    }
                }
            }

            if !changed {
                break;
            }
        }

        (0..grid.height())
            .map(|y| {
                (0..grid.width())
                    .filter(|&x| *grid.get(x, y) != State::Ext)
                    .count()
            })
            .sum()
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn trace_instructions(instructions: &[Instruction]) -> Vec<(isize, isize)> {
    let mut positions = vec![];
    let mut cur = (0, 0);

    for instr in instructions {
        for _ in 0..instr.steps {
            cur = match instr.dir {
                Dir::N => (cur.0, cur.1 - 1),
                Dir::E => (cur.0 + 1, cur.1),
                Dir::S => (cur.0, cur.1 + 1),
                Dir::W => (cur.0 - 1, cur.1),
            };

            positions.push(cur);
        }
    }

    positions
}

fn translate_positions(
    positions: Vec<(isize, isize)>,
    min_x: isize,
    min_y: isize,
) -> Vec<(usize, usize)> {
    positions
        .into_iter()
        .map(|(x, y)| ((x - min_x) as usize, (y - min_y) as usize))
        .collect()
}

fn get_grid_size(positions: &[(isize, isize)]) -> (isize, isize, isize, isize) {
    positions.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(min_x, min_y, max_x, max_y), &(x, y)| {
            (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
        },
    )
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        vec![
            Instruction {
                dir: Dir::E,
                steps: 6,
            },
            Instruction {
                dir: Dir::S,
                steps: 5,
            },
            Instruction {
                dir: Dir::W,
                steps: 2,
            },
            Instruction {
                dir: Dir::S,
                steps: 2,
            },
            Instruction {
                dir: Dir::E,
                steps: 2,
            },
            Instruction {
                dir: Dir::S,
                steps: 2,
            },
            Instruction {
                dir: Dir::W,
                steps: 5,
            },
            Instruction {
                dir: Dir::N,
                steps: 2,
            },
            Instruction {
                dir: Dir::W,
                steps: 1,
            },
            Instruction {
                dir: Dir::N,
                steps: 2,
            },
            Instruction {
                dir: Dir::E,
                steps: 2,
            },
            Instruction {
                dir: Dir::N,
                steps: 3,
            },
            Instruction {
                dir: Dir::W,
                steps: 2,
            },
            Instruction {
                dir: Dir::N,
                steps: 2,
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 62);
    }

    #[test]
    #[ignore = "not yet implemented"]
    #[allow(unreachable_code)]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
