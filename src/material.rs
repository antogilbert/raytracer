use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Colour, Vec3},
};

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Colour) -> Option<Ray>;
}

pub struct Lambertian {
    pub albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Colour) -> Option<Ray> {
        let mut scatter_dir = rec.n + Vec3::random_unit_vec();
        if scatter_dir.near_zero() {
            scatter_dir = rec.n;
        }

        *attenuation = self.albedo;
        Some(Ray::new(&rec.p, &scatter_dir))
    }
}
