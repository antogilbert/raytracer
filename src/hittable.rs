use std::sync::Arc;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub mat: Arc<dyn Material>,
    pub n: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(mat: &Arc<dyn Material>) -> Self {
        Self {
            t: 0.,
            p: Point::new(0., 0., 0.),
            n: Vec3::new(0., 0., 0.),
            mat: mat.clone(),
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_n: Vec3) {
        self.front_face = ray.dir().dot(&outward_n) < 0.;
        self.n = if self.front_face {
            outward_n
        } else {
            -outward_n
        };
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut is_hit: Option<HitRecord> = None;
        let mut closest = t_max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(ray, t_min, closest) {
                closest = rec.t;
                is_hit = Some(rec);
            }
        }

        is_hit
    }
}
