use sh::Point;
use sh::Line;

pub fn display_point(p: Point) {
    println!("The point for x and y is ({}, {})", p.x, p.y);
}

pub fn display_line(l: Line) {
    println!("Our line starts at ({}, {}) and ends at ({}, {}).", l.start.x, l.start.y, l.end.x, l.end.y);
}