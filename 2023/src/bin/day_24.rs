use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: f32,
    y: f32,
    z: f32,
}

impl From<&str> for Coord {
    fn from(string: &str) -> Self {
        let mut splits = string.split(',');

        Self {
            x: f32::from_str(splits.next().unwrap().trim()).unwrap(),
            y: f32::from_str(splits.next().unwrap().trim()).unwrap(),
            z: f32::from_str(splits.next().unwrap().trim()).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Hailstone {
    pos: Coord,
    vel: Coord,
}

impl Hailstone {
    fn intersects(&self, other: &Hailstone) -> Option<Coord> {
        let x1 = self.pos.x;
        let x2 = self.pos.y;
        let x3 = other.pos.x;
        let x4 = other.pos.y;

        let v1 = self.vel.x;
        let v2 = self.vel.y;
        let v3 = other.vel.x;
        let v4 = other.vel.y;

        if v1 == 0.0 || (v4 - v2 * v3 / v1) == 0.0 {
            None
        } else {
            let mu = (x2 - x4 + v2 * (x3 - x1) / v1) / (v4 - v2 * v3 / v1);
            let lambda = (x3 - x1 + mu * v3) / v1;

            if mu < 0.0 || lambda < 0.0 {
                None
            } else {
                Some(Coord {
                    x: x1 + lambda * v1,
                    y: x2 + lambda * v2,
                    z: 0.0,
                })
            }
        }
    }

    fn intersects_in_area(&self, other: &Hailstone, min: f32, max: f32) -> bool {
        self.intersects(other)
            .map(|coord| min <= coord.x && coord.x <= max && min <= coord.y && coord.y <= max)
            .unwrap_or(false)
    }
}

impl From<&str> for Hailstone {
    fn from(string: &str) -> Self {
        let (pos, vel) = string.split_once('@').unwrap();

        Self {
            pos: pos.into(),
            vel: vel.into(),
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Vec<Hailstone>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| line.into()).collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .enumerate()
            .map(|(i, a)| {
                input
                    .iter()
                    .skip(i + 1)
                    .filter(|b| a.intersects_in_area(b, 200000000000000.0, 400000000000000.0))
                    .count()
            })
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
            Hailstone {
                pos: Coord {
                    x: 19.0,
                    y: 13.0,
                    z: 30.0,
                },
                vel: Coord {
                    x: -2.0,
                    y: 1.0,
                    z: -2.0,
                },
            },
            Hailstone {
                pos: Coord {
                    x: 18.0,
                    y: 19.0,
                    z: 22.0,
                },
                vel: Coord {
                    x: -1.0,
                    y: -1.0,
                    z: -2.0,
                },
            },
            Hailstone {
                pos: Coord {
                    x: 20.0,
                    y: 25.0,
                    z: 34.0,
                },
                vel: Coord {
                    x: -2.0,
                    y: -2.0,
                    z: -4.0,
                },
            },
            Hailstone {
                pos: Coord {
                    x: 12.0,
                    y: 31.0,
                    z: 28.0,
                },
                vel: Coord {
                    x: -1.0,
                    y: -2.0,
                    z: -1.0,
                },
            },
            Hailstone {
                pos: Coord {
                    x: 20.0,
                    y: 19.0,
                    z: 15.0,
                },
                vel: Coord {
                    x: 1.0,
                    y: -5.0,
                    z: -3.0,
                },
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        let a = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let b = Hailstone::from("18, 19, 22 @ -1, -1, -2");
        assert!(a.intersects(&b).is_some());

        let a = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let b = Hailstone::from("20, 25, 34 @ -2, -2, -4");
        assert!(a.intersects(&b).is_some());

        let a = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let b = Hailstone::from("12, 31, 28 @ -1, -2, -1");
        assert!(a.intersects(&b).is_some());

        let a = Hailstone::from("19, 13, 30 @ -2, 1, -2");
        let b = Hailstone::from("20, 19, 15 @ 1, -5, -3");
        assert!(a.intersects(&b).is_none());

        let a = Hailstone::from("18, 19, 22 @ -1, -1, -2");
        let b = Hailstone::from("20, 25, 34 @ -2, -2, -4");
        assert!(a.intersects(&b).is_none());

        let a = Hailstone::from("18, 19, 22 @ -1, -1, -2");
        let b = Hailstone::from("12, 31, 28 @ -1, -2, -1");
        assert!(a.intersects(&b).is_some());

        let a = Hailstone::from("18, 19, 22 @ -1, -1, -2");
        let b = Hailstone::from("20, 19, 15 @ 1, -5, -3");
        assert!(a.intersects(&b).is_none());

        let a = Hailstone::from("20, 25, 34 @ -2, -2, -4");
        let b = Hailstone::from("12, 31, 28 @ -1, -2, -1");
        assert!(a.intersects(&b).is_some());

        let a = Hailstone::from("20, 25, 34 @ -2, -2, -4");
        let b = Hailstone::from("20, 19, 15 @ 1, -5, -3");
        assert!(a.intersects(&b).is_none());

        let a = Hailstone::from("12, 31, 28 @ -1, -2, -1");
        let b = Hailstone::from("20, 19, 15 @ 1, -5, -3");
        assert!(a.intersects(&b).is_none());
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn part_2() {
        todo!()
    }
}
