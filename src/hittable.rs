use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

#[derive(Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub n: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: 0.,
            p: Point::new(0., 0., 0.),
            n: Vec3::new(0., 0., 0.),
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
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
