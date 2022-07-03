use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use raytracer::{
    ray::Ray,
    vec3::{Point, Vec3},
};

const WIDTH: i64 = 400;
const ASPECT_RATIO: f64 = 16. / 9.;
const HEIGHT: i64 = (WIDTH as f64 / ASPECT_RATIO) as i64;

const VIEWPORT_HEIGHT: f64 = 2.;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.;

const ORIGIN: Point = Point {
    x: 0.,
    y: 0.,
    z: 0.,
};

const HORIZONTAL: Vec3 = Vec3 {
    x: VIEWPORT_WIDTH,
    y: 0.,
    z: 0.,
};

const VERTICAL: Vec3 = Vec3 {
    x: 0.,
    y: VIEWPORT_HEIGHT,
    z: 0.,
};

const DEPTH: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: FOCAL_LENGTH,
};

fn main() -> Result<(), Box<dyn Error>> {
    let lower_left = ORIGIN - HORIZONTAL / 2. - VERTICAL / 2. - DEPTH;

    let file = File::create("img.ppm")?;
    let log = File::create("raytracer.log")?;
    let mut w = BufWriter::new(file);
    let mut l = BufWriter::new(log);
    w.write_all(b"P3\n")?;
    w.write_all(format!("{WIDTH} {HEIGHT}\n").as_bytes())?;
    w.write_all(b"255\n")?;

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u = i as f64 / ((WIDTH - 1) as f64);
            let v = j as f64 / ((HEIGHT - 1) as f64);

            let dir = lower_left + u * HORIZONTAL + v * VERTICAL - ORIGIN;

            let ray = Ray::new(&ORIGIN, &dir);
            let colour = ray.colour();

            w.write_all(colour.as_colour_string().as_bytes())?;
        }
        l.write_all(format!("Lines remaining: {j}. Total lines: {HEIGHT}\n").as_bytes())?;
    }
    l.write_all(b"Done.")?;
    Ok(())
}
