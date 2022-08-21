use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Camera {
    origin: Point,
    low_left_corner: Point,
    hor: Vec3,
    ver: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        from: &Point,
        to: &Point,
        vup: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_h = 2. * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = (*from - *to).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let hor = focus_dist * viewport_w * u;
        let ver = focus_dist * viewport_h * v;
        let origin = *from;

        Self {
            origin,
            hor,
            ver,
            low_left_corner: origin - hor / 2. - ver / 2. - focus_dist * w,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.low_left_corner + u * self.hor + v * self.ver - self.origin),
        )
    }
}
