use std::collections::HashSet;

use aoc::grid::Pos;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Start,
    Garden,
    Rock,
}

impl Tile {
    pub fn new_start() -> Self {
        Self::Start
    }
    pub fn new_garden() -> Self {
        Self::Garden
    }
    pub fn new_rock() -> Self {
        Self::Rock
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Self::new_start(),
            '.' => Self::new_garden(),
            '#' => Self::new_rock(),
            _ => panic!(),
        }
    }
}

type Grid = aoc::Grid<Tile>;

struct Solver;

impl aoc::Solver for Solver {
    type Input = Grid;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        Grid::from_iter(input.lines().map(|line| line.chars().map(|c| c.into())))
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        get_num_reachable_tiles(input, 64)
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn get_num_reachable_tiles(grid: &Grid, num_steps: usize) -> usize {
    let mut reachable = HashSet::new();
    reachable.insert(find_start(grid));

    for _ in 0..num_steps {
        let mut new_reachable = HashSet::new();

        for pos in reachable {
            new_reachable.extend(get_reachable_from(grid, &pos));
        }

        reachable = new_reachable;
    }

    reachable.len()
}

fn find_start(grid: &Grid) -> Pos {
    (0..grid.height())
        .filter_map(|y| {
            (0..grid.width())
                .find(|x| *grid.get(*x, y) == Tile::Start)
                .map(|x| Pos { x, y })
        })
        .next()
        .unwrap()
}

fn get_reachable_from(grid: &Grid, pos: &Pos) -> Vec<Pos> {
    grid.get_neighbors(pos)
        .into_iter()
        .filter(|Pos { x, y }| *grid.get(*x, *y) != Tile::Rock)
        .collect()
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        Grid::from_iter(
            [
                [
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_start(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                    Tile::new_rock(),
                    Tile::new_rock(),
                    Tile::new_garden(),
                ]
                .into_iter(),
                [
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                    Tile::new_garden(),
                ]
                .into_iter(),
            ]
            .into_iter(),
        )
    }

    #[test]
    fn parsing() {
        let input = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(get_num_reachable_tiles(&get_input(), 6), 16);
    }

    #[test]
    #[ignore = "not yet implemented"]
    #[allow(unreachable_code)]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
