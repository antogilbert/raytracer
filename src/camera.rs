use crate::{
    constants::{DEPTH, HORIZONTAL, ORIGIN, VERTICAL},
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Camera {
    origin: Point,
    low_left_corner: Point,
    hor: Vec3,
    ver: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            origin: ORIGIN,
            low_left_corner: ORIGIN - HORIZONTAL / 2. - VERTICAL / 2. - DEPTH,
            hor: HORIZONTAL,
            ver: VERTICAL,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.low_left_corner + u * self.hor + v * self.ver - self.origin),
        )
    }
}
