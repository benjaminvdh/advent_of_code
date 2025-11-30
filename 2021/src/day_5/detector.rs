use crate::day_5::line::Line;
use crate::day_5::point::Point;

pub trait Detector {
    fn line_is_on_point(&self, line: &Line, point: Point) -> bool;
}
