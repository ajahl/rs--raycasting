extern crate cgmath;

use crate::line::Line;
use cgmath::Point2;

pub struct Segment {
    pub start: Point2<f32>,
    pub end: Point2<f32>,
}

impl Segment {
    pub fn new(start: Point2<f32>, end: Point2<f32>) -> Segment {
        Segment { start, end }
    }

    pub fn slope(&self) -> f32 {
        if (self.end.x - self.start.x).abs() < f32::EPSILON {
            f32::INFINITY
        } else {
            (self.end.y - self.start.y) / (self.end.x - self.start.x)
        }
    }

    pub fn line(&self) -> Result<Line, &'static str> {
        if self.start == self.end {
            return Err("Identical segment points");
        }

        Ok(Line {
            origin: self.start,
            slope: self.slope(),
        })
    }

    pub fn min_x(&self) -> f32 {
        self.start.x.min(self.end.x)
    }

    pub fn max_x(&self) -> f32 {
        self.start.x.max(self.end.x)
    }

    pub fn min_y(&self) -> f32 {
        self.start.y.min(self.end.y)
    }

    pub fn max_y(&self) -> f32 {
        self.start.y.max(self.end.y)
    }

    pub fn in_bounds(&self, point: Point2<f32>) -> bool {
        in_range(self.min_x(), self.max_x(), point.x)
            && in_range(self.min_y(), self.max_y(), point.y)
    }
}

fn in_range(min_x: f32, max_x: f32, value: f32) -> bool {
    const EPSILON: f32 = 1e-7;
    (min_x - EPSILON) <= value && value <= (max_x + EPSILON)
}
