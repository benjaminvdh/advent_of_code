#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn right(self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    pub fn left(self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }
}
