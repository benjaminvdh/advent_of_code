#[derive(Clone, Copy, Debug, PartialEq)]
struct Galaxy {
    x: u64,
    y: u64,
}

impl Galaxy {
    pub fn distance_to(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct StarMap(Vec<Galaxy>);

impl StarMap {
    pub fn get_sum_of_distances(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, a)| {
                self.0
                    .iter()
                    .skip(i + 1)
                    .map(|b| a.distance_to(b))
                    .sum::<u64>()
            })
            .sum()
    }

    pub fn enlarged(&self, scale_factor: u64) -> Self {
        let max_x = self.0.iter().map(|gal| gal.x).max().unwrap();
        let max_y = self.0.iter().map(|gal| gal.y).max().unwrap();

        let mut scaled_col_index = 0;
        let mut scaled_cols = Vec::with_capacity(max_x as usize + 1);

        for x in 0..=max_x {
            scaled_cols.push(scaled_col_index);
            scaled_col_index += if self.0.iter().any(|gal| gal.x == x) {
                1
            } else {
                scale_factor
            };
        }

        let mut scaled_row_index = 0;
        let mut scaled_rows = Vec::with_capacity(max_y as usize + 1);

        for y in 0..=max_y {
            scaled_rows.push(scaled_row_index);
            scaled_row_index += if self.0.iter().any(|gal| gal.y == y) {
                1
            } else {
                scale_factor
            };
        }

        Self(
            self.0
                .iter()
                .map(|gal| Galaxy {
                    x: scaled_cols[gal.x as usize],
                    y: scaled_rows[gal.y as usize],
                })
                .collect(),
        )
    }
}

impl From<&str> for StarMap {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.char_indices().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some(Galaxy {
                                x: x as u64,
                                y: y as u64,
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        )
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = StarMap;
    type Output1 = u64;
    type Output2 = u64;

    fn parse(input: &str) -> Self::Input {
        input.into()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.enlarged(2).get_sum_of_distances()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.enlarged(1_000_000).get_sum_of_distances()
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        StarMap(vec![
            Galaxy { x: 3, y: 0 },
            Galaxy { x: 7, y: 1 },
            Galaxy { x: 0, y: 2 },
            Galaxy { x: 6, y: 4 },
            Galaxy { x: 1, y: 5 },
            Galaxy { x: 9, y: 6 },
            Galaxy { x: 7, y: 8 },
            Galaxy { x: 0, y: 9 },
            Galaxy { x: 4, y: 9 },
        ])
    }

    #[test]
    fn parsing() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 374);
    }

    #[test]
    fn part_2() {
        assert_eq!(get_input().enlarged(10).get_sum_of_distances(), 1030);
        assert_eq!(get_input().enlarged(100).get_sum_of_distances(), 8410);
    }
}
