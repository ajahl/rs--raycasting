use cgmath::{Point2, Vector2};

use crate::segment::Segment;

pub struct Map {}

impl Map {
    pub fn make_map(map: &str) -> Vec<Segment> {
        let mut result: Vec<Segment> = Vec::new();
        let lines: Vec<&str> = map.split("\n").collect();

        let mut y = lines.len() as i32;

        for line in lines {
            let mut x = 0;
            for char in line.chars() {
                match char {
                    '#' | '*' => {
                        result.extend(Map::box_shape(Point2::new(x, y)));
                    }
                    '/' => {
                        result.extend(Map::ul_triangle(Point2::new(x, y)));
                    }
                    '&' => {
                        result.extend(Map::ur_triangle(Point2::new(x, y)));
                    }
                    '%' => {
                        result.extend(Map::lr_triangle(Point2::new(x, y)));
                    }
                    '`' => {
                        result.extend(Map::ll_triangle(Point2::new(x, y)));
                    }
                    _ => {}
                }
                x += 1
            }
            y -= 1
        }

        result
    }

    pub fn box_shape(point: Point2<i32>) -> Vec<Segment> {
        let point_f32 = Point2::new(point.x as f32, point.y as f32);
        vec![
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(1.0, 0.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(1.0, 0.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(0.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(0.0, -1.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
        ]
    }

    pub fn lr_triangle(point: Point2<i32>) -> Vec<Segment> {
        let point_f32 = Point2::new(point.x as f32, point.y as f32);
        vec![
            Segment::new(
                point_f32 + Vector2::new(0.0, -1.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(1.0, 0.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(0.0, -1.0),
                point_f32 + Vector2::new(1.0, 0.0),
            ),
        ]
    }

    pub fn ur_triangle(point: Point2<i32>) -> Vec<Segment> {
        let point_f32 = Point2::new(point.x as f32, point.y as f32);
        vec![
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(1.0, 0.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(1.0, 0.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
        ]
    }

    pub fn ll_triangle(point: Point2<i32>) -> Vec<Segment> {
        let point_f32 = Point2::new(point.x as f32, point.y as f32);
        vec![
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(0.0, -1.0),
                point_f32 + Vector2::new(1.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(0.0, -1.0),
            ),
        ]
    }

    pub fn ul_triangle(point: Point2<i32>) -> Vec<Segment> {
        let point_f32 = Point2::new(point.x as f32, point.y as f32);
        vec![
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(1.0, 0.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(1.0, 0.0),
                point_f32 + Vector2::new(0.0, -1.0),
            ),
            Segment::new(
                point_f32 + Vector2::new(0.0, 0.0),
                point_f32 + Vector2::new(0.0, -1.0),
            ),
        ]
    }
}
