use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use raytracer::vec3::Colour;

const WIDTH: i64 = 256;
const HEIGHT: i64 = 256;

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
            let r = i as f64 / ((WIDTH - 1) as f64);
            let g = j as f64 / ((HEIGHT - 1) as f64);
            let b: f64 = 0.25;
            let colour = Colour::new(r, g, b);

            w.write_all(colour.as_colour_string().as_bytes())?;
        }
        l.write_all(format!("Lines remaining: {j}. Total lines: {HEIGHT}\n").as_bytes())?;
    }
    l.write_all(b"Done.")?;
    Ok(())
}
