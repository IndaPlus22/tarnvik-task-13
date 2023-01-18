use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let mut file = File::create("pic.ppm")?;
    file.write_all(b"P3 \n200 100\n255\n")?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b = 0.2;

            let ir = (255.99*r) as i32;
            let ig = (255.99*g) as i32;
            let ib = (255.99*b) as i32;

            let picture = ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
            file.write_all(picture.as_bytes())?;
        }
    }
    Ok(())
}