use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

pub type Colour = Vec3;
pub type Point = Vec3;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negate() {
        let nv = -Vec3::new(1., 2., 3.);
        let pv = Vec3::new(1., 2., 3.);

        assert!(nv.x + pv.x < 1e-10);
        assert!(nv.y + pv.y < 1e-10);
        assert!(nv.z + pv.z < 1e-10);
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
    fn mul_assign() {
        let mut r = Vec3::new(1., 2., 3.);
        r *= 2.;

        assert!(r.x - 2. < 1e-10);
        assert!(r.y - 4. < 1e-10);
        assert!(r.z - 6. < 1e-10);
    }

    #[test]
    fn div_assign() {
        let mut r = Vec3::new(2., 4., 6.);
        r /= 2.;

        assert!(r.x - 1. < 1e-10);
        assert!(r.y - 2. < 1e-10);
        assert!(r.z - 3. < 1e-10);
    }
}
