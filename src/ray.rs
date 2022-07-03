use crate::vec3::{Colour, Point, Vec3};

const WHITE: Colour = Colour {
    x: 1.,
    y: 1.,
    z: 1.,
};

const BLUE: Colour = Colour {
    x: 0.5,
    y: 0.7,
    z: 1.,
};

pub struct Ray {
    dir: Vec3,
    orig: Point,
}

impl Ray {
    pub fn colour(&self) -> Colour {
        let unit_dir = self.dir.unit_vector();
        let t = 0.5 * (unit_dir.y + 1.);
        (1. - t) * WHITE + t * BLUE
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
