use aoc::Dir;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn advance(&self, grid: &Grid, dir: Dir) -> Option<Self> {
        match dir {
            Dir::N => {
                if self.y > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Dir::E => {
                if self.x < grid.width() - 1 {
                    Some(Self {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Dir::S => {
                if self.y < grid.height() - 1 {
                    Some(Self {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Dir::W => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }
}

fn update_dir(dir: Dir, contents: Contents) -> Vec<Dir> {
    match contents {
        Contents::Empty => vec![dir],
        Contents::Acute => vec![match dir {
            Dir::N => Dir::E,
            Dir::E => Dir::N,
            Dir::S => Dir::W,
            Dir::W => Dir::S,
        }],
        Contents::Grave => vec![match dir {
            Dir::N => Dir::W,
            Dir::E => Dir::S,
            Dir::S => Dir::E,
            Dir::W => Dir::N,
        }],
        Contents::Hor => match dir {
            Dir::N => vec![Dir::W, Dir::E],
            Dir::E => vec![Dir::E],
            Dir::S => vec![Dir::W, Dir::E],
            Dir::W => vec![Dir::W],
        },
        Contents::Ver => match dir {
            Dir::N => vec![Dir::N],
            Dir::E => vec![Dir::N, Dir::S],
            Dir::S => vec![Dir::S],
            Dir::W => vec![Dir::N, Dir::S],
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Beam {
    pos: Point,
    dir: Dir,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Contents {
    Empty,
    Acute,
    Grave,
    Hor,
    Ver,
}

impl From<char> for Contents {
    fn from(c: char) -> Self {
        match c {
            '.' => Contents::Empty,
            '/' => Contents::Acute,
            '\\' => Contents::Grave,
            '-' => Contents::Hor,
            '|' => Contents::Ver,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Cell {
    contents: Contents,
    energized: bool,
    handled: Vec<Beam>,
}

impl Cell {
    fn new(contents: Contents) -> Self {
        Self {
            contents,
            energized: false,
            handled: vec![],
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        Self::new(c.into())
    }
}

type Grid = aoc::Grid<Cell>;

struct Solver;

impl aoc::Solver for Solver {
    type Input = Grid;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        Grid::from_iter(
            input
                .lines()
                .map(|line| line.chars().map(|c| Cell::from(c)).into_iter())
                .into_iter(),
        )
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        energize_grid(
            input,
            Beam {
                pos: Point { x: 0, y: 0 },
                dir: Dir::E,
            },
        )
    }

    fn part_2(grid: &Self::Input) -> Self::Output2 {
        (0..grid.height())
            .map(|y| Beam {
                pos: Point { x: 0, y },
                dir: Dir::E,
            })
            .chain((0..grid.height()).map(|y| Beam {
                pos: Point {
                    x: grid.width() - 1,
                    y,
                },
                dir: Dir::W,
            }))
            .chain((0..grid.width()).map(|x| Beam {
                pos: Point { x, y: 0 },
                dir: Dir::S,
            }))
            .chain((0..grid.width()).map(|x| Beam {
                pos: Point {
                    x,
                    y: grid.height() - 1,
                },
                dir: Dir::N,
            }))
            .map(|beam| energize_grid(grid, beam))
            .max()
            .unwrap()
    }
}

fn energize_grid(grid: &Grid, initial_beam: Beam) -> usize {
    let mut grid: Grid = grid.clone();

    let mut beams = vec![initial_beam];

    loop {
        beams = traverse(&mut grid, beams);

        if beams.is_empty() {
            break;
        }
    }

    (0..grid.height())
        .map(|y| {
            (0..grid.width())
                .filter(|&x| grid.get(x, y).energized)
                .count()
        })
        .sum()
}

fn traverse(grid: &mut Grid, beams: Vec<Beam>) -> Vec<Beam> {
    for beam in beams.iter() {
        let Point { x, y } = beam.pos;
        grid.get_mut(x, y).energized = true;
    }

    beams
        .into_iter()
        .map(|beam| {
            let Point { x, y } = beam.pos;

            if grid.get(x, y).handled.contains(&beam) {
                vec![]
            } else {
                grid.get_mut(x, y).handled.push(beam);

                let new_dirs = update_dir(beam.dir, grid.get(x, y).contents);

                new_dirs
                    .into_iter()
                    .filter_map(|dir| beam.pos.advance(grid, dir).map(|pos| Beam { pos, dir }))
                    .collect::<Vec<_>>()
            }
        })
        .flatten()
        .collect()
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        Grid::from_iter([
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Ver),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Grave),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
            ],
            [
                Cell::new(Contents::Ver),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Hor),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Grave),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Ver),
                Cell::new(Contents::Hor),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Ver),
                Cell::new(Contents::Empty),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Grave),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Acute),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Grave),
                Cell::new(Contents::Grave),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Hor),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Hor),
                Cell::new(Contents::Acute),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Ver),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Ver),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Hor),
                Cell::new(Contents::Ver),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Grave),
            ],
            [
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Acute),
                Cell::new(Contents::Acute),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Ver),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
                Cell::new(Contents::Empty),
            ],
        ])
    }

    #[test]
    fn parsing() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 46);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 51);
    }
}
