use crate::vec3::{Point, Vec3};

pub struct Ray {
    dir: Vec3,
    orig: Point,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn new(orig: &Point, dir: &Vec3) -> Self {
        Self {
            dir: dir.clone(),
            orig: orig.clone(),
        }
    }

    pub fn origin(&self) -> Point {
        self.orig
    }
}
