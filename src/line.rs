use cgmath::Point2;
use std::mem;

pub struct Line {
    pub origin: Point2<f32>,
    pub slope: f32,
}

impl Line {
    pub fn intercept(start_line: &Line, segment_line: &Line) -> Point2<f32> {
        let mut l1 = start_line;
        let mut l2 = segment_line;

        if l1.slope.abs() > l2.slope.abs() {
            mem::swap(&mut l1, &mut l2);
        }

        let x = -(-l2.origin.y + l1.origin.y + l2.slope * l2.origin.x - l1.slope * l1.origin.x)
            / (l1.slope - l2.slope);

        let y = l1.slope * (x - l1.origin.x) + l1.origin.y;

        Point2::new(x, y)
    }
}
