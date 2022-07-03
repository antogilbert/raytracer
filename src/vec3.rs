use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

pub type Colour = Vec3;
pub type Point = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn as_colour_string(&self) -> String {
        let ir = (255.999 * self.x) as i64;
        let ig = (255.999 * self.y) as i64;
        let ib = (255.999 * self.z) as i64;
        format!("{ir} {ig} {ib}\n")
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

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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
}
