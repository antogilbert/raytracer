use crate::{
    constants::ORIGIN,
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
    pub fn new(from: &Point, to: &Point, vup: &Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_h = 2. * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = (*from - *to).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let hor = viewport_w * u;
        let ver = viewport_h * v;
        let origin = *from;

        Self {
            origin,
            hor,
            ver,
            low_left_corner: origin - hor / 2. - ver / 2. - w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.low_left_corner + u * self.hor + v * self.ver - self.origin),
        )
    }
}
