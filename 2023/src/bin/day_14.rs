use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rock {
    Empty,
    Rounded,
    Cubic,
}

impl Display for Rock {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let c = match *self {
            Rock::Empty => '.',
            Rock::Rounded => 'O',
            Rock::Cubic => '#',
        };
        write!(fmt, "{c}",)
    }
}

impl From<char> for Rock {
    fn from(c: char) -> Self {
        match c {
            '.' => Rock::Empty,
            'O' => Rock::Rounded,
            '#' => Rock::Cubic,
            _ => panic!(),
        }
    }
}

struct Solver;

type RockGrid = aoc::Grid<Rock>;

fn tilt_north(grid: &mut RockGrid) {
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if matches!(*grid.get(x, y), Rock::Rounded) {
                let mut curr_y = y;

                loop {
                    if curr_y == 0 {
                        break;
                    }

                    if !matches!(*grid.get(x, curr_y - 1), Rock::Empty) {
                        break;
                    }

                    *grid.get_mut(x, curr_y) = Rock::Empty;
                    *grid.get_mut(x, curr_y - 1) = Rock::Rounded;
                    curr_y -= 1;
                }
            }
        }
    }
}

fn tilt_east(grid: &mut RockGrid) {
    for y in 0..grid.height() {
        for x in (0..grid.width()).rev() {
            if matches!(*grid.get(x, y), Rock::Rounded) {
                let mut curr_x = x;

                loop {
                    if curr_x == grid.width() - 1 {
                        break;
                    }

                    if !matches!(*grid.get(curr_x + 1, y), Rock::Empty) {
                        break;
                    }

                    *grid.get_mut(curr_x, y) = Rock::Empty;
                    *grid.get_mut(curr_x + 1, y) = Rock::Rounded;
                    curr_x += 1;
                }
            }
        }
    }
}

fn tilt_south(grid: &mut RockGrid) {
    for x in 0..grid.width() {
        for y in (0..grid.height()).rev() {
            if matches!(*grid.get(x, y), Rock::Rounded) {
                let mut curr_y = y;

                loop {
                    if curr_y == grid.height() - 1 {
                        break;
                    }

                    if !matches!(*grid.get(x, curr_y + 1), Rock::Empty) {
                        break;
                    }

                    *grid.get_mut(x, curr_y) = Rock::Empty;
                    *grid.get_mut(x, curr_y + 1) = Rock::Rounded;
                    curr_y += 1;
                }
            }
        }
    }
}

fn tilt_west(grid: &mut RockGrid) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if matches!(*grid.get(x, y), Rock::Rounded) {
                let mut curr_x = x;

                loop {
                    if curr_x == 0 {
                        break;
                    }

                    if !matches!(*grid.get(curr_x - 1, y), Rock::Empty) {
                        break;
                    }

                    *grid.get_mut(curr_x, y) = Rock::Empty;
                    *grid.get_mut(curr_x - 1, y) = Rock::Rounded;
                    curr_x -= 1;
                }
            }
        }
    }
}

impl aoc::Solver for Solver {
    type Input = RockGrid;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        RockGrid::from_iter(input.lines().map(|line| line.chars().map(|c| c.into())))
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut grid = input.clone();
        tilt_north(&mut grid);
        get_load(&grid)
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut grid = input.clone();

        let mut grids = vec![];
        grids.push(grid.clone());

        let mut cycle_start = 0;
        let mut cycle_repeat = 0;

        for _ in 0..1_000_000_000 {
            tilt_north(&mut grid);
            tilt_west(&mut grid);
            tilt_south(&mut grid);
            tilt_east(&mut grid);

            if let Some(match_index) = grids.iter().position(|other| other == &grid) {
                cycle_start = match_index;
                cycle_repeat = grids.len();
                break;
            }

            grids.push(grid.clone());
        }

        let cycle_len = cycle_repeat - cycle_start;

        get_load(&grids[(1_000_000_000 - cycle_start) % cycle_len + cycle_start])
    }
}

fn get_load(grid: &RockGrid) -> usize {
    let mut sum = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if matches!(*grid.get(x, y), Rock::Rounded) {
                sum += grid.height() - y;
            }
        }
    }

    sum
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        RockGrid::from_iter(
            [
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Cubic,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Cubic,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
            ]
            .into_iter(),
        )
    }

    #[test]
    fn parsing() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 136);
    }

    fn get_tilt_input() -> RockGrid {
        RockGrid::from_iter(
            [
                [
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
            ]
            .into_iter(),
        )
    }

    #[test]
    fn tilt_north() {
        let mut grid = get_tilt_input();
        super::tilt_north(&mut grid);
        let ref_grid = RockGrid::from_iter(
            [
                [
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
            ]
            .into_iter(),
        );
        assert_eq!(grid, ref_grid);
    }

    #[test]
    fn tilt_east() {
        let mut grid = get_tilt_input();
        super::tilt_east(&mut grid);
        let ref_grid = RockGrid::from_iter(
            [
                [
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Cubic,
                ]
                .into_iter(),
            ]
            .into_iter(),
        );
        assert_eq!(grid, ref_grid);
    }

    #[test]
    fn tilt_south() {
        let mut grid = get_tilt_input();
        super::tilt_south(&mut grid);
        let ref_grid = RockGrid::from_iter(
            [
                [
                    Rock::Cubic,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Rounded,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Cubic,
                ]
                .into_iter(),
            ]
            .into_iter(),
        );
        assert_eq!(grid, ref_grid);
    }

    #[test]
    fn tilt_west() {
        let mut grid = get_tilt_input();
        super::tilt_west(&mut grid);
        let ref_grid = RockGrid::from_iter(
            [
                [
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                    Rock::Empty,
                ]
                .into_iter(),
                [
                    Rock::Cubic,
                    Rock::Rounded,
                    Rock::Rounded,
                    Rock::Empty,
                    Rock::Cubic,
                ]
                .into_iter(),
            ]
            .into_iter(),
        );
        assert_eq!(grid, ref_grid);
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), 64);
    }
}
