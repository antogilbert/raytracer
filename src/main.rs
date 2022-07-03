use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use raytracer::{
    ray::Ray,
    vec3::{Colour, Point, Vec3},
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

const RED: Colour = Colour {
    x: 1.,
    y: 0.,
    z: 0.,
};

fn hit_sphere(centre: &Point, radius: f64, ray: &Ray) -> bool {
    // Equation for sphere intersection with the ray
    //
    // t^2 b*b + t 2b*(O-C) + (O-C)*(O-C) -r^2 = 0
    //
    // b: direction vector
    // O: ray origin
    // C: sphere centre
    // r: sphere radius
    // t: unknown. ray parameter. Ray = O + t*b

    let oc = ray.origin() - *centre;
    let d = ray.dir();

    let a = d.dot(&d);
    let b = 2. * d.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let delta = b * b - 4. * a * c;

    delta > 0.
}

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

            let p = Point::new(0., 0., -1.);
            let ray = Ray::new(&ORIGIN, &dir);
            if hit_sphere(&p, 0.5, &ray) {
                w.write_all(RED.as_colour_string().as_bytes())?;
            } else {
                let colour = ray.colour();

                w.write_all(colour.as_colour_string().as_bytes())?;
            }
        }
        l.write_all(format!("Lines remaining: {j}. Total lines: {HEIGHT}\n").as_bytes())?;
    }
    l.write_all(b"Done.")?;
    Ok(())
}
