use std::ops::RangeInclusive;

use crate::day_5::point::Point;

#[derive(Debug, PartialEq)]
pub struct Line {
    start: Point,
    x_range: RangeInclusive<u32>,
    y_range: RangeInclusive<u32>,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let x_range = get_range(start.x, end.x);
        let y_range = get_range(start.y, end.y);

        Self {
            start,
            x_range,
            y_range,
        }
    }

    pub fn is_on(&self, point: Point) -> bool {
        self.x_range.contains(&point.x)
            && self.y_range.contains(&point.y)
            && (!self.is_diagonal()
                || crate::abs_diff(point.x, self.start.x) == crate::abs_diff(point.y, self.start.y))
    }

    pub fn get_max_x(&self) -> u32 {
        *self.x_range.end()
    }

    pub fn get_max_y(&self) -> u32 {
        *self.y_range.end()
    }

    pub fn is_diagonal(&self) -> bool {
        self.x_range.start() != self.x_range.end() && self.y_range.start() != self.y_range.end()
    }
}

fn get_range(start: u32, end: u32) -> RangeInclusive<u32> {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_line() {
        let line = Line::new(Point::new(8, 2), Point::new(3, 2));

        let ref_points = [
            Point::new(8, 2),
            Point::new(7, 2),
            Point::new(6, 2),
            Point::new(5, 2),
        ];

        assert!(ref_points.into_iter().all(|point| line.is_on(point)));
    }

    #[test]
    fn diagonal_line() {
        let line = Line::new(Point::new(12, 3), Point::new(8, 7));

        let ref_points = [
            Point::new(12, 3),
            Point::new(11, 4),
            Point::new(10, 5),
            Point::new(9, 6),
            Point::new(8, 7),
        ];

        assert!(ref_points.into_iter().all(|point| line.is_on(point)));

        let other_points = [
            Point::new(11, 6),
            Point::new(13, 2),
            Point::new(12, 7),
            Point::new(7, 8),
            Point::new(9, 7),
        ];

        assert!(other_points.into_iter().all(|point| !line.is_on(point)));
    }
}
