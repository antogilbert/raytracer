use crate::vec3::{Colour, Point, Vec3};

pub const WIDTH: i32 = 400;
pub const ASPECT_RATIO: f64 = 16. / 9.;
pub const HEIGHT: i32 = (WIDTH as f64 / ASPECT_RATIO) as i32;
pub const SAMPLES_PER_PIXEL: i64 = 100;

pub const VIEWPORT_HEIGHT: f64 = 2.;
pub const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f64 = 1.;
pub const MIN_CLAMP: f64 = 0.;
pub const MAX_CLAMP: f64 = 0.999;

pub const ORIGIN: Point = Point {
    x: 0.,
    y: 0.,
    z: 0.,
};

pub const HORIZONTAL: Vec3 = Vec3 {
    x: VIEWPORT_WIDTH,
    y: 0.,
    z: 0.,
};

pub const VERTICAL: Vec3 = Vec3 {
    x: 0.,
    y: VIEWPORT_HEIGHT,
    z: 0.,
};

pub const DEPTH: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: FOCAL_LENGTH,
};

pub const WHITE: Colour = Colour {
    x: 1.,
    y: 1.,
    z: 1.,
};

pub const BLUE: Colour = Colour {
    x: 0.5,
    y: 0.7,
    z: 1.,
};

pub const SPHERE_CENTRE: Point = Point {
    x: 0.,
    y: 0.,
    z: -1.,
};
