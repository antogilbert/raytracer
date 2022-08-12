use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use rand::Rng;
use rayon::prelude::*;
use raytracer::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    ray::Ray,
    sphere::Sphere,
    vec3::{Colour, Point, Vec3},
};

use raytracer::constants::*;

fn get_colour(ray: Ray, world: &HittableList, recursion_depth: i32) -> Colour {
    if recursion_depth <= 0 {
        return BLACK;
    }

    if let Some(rec) = world.hit(&ray, 0.001, f64::INFINITY) {
        let tgt = rec.p + rec.n + Vec3::random_in_unit_sphere().unit_vector();
        return 0.5 * get_colour(Ray::new(&rec.p, &(tgt - rec.p)), world, recursion_depth - 1);
    }

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
    let mut world = HittableList::new();
    world.add(Sphere::new(Point::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(Point::new(0., -100.5, -1.), 100.));

    let cam = Camera::new();

    let file = File::create("img.ppm")?;
    let log = File::create("raytracer.log")?;
    let mut w = BufWriter::new(file);
    let mut l = BufWriter::new(log);
    w.write_all(b"P3\n")?;
    w.write_all(format!("{WIDTH} {HEIGHT}\n").as_bytes())?;
    w.write_all(b"255\n")?;

    let image = (0..HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            // println!("  Writing height level {j}");
            (0..WIDTH)
                .flat_map(|i| {
                    let px_colour: Vec3 = (0..SAMPLES_PER_PIXEL)
                        .map(|s| {
                            let mut rng = rand::thread_rng();
                            let u = (i as f64 + rng.gen_range(0.0..1.)) / ((WIDTH - 1) as f64);
                            let v = (j as f64 + rng.gen_range(0.0..1.)) / ((HEIGHT - 1) as f64);
                            let ray = cam.get_ray(u, v);

                            // println!("      Writing sample {s}");
                            get_colour(ray, &world, MAX_RECURSION)
                        })
                        .sum();
                    // println!("    Writing width level {i}");
                    px_colour.as_colour_bytes()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for nums in image.chunks(3) {
        w.write_all(format!("{} {} {}\n", nums[0], nums[1], nums[2]).as_bytes())?;
    }

    l.write_all(b"Done.")?;
    Ok(())
}
