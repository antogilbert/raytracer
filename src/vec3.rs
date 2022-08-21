use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use rand::Rng;

use crate::constants::{MAX_CLAMP, MIN_CLAMP, MIN_DIM, SAMPLES_PER_PIXEL};

pub fn clamp(x: f64) -> f64 {
    if x < MIN_CLAMP {
        return MIN_CLAMP;
    }
    if x > MAX_CLAMP {
        return MAX_CLAMP;
    }
    x
}

pub type Colour = Vec3;
pub type Point = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::default(), |v, elem| v + elem)
    }
}

impl Vec3 {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0.0..1.),
            y: rng.gen_range(0.0..1.),
            z: rng.gen_range(0.0..1.),
        }
    }

    pub fn random_bounded(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_bounded(-1., 1.);
            if p.len_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_hemisphere(n: &Vec3) -> Self {
        let in_sphere = Vec3::random_in_unit_sphere();

        if n.dot(&in_sphere) < 0. {
            return -in_sphere;
        }

        in_sphere
    }

    pub fn random_unit_vec() -> Self {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn as_colour_string(&self) -> String {
        let r = self.x / SAMPLES_PER_PIXEL as f64;
        let g = self.y / SAMPLES_PER_PIXEL as f64;
        let b = self.z / SAMPLES_PER_PIXEL as f64;

        let scale = 1. / SAMPLES_PER_PIXEL as f64;

        let r = (scale * r).sqrt();
        let g = (scale * g).sqrt();
        let b = (scale * b).sqrt();

        let ir = (256. * clamp(r)) as i64;
        let ig = (256. * clamp(g)) as i64;
        let ib = (256. * clamp(b)) as i64;

        format!("{ir} {ig} {ib}")
    }

    pub fn as_colour_bytes(&self) -> Vec<u8> {
        let r = self.x / SAMPLES_PER_PIXEL as f64;
        let g = self.y / SAMPLES_PER_PIXEL as f64;
        let b = self.z / SAMPLES_PER_PIXEL as f64;

        let ir = (256. * clamp(r)) as u8;
        let ig = (256. * clamp(g)) as u8;
        let ib = (256. * clamp(b)) as u8;

        vec![ir, ig, ib]
    }

    pub fn cross(&self, rhs: &Vec3) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn near_zero(&self) -> bool {
        (self.x.abs() < MIN_DIM) && (self.y.abs() < MIN_DIM) && (self.z.abs() < MIN_DIM)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        return *self - 2. * self.dot(normal) * *normal;
    }

    pub fn refract(&self, normal: &Vec3, refraction_ratio: f64) -> Self {
        let cos_theta = (-self.dot(normal)).min(1.);
        let r_out_perp = refraction_ratio * (*self + cos_theta * *normal);
        let r_out_par = (-(1. - r_out_perp.len_squared()).sqrt()) * *normal;
        r_out_par + r_out_perp
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3 {{ {}, {}, {} }}", self.x, self.y, self.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let l = Vec3::default();
        let r = Vec3::new(1., 2., 3.);
        let s = l + r;
        assert!(s.x - 1. < 1e-10);
        assert!(s.y - 2. < 1e-10);
        assert!(s.z - 3. < 1e-10);
    }

    #[test]
    fn add_assign() {
        let mut l = Vec3::default();
        let r = Vec3::new(1., 2., 3.);

        l += r;

        assert!(l.x - 1. < 1e-10);
        assert!(l.y - 2. < 1e-10);
        assert!(l.z - 3. < 1e-10);
    }

    #[test]
    fn as_colour_string() {
        let r = Vec3::new(0., 1., 0.25);

        assert_eq!(r.as_colour_string(), "0 255 63\n");
    }

    #[test]
    fn cross_product() {
        let r = Vec3::new(2., 3., 4.);
        let l = Vec3::new(5., 6., 7.);

        let res = Vec3::new(-3., 6., -3.);
        let r_cross_l = r.cross(&l);

        assert!(res.x - r_cross_l.x < 1e-10);
        assert!(res.y - r_cross_l.y < 1e-10);
        assert!(res.z - r_cross_l.z < 1e-10);
    }

    #[test]
    fn display() {
        let r = Vec3::new(1., 2., 3.);
        assert_eq!(r.to_string(), "Vec3 { 1, 2, 3 }".to_string());

        let r = Vec3::new(1.5, 2.5, 3.5);
        assert_eq!(r.to_string(), "Vec3 { 1.5, 2.5, 3.5 }".to_string());

        let r = Vec3::new(1.50, 2.05, 3.456);
        assert_eq!(r.to_string(), "Vec3 { 1.5, 2.05, 3.456 }".to_string());
    }

    #[test]
    fn div() {
        let r = Vec3::new(2., 4., 6.);
        let d = r / 2.;

        assert!(d.x - 1. < 1e-10);
        assert!(d.y - 2. < 1e-10);
        assert!(d.z - 3. < 1e-10);
    }

    #[test]
    fn div_assign() {
        let mut r = Vec3::new(2., 4., 6.);
        r /= 2.;

        assert!(r.x - 1. < 1e-10);
        assert!(r.y - 2. < 1e-10);
        assert!(r.z - 3. < 1e-10);
    }

    #[test]
    fn dot_product() {
        let r = Vec3::new(1., 2., 3.);
        let l = Vec3::new(2., 4., 6.);

        assert_eq!(r.dot(&l), 2. + 2. * 4. + 3. * 6.);
        assert_eq!(l.dot(&r), 2. + 2. * 4. + 3. * 6.);
    }

    #[test]
    fn len() {
        let r = Vec3::new(1., 2., 2.);
        assert!(r.len() - 3. < 1e-10)
    }

    #[test]
    fn mul() {
        let l = Vec3::new(1., 2., 3.);
        let r = Vec3::new(1., 2., 3.);
        let s = l * r;
        let f = 2.;

        assert!(s.x - 1. < 1e-10);
        assert!(s.y - 4. < 1e-10);
        assert!(s.z - 9. < 1e-10);

        let n = f * s.clone();

        assert!(n.x - 2. < 1e-10);
        assert!(n.y - 8. < 1e-10);
        assert!(n.z - 18. < 1e-10);

        let n = s * f;

        assert!(n.x - 2. < 1e-10);
        assert!(n.y - 8. < 1e-10);
        assert!(n.z - 18. < 1e-10);
    }

    #[test]
    fn mul_assign() {
        let mut r = Vec3::new(1., 2., 3.);
        r *= 2.;

        assert!(r.x - 2. < 1e-10);
        assert!(r.y - 4. < 1e-10);
        assert!(r.z - 6. < 1e-10);
    }

    #[test]
    fn negate() {
        let nv = -Vec3::new(1., 2., 3.);
        let pv = Vec3::new(1., 2., 3.);

        assert!(nv.x + pv.x < 1e-10);
        assert!(nv.y + pv.y < 1e-10);
        assert!(nv.z + pv.z < 1e-10);
    }

    #[test]
    fn new() {
        let pv = Vec3::new(1., 2., 3.);

        assert!(1. - pv.x < 1e-10);
        assert!(2. - pv.y < 1e-10);
        assert!(3. - pv.z < 1e-10);
    }

    #[test]
    fn sub() {
        let l = Vec3::default();
        let r = Vec3::new(1., 2., 3.);
        let s = l - r;
        assert!(s.x + 1. < 1e-10);
        assert!(s.y + 2. < 1e-10);
        assert!(s.z + 3. < 1e-10);
    }

    #[test]
    fn unit_vector() {
        let r = Vec3::new(1., 2., 2.);
        let u = r.unit_vector();

        assert!(u.x - 1. / 3. < 1e-10);
        assert!(u.y - 2. / 3. < 1e-10);
        assert!(u.z - 2. / 3. < 1e-10);
    }

    #[test]
    fn sum() {
        let v = vec![Vec3::new(1., 2., 3.), Vec3::new(5., 7., 1.)];
        let sum: Vec3 = v.into_iter().sum();
        assert!(sum.x - 1. + 5. < 1e-10);
        assert!(sum.y - 2. + 7. < 1e-10);
        assert!(sum.z - 3. + 1. < 1e-10);
    }
}
