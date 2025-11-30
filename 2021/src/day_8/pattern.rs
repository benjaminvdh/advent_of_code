use std::ops::{BitAnd, BitOr, Not};

pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl BitOr for Segment {
    type Output = Pattern;

    fn bitor(self, rhs: Self) -> Self::Output {
        Pattern::from(self) | rhs
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pattern {
    bits: u8,
}

impl Pattern {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn num_segments(&self) -> u32 {
        self.bits.count_ones()
    }

    pub fn is_one(&self) -> bool {
        self.bits.count_ones() == 2
    }

    pub fn is_four(&self) -> bool {
        self.bits.count_ones() == 4
    }

    pub fn is_seven(&self) -> bool {
        self.bits.count_ones() == 3
    }

    pub fn is_eight(&self) -> bool {
        self.bits.count_ones() == 7
    }
}

impl From<Segment> for Pattern {
    fn from(segment: Segment) -> Self {
        let bits = match segment {
            Segment::A => 1 << 0,
            Segment::B => 1 << 1,
            Segment::C => 1 << 2,
            Segment::D => 1 << 3,
            Segment::E => 1 << 4,
            Segment::F => 1 << 5,
            Segment::G => 1 << 6,
        };

        Self { bits }
    }
}

impl BitOr for Pattern {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitOr<Segment> for Pattern {
    type Output = Self;

    fn bitor(self, rhs: Segment) -> Self::Output {
        self | Pattern::from(rhs)
    }
}

impl BitAnd for Pattern {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl Not for Pattern {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { bits: !self.bits }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctor() {
        let a = Pattern::from(Segment::A);
        assert_eq!(a.bits, 1);

        let d = Pattern::from(Segment::D);
        assert_eq!(d.bits, 8);

        let g = Pattern::from(Segment::G);
        assert_eq!(g.bits, 64);
    }

    #[test]
    fn or() {
        let c = Pattern::from(Segment::C);
        let e = Pattern::from(Segment::E);

        assert_eq!(c | e, Pattern { bits: 20 });
    }

    #[test]
    fn segment_or() {
        let pattern = Segment::F | Segment::B;
        assert_eq!(pattern.bits, 34);
    }
}
