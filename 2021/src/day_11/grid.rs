pub const GRID_SIZE: usize = 10;

pub fn update(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE]) -> u32 {
    let mut flashes = 0;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            increment(grid, x, y);
        }
    }

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if grid[y][x] > 9 {
                flashes += 1;
                grid[y][x] = 0;
            }
        }
    }

    flashes
}

fn increment(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], x: usize, y: usize) {
    grid[y][x] += 1;

    if grid[y][x] == 10 {
        for (xn, yn) in Iter::new(x, y) {
            increment(grid, xn, yn);
        }
    }
}

struct Iter {
    x: usize,
    y: usize,
    dir: Option<Dir>,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

impl Iterator for Dir {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Dir::NW => {
                *self = Dir::N;
                Some(*self)
            }
            Dir::N => {
                *self = Dir::NE;
                Some(*self)
            }
            Dir::NE => {
                *self = Dir::W;
                Some(*self)
            }
            Dir::W => {
                *self = Dir::E;
                Some(*self)
            }
            Dir::E => {
                *self = Dir::SW;
                Some(*self)
            }
            Dir::SW => {
                *self = Dir::S;
                Some(*self)
            }
            Dir::S => {
                *self = Dir::SE;
                Some(*self)
            }
            Dir::SE => None,
        }
    }
}

impl Iter {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
            dir: Some(Dir::NW),
        }
    }
}

impl Iterator for Iter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut dir) = self.dir {
            let old_dir = dir;
            self.dir = dir.next();

            get_coords_for_dir(self.x, self.y, old_dir).or_else(|| self.next())
        } else {
            None
        }
    }
}

fn get_coords_for_dir(x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
    Some(match dir {
        Dir::NW => (x.checked_sub(1)?, y.checked_sub(1)?),
        Dir::N => (x, y.checked_sub(1)?),
        Dir::NE => (checked_grid_add(x)?, y.checked_sub(1)?),
        Dir::E => (checked_grid_add(x)?, y),
        Dir::SE => (checked_grid_add(x)?, checked_grid_add(y)?),
        Dir::S => (x, checked_grid_add(y)?),
        Dir::SW => (x.checked_sub(1)?, checked_grid_add(y)?),
        Dir::W => (x.checked_sub(1)?, y),
    })
}

fn checked_grid_add(a: usize) -> Option<usize> {
    if a + 1 < GRID_SIZE {
        Some(a + 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_iter() {
        let mut dir = Dir::E;

        assert!(matches!(dir.next().unwrap(), Dir::SW));
        assert!(matches!(dir.next().unwrap(), Dir::S));
        assert!(matches!(dir.next().unwrap(), Dir::SE));
        assert!(dir.next().is_none());
    }

    #[test]
    fn grid_iter() {
        let mut iter = Iter::new(0, 0);
        assert_eq!(iter.next().unwrap(), (1, 0));
        assert_eq!(iter.next().unwrap(), (0, 1));
        assert_eq!(iter.next().unwrap(), (1, 1));
        assert!(iter.next().is_none());
    }

    #[test]
    fn update_grid() {
        let mut grid = [
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        let reference: [[u8; 10]; 10] = [
            [0, 3, 9, 7, 6, 6, 6, 8, 6, 6],
            [0, 7, 4, 9, 7, 6, 6, 9, 1, 8],
            [0, 0, 5, 3, 9, 7, 6, 9, 3, 3],
            [0, 0, 0, 4, 2, 9, 7, 8, 2, 2],
            [0, 0, 0, 4, 2, 2, 9, 8, 9, 2],
            [0, 0, 5, 3, 2, 2, 2, 8, 7, 7],
            [0, 5, 3, 2, 2, 2, 2, 9, 6, 6],
            [9, 3, 2, 2, 2, 2, 8, 9, 6, 6],
            [7, 9, 2, 2, 2, 8, 6, 8, 6, 6],
            [6, 7, 8, 9, 9, 9, 8, 7, 6, 6],
        ];

        for _ in 0..100 {
            update(&mut grid);
        }

        assert_eq!(grid, reference);
    }
}
