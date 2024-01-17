extern crate cgmath;

use crate::{line::Line, segment::Segment};
use cgmath::{MetricSpace, Point2, Vector2};

pub struct Ray {
    pub origin: Point2<f32>,
    pub direction: Vector2<f32>,
    pub angle: f32,
}

impl Ray {
    pub const DISTANT_POINT: f32 = 100.0;

    // pub fn angle(origin: Point2<f32>, direction: Point2<f32>) -> f32 {
    //     let direction_vector = direction. - origin;
    //     direction_vector.y.atan2(direction_vector.x)
    // }

    pub fn angle(direction: Vector2<f32>) -> f32 {
        direction.y.atan2(direction.x)
    }

    pub fn to_segment(&self) -> Segment {
        Segment::new(self.origin, self.distant_point())
    }

    pub fn distant_point(&self) -> Point2<f32> {
        let x = self.origin.x + (self.angle.sin() * Ray::DISTANT_POINT);
        let y = self.origin.y + (self.angle.cos() * Ray::DISTANT_POINT);
        Point2::new(x, y)
    }

    pub fn intersect(
        ray: Ray,
        map_wall_segments: &Vec<Segment>,
    ) -> Result<Vec<(f32, Point2<f32>, &Segment)>, &'static str> {
        let ray_segment = ray.to_segment();
        let start_line = ray_segment.line()?;

        let mut result: Vec<(f32, Point2<f32>, &Segment)> = Vec::new();

        for segment in map_wall_segments {
            let ray_segment = ray.to_segment();
            if !(segment.min_x() <= ray_segment.max_x()
                && segment.max_x() >= ray_segment.min_x()
                && segment.min_y() <= ray_segment.max_y()
                && segment.max_y() >= ray_segment.min_y())
            {
                continue;
            }

            let segment_line = segment.line();
            match segment_line {
                Ok(line) => {
                    if line.slope != line.slope {
                        let point = Line::intercept(&start_line, &line);

                        if segment.in_bounds(point) && ray_segment.in_bounds(point) {
                            result.push((
                                Point2::distance(ray_segment.start, point),
                                point,
                                segment,
                            ));
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            }
        }
        Ok(result)
    }
}
