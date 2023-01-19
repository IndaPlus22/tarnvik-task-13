use std::fs::File;
use std::io::prelude::*;

struct Vec3 (f64, f64, f64);

impl Vec3 {
    pub fn add(self, v: Vec3) -> Vec3{
        Vec3(
            self.0 + v.0, 
            self.1 + v.1, 
            self.2 + v.2
        )
    }

    pub fn scalar_multiply(self, s: f64) -> Vec3{
        Vec3(
            self.0 * s,
            self.1 * s,
            self.2 * s
        )
    }

    pub fn scalar_division(self, s: &f64) -> Vec3{
        Vec3(
            self.0 / s,
            self.1 / s,
            self.2 / s
        )
    }

    pub fn length(&self) -> f64{
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    pub fn unit_vector(self) -> Vec3{

        let length = &self.length();

        self.scalar_division(length)
    }
}

struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn origin(self) -> Vec3{
        self.a
    }

    pub fn direction(self) -> Vec3{
        self.b
    }

    pub fn point_at_parameter(self, t : f64) -> Vec3{
        self.a.add(self.b.scalar_multiply(t))
    }
}

fn color(r : Ray) -> Vec3{
    let unit_direction = (r.direction()).unit_vector();
    let t = 0.5*(unit_direction.1 + 1.0);
    Vec3(1.0, 1.0, 1.0)
        .scalar_multiply(1.0-t)
        .add(Vec3(0.5, 0.7, 1.0)
        .scalar_multiply(t))
}

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let mut file = File::create("pic.ppm")?;
    file.write_all((String::from("P3 \n") + &nx.to_string() + " " +  &ny.to_string() + " \n255\n").as_bytes())?;



    for j in (0..ny).rev() {
        for i in 0..nx {
            let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
            let horizontal = Vec3(4.0, 0.0, 0.0);
            let vertical =Vec3(0.0, 2.0, 0.0);
            let origin =Vec3(0.0, 0.0, 0.0);

            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let r = Ray {
                a: origin,
                b: lower_left_corner.add(horizontal.scalar_multiply(u)).add(vertical.scalar_multiply(v)) 
            };

            let col = color(r);

            let ir = (255.99*col.0) as i32;
            let ig = (255.99*col.1) as i32;
            let ib = (255.99*col.2) as i32;

            let picture_string = ir.to_string() + " " + &ig.to_string() + " " + &ib.to_string() + "\n";
            file.write_all(picture_string.as_bytes())?;
        }
    }
    Ok(())
}