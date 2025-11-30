use std::collections::HashSet;

use aoc::grid::Pos;
use aoc::Dir;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Path,
    Forest,
    Slope(Dir),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(Dir::N),
            '>' => Self::Slope(Dir::E),
            'v' => Self::Slope(Dir::S),
            '<' => Self::Slope(Dir::W),
            _ => panic!(),
        }
    }
}

fn get_num_paths(grid: &Grid, cur: Pos, end: Pos, mut visited: HashSet<Pos>) -> usize {
    if cur == end {
        visited.len() - 1 // Do not count Start
    } else {
        match *grid.get(cur.x, cur.y) {
            Cell::Path => {
                let mut neighbors = grid.get_neighbors(&cur);
                neighbors.retain(|pos| {
                    !visited.contains(pos) && *grid.get(pos.x, pos.y) != Cell::Forest
                });

                if neighbors.is_empty() {
                    0
                } else if neighbors.len() == 1 {
                    visited.insert(neighbors[0]);
                    get_num_paths(grid, neighbors[0], end, visited)
                } else {
                    let max: usize = neighbors
                        .iter()
                        .skip(1)
                        .map(|&next| {
                            let mut visited = visited.clone();
                            visited.insert(next);
                            get_num_paths(grid, next, end, visited)
                        })
                        .max()
                        .unwrap();
                    visited.insert(neighbors[0]);
                    max.max(get_num_paths(grid, neighbors[0], end, visited))
                }
            }
            Cell::Forest => 0,
            Cell::Slope(dir) => {
                let next = cur.apply(dir);
                if !visited.insert(next) {
                    0
                } else {
                    get_num_paths(grid, next, end, visited)
                }
            }
        }
    }
}

type Grid = aoc::Grid<Cell>;

struct Solver;

impl aoc::Solver for Solver {
    type Input = Grid;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        Grid::from_iter(input.lines().map(|line| line.chars().map(|c| c.into())))
    }

    fn part_1(grid: &Self::Input) -> Self::Output1 {
        let start = Pos { x: 1, y: 0 };
        let next = Pos { x: 1, y: 1 };
        let end = Pos {
            x: grid.width() - 2,
            y: grid.height() - 1,
        };
        let visited = HashSet::from_iter([start, next].into_iter());

        get_num_paths(grid, next, end, visited)
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
        Grid::from_iter(
            [
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Slope(Dir::E),
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Slope(Dir::S),
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Path,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
                [
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Forest,
                    Cell::Path,
                    Cell::Forest,
                ]
                .into_iter(),
            ]
            .into_iter(),
        )
    }

    #[test]
    fn parsing() {
        let input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 94);
    }

    #[test]
    #[ignore = "not yet implemented"]
    #[allow(unreachable_code)]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input()), todo!());
    }
}
