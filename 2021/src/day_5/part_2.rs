use crate::day_5::detector;
use crate::day_5::line::Line;
use crate::day_5::point::Point;

struct Detector;

impl detector::Detector for Detector {
    fn line_is_on_point(&self, line: &Line, point: Point) -> bool {
        line.is_on(point)
    }
}

pub fn count_dangerous_tiles(lines: &[Line]) -> u32 {
    let detector = Detector;

    crate::day_5::grid::count_dangerous_tiles(lines, &detector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_overlap_including_diagonals() {
        let lines = [
            Line::new(Point::new(0, 9), Point::new(5, 9)),
            Line::new(Point::new(8, 0), Point::new(0, 8)),
            Line::new(Point::new(9, 4), Point::new(3, 4)),
            Line::new(Point::new(2, 2), Point::new(2, 1)),
            Line::new(Point::new(7, 0), Point::new(7, 4)),
            Line::new(Point::new(6, 4), Point::new(2, 0)),
            Line::new(Point::new(0, 9), Point::new(2, 9)),
            Line::new(Point::new(3, 4), Point::new(1, 4)),
            Line::new(Point::new(0, 0), Point::new(8, 8)),
            Line::new(Point::new(5, 5), Point::new(8, 2)),
        ];

        assert_eq!(count_dangerous_tiles(&lines), 12);
    }
}
