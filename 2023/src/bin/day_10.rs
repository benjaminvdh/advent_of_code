#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    V,
    H,
    L,
    J,
    T,
    F,
    G,
    S,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '|' => Dir::V,
            '-' => Dir::H,
            'L' => Dir::L,
            'J' => Dir::J,
            '7' => Dir::T,
            'F' => Dir::F,
            '.' => Dir::G,
            'S' => Dir::S,
            _ => panic!(),
        }
    }
}

impl Dir {
    #[allow(unused)]
    fn get_char(&self) -> char {
        match *self {
            Dir::V => '│',
            Dir::H => '─',
            Dir::L => '└',
            Dir::J => '┘',
            Dir::T => '┐',
            Dir::F => '┌',
            Dir::G => '.',
            Dir::S => 'S',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    x: u64,
    y: u64,
}

impl From<(usize, usize)> for Pos {
    fn from(coords: (usize, usize)) -> Self {
        Self {
            x: coords.0 as u64,
            y: coords.1 as u64,
        }
    }
}

impl From<(u64, u64)> for Pos {
    fn from(coords: (u64, u64)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid(Vec<Vec<Dir>>);

impl Grid {
    pub fn get_start(&self) -> Pos {
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if matches!(self.0[y][x], Dir::S) {
                    return (x, y).into();
                }
            }
        }

        panic!()
    }

    pub fn get(&self, pos: Pos) -> Dir {
        self.0[pos.y as usize][pos.x as usize]
    }

    pub fn get_next(&self, curr: Pos, prev: Pos) -> Pos {
        match self.get(curr) {
            Dir::V => (
                curr.x,
                if prev.y == curr.y - 1 {
                    curr.y + 1
                } else {
                    curr.y - 1
                },
            ),
            Dir::H => (
                if prev.x == curr.x - 1 {
                    curr.x + 1
                } else {
                    curr.x - 1
                },
                curr.y,
            ),
            Dir::L => {
                if prev.x == curr.x {
                    (curr.x + 1, curr.y)
                } else {
                    (curr.x, curr.y - 1)
                }
            }
            Dir::J => {
                if prev.x == curr.x {
                    (curr.x - 1, curr.y)
                } else {
                    (curr.x, curr.y - 1)
                }
            }
            Dir::T => {
                if prev.x == curr.x {
                    (curr.x - 1, curr.y)
                } else {
                    (curr.x, curr.y + 1)
                }
            }
            Dir::F => {
                if prev.x == curr.x {
                    (curr.x + 1, curr.y)
                } else {
                    (curr.x, curr.y + 1)
                }
            }
            _ => panic!(),
        }
        .into()
    }

    pub fn are_connected(&self, start_pos: Pos, cell_pos: Pos) -> bool {
        match self.get(cell_pos) {
            Dir::V => cell_pos.x == start_pos.x,
            Dir::H => cell_pos.y == start_pos.y,
            Dir::L => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y + 1)
                    || (cell_pos.x == start_pos.x - 1 && cell_pos.y == start_pos.y)
            }
            Dir::J => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y + 1)
                    || (cell_pos.x == start_pos.x + 1 && cell_pos.y == start_pos.y)
            }
            Dir::T => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y - 1)
                    || (cell_pos.x == start_pos.x + 1 && cell_pos.y == start_pos.y)
            }
            Dir::F => {
                (cell_pos.x == start_pos.x && cell_pos.y == start_pos.y - 1)
                    || (cell_pos.x == start_pos.x - 1 && cell_pos.y == start_pos.y)
            }
            Dir::G => false,
            Dir::S => false,
        }
    }

    #[allow(unused)]
    fn print(&self) {
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                let pos = Pos::from((x, y));
                print!("{}", self.get(pos).get_char());
            }
            println!();
        }
    }
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Grid;
    type Output1 = u32;
    type Output2 = u32;

    fn parse(input: &str) -> Self::Input {
        Grid(input.lines().map(|line| parse_line(line)).collect())
    }

    fn part_1(grid: &Self::Input) -> Self::Output1 {
        let start = grid.get_start();

        let next_steps = [
            Some((start.x + 1, start.y).into()),
            Some((start.x, start.y + 1).into()),
            start.x.checked_sub(1).map(|x| (x, start.y).into()),
            start.y.checked_sub(1).map(|y| (start.x, y).into()),
        ]
        .into_iter()
        .filter_map(|p| p);

        for step in next_steps {
            if !grid.are_connected(start, step) {
                continue;
            }

            let mut prev = start;
            let mut curr = step;
            let mut num_steps = 0;

            loop {
                let next = grid.get_next(curr, prev);
                num_steps += 1;

                if matches!(grid.get(next), Dir::S) {
                    return num_steps / 2 + 1;
                }

                prev = curr;
                curr = next;
            }
        }

        panic!()
    }

    fn part_2(grid: &Self::Input) -> Self::Output2 {
        let simplified = get_simplified_grid(grid);
        let inflated = inflate(&simplified);

        let mut map = get_reachable_map(&inflated);
        loop {
            if !fill_in_reachable_map(&inflated, &mut map) {
                break;
            }
        }

        let mut num_enclosed = 0;

        for y in 0..grid.0.len() {
            for x in 0..grid.0[y].len() {
                if matches!(simplified.get((x, y).into()), Dir::G) && !map[y * 2][x * 2] {
                    num_enclosed += 1;
                }
            }
        }

        num_enclosed
    }
}

