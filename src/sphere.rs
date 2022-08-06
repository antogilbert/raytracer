use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Point,
};

struct Sphere {
    centre: Point,
    r: f64,
}

impl Sphere {
    pub fn new(centre: Point, r: f64) -> Self {
        Self { centre, r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.centre;
        let a = ray.dir().len_squared();
        let half_b = oc.dot(&ray.dir());
        let c = oc.len_squared() - self.r.powi(2);

        let delta_sq = half_b.powi(2) - a * c;
        if delta_sq < 0. {
            return false;
        }

        let delta = delta_sq.sqrt();

        let mut root = (-half_b - delta) / a;

        if root < t_min || root > t_max {
            root = (-half_b + delta) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(root);
        let outward_n = (record.p - self.centre) / self.r;
        record.set_face_normal(&ray, outward_n);
        true
    }
}
