use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use raytracer::vec3;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::create("img.ppm")?;
    let log = File::create("raytracer.log")?;
    let mut w = BufWriter::new(file);
    let mut l = BufWriter::new(log);
    w.write_all(b"P3\n")?;
    w.write_all(format!("{WIDTH} {HEIGHT}\n").as_bytes())?;
    w.write_all(b"255\n")?;

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let r = i as f32 / ((WIDTH - 1) as f32);
            let g = j as f32 / ((HEIGHT - 1) as f32);
            let b: f32 = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            w.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
        }
        l.write_all(format!("Lines remaining: {j}. Total lines: {HEIGHT}\n").as_bytes())?;
    }
    l.write_all(b"Done.")?;
    Ok(())
}
