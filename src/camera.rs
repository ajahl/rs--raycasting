extern crate cgmath;

use crate::ray::Ray;
use cgmath::{Point2, Rad, Vector2};
use std::f32::consts::PI;

pub struct Camera {
    position: Point2<f32>,
    direction: f32,
    viewing_angle: f32,
    planar_projection: bool,
}

impl Camera {
    pub fn new(position: Point2<f32>, direction: f32, viewing_angle: f32) -> Camera {
        Camera {
            position,
            direction,
            viewing_angle,
            planar_projection: true,
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.direction = (self.direction + angle) % (2. * PI)
    }

    pub fn rays(&self, count: u32) -> Vec<(Ray, Point2<f32>)> {
        let mut rays = Vec::new();

        let start_angle = self.direction - self.viewing_angle / 2.0;
        let end_angle = start_angle + self.viewing_angle;

        if self.planar_projection {
            let viewing_plane_start =
                self.position + Vector2::new(start_angle.sin(), start_angle.cos());
            let viewing_plane_end = self.position + Vector2::new(end_angle.sin(), end_angle.cos());

            let d_x = (viewing_plane_end.x - viewing_plane_start.x) / count as f32;
            let d_y = (viewing_plane_end.y - viewing_plane_start.y) / count as f32;

            for current in 0..count {
                let plane_point = Point2::new(
                    viewing_plane_start.x + (d_x * current as f32),
                    viewing_plane_start.y + (d_y * current as f32),
                );
                let ray_direction = plane_point - self.position;
                rays.push((
                    Ray {
                        origin: self.position,
                        direction: ray_direction,
                        angle: Ray::angle(ray_direction),
                    },
                    plane_point,
                ));
            }
        } else {
            let angle_slice = self.viewing_angle / count as f32;

            for current in 0..count {
                let angle = Rad(start_angle + current as f32 * angle_slice);
                let direction = Vector2::new(angle.0.cos(), angle.0.sin());
                rays.push((
                    Ray {
                        origin: self.position,
                        direction,
                        angle: start_angle + (current as f32) * angle_slice,
                    },
                    self.position,
                ));
            }
        }

        rays
    }

    pub fn calculate_fov(width: u32) -> f32 {
        let width = width as f32;
        let half_fov_rad = (PI / 2.0) / 2.0;
        2.0 * (width / 800.0 * half_fov_rad.tan()).atan()
    }
}
