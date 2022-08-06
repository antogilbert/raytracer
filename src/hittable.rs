use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub n: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_n: Vec3) {
        self.front_face = ray.dir().dot(&outward_n) < 0.;
        self.n = if self.front_face {
            outward_n
        } else {
            -outward_n
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
