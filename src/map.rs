use std::{collections::HashSet, rc::Rc};

use cgmath::{Point2, Vector2};

use crate::segment::Segment;

pub struct Map {}

impl Map {
    pub fn make_map(map: &str) -> Vec<&Segment> {
        let mut result: Vec<Segment> = Vec::new();
        let lines: Vec<&str> = map.split('\n').collect();

        let mut y = lines.len() as i32;

        for line in lines {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' | '*' => {
                        result.extend(Map::box_shape(Point2::new(x as i32, y)));
                    }
                    '/' => {
                        result.extend(Map::ul_triangle(Point2::new(x as i32, y)));
                    }
                    '&' => {
                        result.extend(Map::ur_triangle(Point2::new(x as i32, y)));
                    }
                    '%' => {
                        result.extend(Map::lr_triangle(Point2::new(x as i32, y)));
                    }
                    '`' => {
                        result.extend(Map::ll_triangle(Point2::new(x as i32, y)));
                    }
                    _ => {}
                }
            }
            y -= 1
        }

        println!("Segments: {}", result.len());

        let mut element_seen = HashSet::new();

        let mut result_with_singles: Vec<&Segment> = result
            .into_iter()
            .filter_map(|item| {
                if element_seen.insert(item.clone()) {
                    Some(item)
                } else {
                    None
                }
            })
            .collect();

        println!(
            "Filtered duplicated wall segments: {}",
            result_with_singles.len()
        );

        let mut result_with_singles_copy: Vec<&Segment> = result_with_singles.clone();//.iter().collect();
        let mut again = true;

        while again {
            let mut result_combined_singles = Vec::new();
            let mut remove_list: Vec<&Segment> = Vec::new();

            for s in &result_with_singles {
        
                for n in &result_with_singles_copy {
                    // println!("segments: {}  {}", result_with_singles_copy.len(), remove_list.len());
        
                    if s.end.x == n.start.x && s.end.y == n.start.y && n.slope() == s.slope() {
                        remove_list.push(n);
                        remove_list.push(s);
                        let combined_segment = Rc::new(Segment::new(
                            Point2::new(s.start.x, s.start.y),
                            Point2::new(n.end.x, n.end.y),
                        ));
                        result_combined_singles.push(combined_segment);
                        break;
                    } else if !std::ptr::eq(s, *n)
                        && s.end.x == n.end.x
                        && s.end.y == n.end.y
                        && n.slope() == s.slope()
                    {
                        remove_list.push(n);
                        remove_list.push(s);
                        let combined_segment = Rc::new(Segment::new(
                            Point2::new(s.start.x, s.start.y),
                            Point2::new(n.end.x, n.end.y),
                        ));
                        result_combined_singles.push(combined_segment);
                        break;
                    } else if !std::ptr::eq(s, *n)
                        && s.start.x == n.start.x
                        && s.start.y == n.start.y
                        && n.slope() == s.slope()
                    {
                        remove_list.push(n);
                        remove_list.push(s);
                        let combined_segment = Rc::new(Segment::new(
                            Point2::new(s.start.x, s.start.y),
                            Point2::new(n.end.x, n.end.y),
                        ));
                        result_combined_singles.push(combined_segment);
                    }
                }
                // println!("Remaining segments: {}  {}", result_with_singles_copy.len(), remove_list.len());
                if remove_list.len() > 0 {
                    break;
                }
            }
            if remove_list.len() == 0 {
                again = false;
            }
            for e in &remove_list {
                let position = &result_with_singles_copy.iter().position(|&i| &i == e);
                if position.is_some() {
                    result_with_singles_copy.remove(position.unwrap());
                }
            }
            // result_with_singles_copy.retain(|e| remove_list.contains(&e));
            // println!("Remaining segments after removing: {}  {}", result_with_singles_copy.len(), remove_list.len());
        }

        // println!("Merged segments: {}", result_with_singles_copy.len());

        result_with_singles_copy
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
