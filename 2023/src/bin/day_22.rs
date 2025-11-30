use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Coord {
    fn from(string: &str) -> Self {
        let mut splits = string.split(',');

        Self {
            x: i64::from_str(splits.next().unwrap()).unwrap(),
            y: i64::from_str(splits.next().unwrap()).unwrap(),
            z: i64::from_str(splits.next().unwrap()).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Brick {
    start: Coord,
    end: Coord,
}

impl Brick {
    fn overlaps(&self, other: &Self) -> bool {
        (self.start.x..=self.end.x).any(|x1| {
            (self.start.y..=self.end.y).any(|y1| {
                (other.start.x..=other.end.x)
                    .any(|x2| (other.start.y..=other.end.y).any(|y2| x1 == x2 && y1 == y2))
            })
        })
    }

    fn supports(&self, other: &Self) -> bool {
        &self != &other && self.overlaps(other) && self.end.z + 1 == other.start.z
    }
}

impl From<&str> for Brick {
    fn from(string: &str) -> Self {
        let (start, end) = string.split_once('~').unwrap();

        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}

fn drop_bricks(bricks: &mut Vec<Brick>) {
    loop {
        if let Some(brick) = get_droppable_brick(bricks) {
            let diff = bricks[brick].start.z
                - get_brick_below(&bricks, &bricks[brick])
                    .map(|support| bricks[support].end.z + 1)
                    .unwrap_or(1);

            bricks[brick].start.z -= diff;
            bricks[brick].end.z -= diff;
        } else {
            break;
        }
    }
}

fn get_droppable_brick(bricks: &Vec<Brick>) -> Option<usize> {
    bricks
        .iter()
        .position(|brick| brick.start.z > 1 && !bricks.iter().any(|other| other.supports(brick)))
}

fn get_brick_below(bricks: &[Brick], brick: &Brick) -> Option<usize> {
    bricks
        .iter()
        .enumerate()
        .filter(|(_, other)| brick.overlaps(other) && other.end.z < brick.start.z)
        .max_by_key(|(_, brick)| brick.end.z)
        .map(|(i, _)| i)
}

fn get_num_disintegrable_bricks(bricks: &Vec<Brick>) -> usize {
    bricks
        .iter()
        .filter(|support| {
            !bricks.iter().any(|brick| {
                support.supports(brick)
                    && bricks
                        .iter()
                        .filter(|support| support.supports(brick))
                        .count()
                        == 1
            })
        })
        .count()
}

#[allow(unused)]
fn print(bricks: &[Brick]) {
    let x_min = bricks.iter().map(|brick| brick.start.x).min().unwrap_or(0);
    let x_max = bricks.iter().map(|brick| brick.end.x).max().unwrap_or(0);
    let z_min = bricks.iter().map(|brick| brick.start.z).min().unwrap_or(0);
    let z_max = bricks.iter().map(|brick| brick.end.z).max().unwrap_or(0);

    for z in (z_min..=z_max).rev() {
        for x in x_min..=x_max {
            if bricks.iter().any(|brick| {
                brick.start.x <= x && x <= brick.end.x && brick.start.z <= z && z <= brick.end.z
            }) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<Brick>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| line.into()).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut bricks = input.clone();
        bricks.sort_by_key(|brick| brick.start.z);
        drop_bricks(&mut bricks);

        get_num_disintegrable_bricks(&bricks)
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
            Brick {
                start: Coord { x: 1, y: 0, z: 1 },
                end: Coord { x: 1, y: 2, z: 1 },
            },
            Brick {
                start: Coord { x: 0, y: 0, z: 2 },
                end: Coord { x: 2, y: 0, z: 2 },
            },
            Brick {
                start: Coord { x: 0, y: 2, z: 3 },
                end: Coord { x: 2, y: 2, z: 3 },
            },
            Brick {
                start: Coord { x: 0, y: 0, z: 4 },
                end: Coord { x: 0, y: 2, z: 4 },
            },
            Brick {
                start: Coord { x: 2, y: 0, z: 5 },
                end: Coord { x: 2, y: 2, z: 5 },
            },
            Brick {
                start: Coord { x: 0, y: 1, z: 6 },
                end: Coord { x: 2, y: 1, z: 6 },
            },
            Brick {
                start: Coord { x: 1, y: 1, z: 8 },
                end: Coord { x: 1, y: 1, z: 9 },
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 5);
    }

    #[test]
    #[ignore = "not yet implemented"]
    #[allow(unreachable_code)]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
