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

const SPHERE_CENTRE: Point = Point {
    x: 0.,
    y: 0.,
    z: -1.,
};

fn colour(ray: &Ray) -> Colour {
    let t = hit_sphere(&SPHERE_CENTRE, 0.5, &ray);
    if t > 0. {
        let mut n = (ray.at(t) - SPHERE_CENTRE).unit_vector();
        n += 1.;
        n *= 0.5;
        return n;
    }

    let unit_dir = ray.dir().unit_vector();
    let t = 0.5 * (unit_dir.y + 1.);
    (1. - t) * WHITE + t * BLUE
}

fn hit_sphere(centre: &Point, radius: f64, ray: &Ray) -> f64 {
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

    let a = d.len_squared();
    let half_b = d.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let delta = half_b * half_b - a * c;

    if delta < 0. {
        return -1.;
    }

    (-half_b - delta.sqrt()) / a
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

            let ray = Ray::new(&ORIGIN, &dir);

            let col = colour(&ray);

            w.write_all(col.as_colour_string().as_bytes())?;
        }
        l.write_all(format!("Lines remaining: {j}. Total lines: {HEIGHT}\n").as_bytes())?;
    }
    l.write_all(b"Done.")?;
    Ok(())
}