#[allow(unused)]
fn print_map(map: &Vec<Vec<bool>>) {
    for row in map {
        for val in row {
            print!("{}", if *val { "1" } else { "0" });
        }
        println!();
    }
}

fn get_simplified_grid(grid: &Grid) -> Grid {
    let start = grid.get_start();

    let next_steps = [
        Some((start.x + 1, start.y).into()),
        Some((start.x, start.y + 1).into()),
        start.x.checked_sub(1).map(|x| (x, start.y).into()),
        start.y.checked_sub(1).map(|y| (start.x, y).into()),
    ]
    .into_iter()
    .filter_map(|p| p);

    for step in next_steps {
        if !grid.are_connected(start, step) {
            continue;
        }

        let mut loop_tiles = vec![start, step];

        let mut prev = start;
        let mut curr = step;

        loop {
            let next = grid.get_next(curr, prev);
            loop_tiles.push(next);

            if matches!(grid.get(next), Dir::S) {
                let mut simplified = Grid(vec![]);

                for y in 0..grid.0.len() {
                    simplified.0.push(vec![]);

                    for x in 0..grid.0[y].len() {
                        if loop_tiles.contains(&(x, y).into()) {
                            simplified.0[y].push(grid.get((x, y).into()));
                        } else {
                            simplified.0[y].push(Dir::G);
                        }
                    }
                }

                return simplified;
            }

            prev = curr;
            curr = next;
        }
    }

    panic!()
}

fn inflate(grid: &Grid) -> Grid {
    let mut inflated = Grid(vec![]);

    for y in 0..grid.0.len() {
        let mut new_row_1 = vec![];

        for x in 0..grid.0[y].len() {
            match grid.get((x, y).into()) {
                d @ Dir::H | d @ Dir::L | d @ Dir::F => {
                    new_row_1.push(d);
                    new_row_1.push(Dir::H);
                }
                Dir::S => {
                    new_row_1.push(Dir::S);
                    new_row_1.push(Dir::S);
                }
                d => {
                    new_row_1.push(d);
                    new_row_1.push(Dir::G);
                }
            }
        }

        let mut new_row_2 = vec![];
        for d in new_row_1.iter() {
            match d {
                Dir::F | Dir::T | Dir::V => new_row_2.push(Dir::V),
                Dir::S => new_row_2.push(Dir::S),
                _ => new_row_2.push(Dir::G),
            }
        }

        inflated.0.push(new_row_1);
        inflated.0.push(new_row_2);
    }

    inflated
}

fn get_reachable_map(grid: &Grid) -> Vec<Vec<bool>> {
    let mut map = vec![];

    for y in 0..grid.0.len() {
        let mut row = vec![];

        for x in 0..grid.0[y].len() {
            let val = if x == 0 || y == 0 || x == grid.0[y].len() - 1 || y == grid.0.len() - 1 {
                match grid.get((x, y).into()) {
                    Dir::G => true,
                    _ => false,
                }
            } else {
                false
            };

            row.push(val);
        }

        map.push(row);
    }

    map
}

fn fill_in_reachable_map(grid: &Grid, map: &mut Vec<Vec<bool>>) -> bool {
    let mut has_changed = false;

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if !map[y][x] {
                if matches!(grid.get((x, y).into()), Dir::G) {
                    if map[y][x - 1] || map[y][x + 1] || map[y - 1][x] || map[y + 1][x] {
                        map[y][x] = true;
                        has_changed = true;
                    }
                }
            }
        }
    }

    has_changed
}

fn parse_line(line: &str) -> Vec<Dir> {
    line.chars().map(|c| Dir::from(c)).collect()
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        Grid(vec![
            vec![Dir::G, Dir::G, Dir::F, Dir::T, Dir::G],
            vec![Dir::G, Dir::F, Dir::J, Dir::V, Dir::G],
            vec![Dir::S, Dir::J, Dir::G, Dir::L, Dir::T],
            vec![Dir::V, Dir::F, Dir::H, Dir::H, Dir::J],
            vec![Dir::L, Dir::J, Dir::G, Dir::G, Dir::G],
        ])
    }

    #[test]
    fn parsing() {
        let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 8);
    }

    fn get_input_2() -> <Solver as aoc::Solver>::Input {
        <Solver as aoc::Solver>::parse(
            r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(<Solver as aoc::Solver>::part_2(&get_input_2()), 10);
    }
}
