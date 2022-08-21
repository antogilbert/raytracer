use crate::{
    constants::WHITE,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Colour, Vec3},
};

use rand::Rng;

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Colour) -> Option<Ray>;
}

pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn new(a: &Colour) -> Self {
        Self { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord, attenuation: &mut Colour) -> Option<Ray> {
        let mut scatter_dir = rec.n + Vec3::random_unit_vec();
        if scatter_dir.near_zero() {
            scatter_dir = rec.n;
        }

        *attenuation = self.albedo;
        Some(Ray::new(&rec.p, &scatter_dir))
    }
}

pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &Colour, f: f64) -> Self {
        Self {
            albedo: *a,
            fuzz: if f < 1. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Colour) -> Option<Ray> {
        let reflected = ray.dir().unit_vector().reflect(&rec.n);
        let scattered = Ray::new(
            &rec.p,
            &(reflected + self.fuzz * Vec3::random_in_unit_sphere()),
        );
        *attenuation = self.albedo;

        if scattered.dir().dot(&rec.n) < 0. {
            return None;
        }

        Some(scattered)
    }
}

pub struct Dielectric {
    pub refraction_idx: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { refraction_idx: ir }
    }

    fn reflectance(cos: f64, refraction_idx: f64) -> f64 {
        let r0 = ((1. - refraction_idx) / (1. + refraction_idx)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Colour) -> Option<Ray> {
        *attenuation = WHITE;
        let refraction_ratio = if rec.front_face {
            1. / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_dir = ray.dir().unit_vector();

        let cos_theta = (-unit_dir).dot(&rec.n).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let rand = rand::thread_rng().gen_range(0.0..1.);
        let cannot_refract = (refraction_ratio * sin_theta > 1.)
            || (Dielectric::reflectance(cos_theta, refraction_ratio) > rand);

        let direction = if cannot_refract {
            unit_dir.refract(&rec.n, refraction_ratio)
        } else {
            unit_dir.reflect(&rec.n)
        };

        Some(Ray::new(&rec.p, &direction))
    }
}
