use crate::day_5::detector::Detector;
use crate::day_5::line::Line;
use crate::day_5::point::Point;

pub fn count_dangerous_tiles<D: Detector>(lines: &[Line], detector: &D) -> u32 {
    let x = get_max_x_coord(lines);
    let y = get_max_y_coord(lines);

    (0..=x)
        .map(|x| (0..=y).filter(|&y| danger(lines, x, y, detector)).count() as u32)
        .sum()
}

fn danger<D: Detector>(lines: &[Line], x: u32, y: u32, detector: &D) -> bool {
    lines
        .iter()
        .filter(|line| detector.line_is_on_point(line, Point::new(x, y)))
        .count()
        > 1
}

fn get_max_x_coord(lines: &[Line]) -> u32 {
    lines.iter().map(|line| line.get_max_x()).max().unwrap_or(0)
}

fn get_max_y_coord(lines: &[Line]) -> u32 {
    lines.iter().map(|line| line.get_max_y()).max().unwrap_or(0)
}
