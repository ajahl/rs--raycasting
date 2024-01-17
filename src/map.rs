use cgmath::{Point2, Vector2};

use crate::segment::Segment;

pub struct Map {}

impl Map {
    pub fn make_map(map: &str) -> Vec<Segment> {
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

        /*
                print(f"Segments: {len(result)}")

        # if any segment exists twice, then it was between two map items
        # and both can be removed!
        result = [item for item in result if result.count(item) == 1]

        print(f"Filtered duplicated wall segments: {len(result)}")

        cont = True
        while cont:
            remove_list = []
            for s in result:
                for n in result:
                    if s.end.x == n.start.x and s.end.y == n.start.y and n.slope == s.slope:
                        remove_list += [n, s]
                        result.append(
                            Segment(Point(s.start.x, s.start.y), Point(n.end.x, n.end.y))
                        )
                        break
                    elif (
                        s is not n
                        and s.end.x == n.end.x
                        and s.end.y == n.end.y
                        and n.slope == s.slope
                    ):
                        remove_list += [n, s]
                        result.append(
                            Segment(
                                Point(s.start.x, s.start.y), Point(n.start.x, n.start.y)
                            )
                        )
                        break
                    elif (
                        s is not n
                        and s.start.x == n.start.x
                        and s.start.y == n.start.y
                        and n.slope == s.slope
                    ):
                        remove_list += [n, s]
                        result.append(
                            Segment(Point(s.end.x, s.end.y), Point(n.end.x, n.end.y))
                        )
                        break

                if len(remove_list) > 0:
                    break

            if len(remove_list) == 0:
                cont = False

            for i in remove_list:
                result.remove(i)

        print(f"Merged segments: {len(result)}")
             */

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
