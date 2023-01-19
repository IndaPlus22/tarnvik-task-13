use core::num::dec2flt::float;
use std::fs::File;
use std::io::prelude::*;

struct Vec3 (f64, f64, f64);

impl Vec3 {
    pub fn add(&self, v: Vec3) -> Vec3{
        Vec3(
            self.0 + v.0, 
            self.1 + v.1, 
            self.2 + v.2
        )
    }

    pub fn subtract(&self, v: &Vec3) -> Vec3{
        Vec3(
            self.0 - v.0, 
            self.1 - v.1, 
            self.2 - v.2
        )
    }

    pub fn scalar_multiply(&self, s: f64) -> Vec3{
        Vec3(
            self.0 * s,
            self.1 * s,
            self.2 * s
        )
    }

    pub fn scalar_division(&self, s: &f64) -> Vec3{
        Vec3(
            self.0 / s,
            self.1 / s,
            self.2 / s
        )
    }

    //scalar product
    pub fn dot (&self, v: Vec3) -> f64{
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2        
    }

    pub fn length(&self) -> f64{
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3{

        let length = &self.length();
        self.scalar_division(length)
    }
}

struct Hit_record{
    f: f64,
    p: Vec3,
    normal: Vec3
}

//hitable?????

struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere{
    pub fn hit(r: &Ray, center: Vec3, radius: f64) {
        let oc = r.origin().subtract(&center);
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(r.origin().subtract(&center)) - radius * radius;
        let discriminant = b*b - a*c;

        if discriminant > 0 {}
    }
    
}

struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn origin(&self) -> Vec3{
        let v = Vec3(self.a.0, self.a.1, self.a.2);
        v
    }

    pub fn direction(&self) -> Vec3{
        let v = Vec3(self.b.0, self.b.1, self.b.2);
        v
    }

    pub fn point_at_parameter(self, t : f64) -> Vec3{
        self.a.add(self.b.scalar_multiply(t))
    }
}

fn hit_sphere(r: &Ray, center: Vec3, radius: f64) -> f64{
    let oc = r.origin().subtract(&center);
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(r.origin().subtract(&center)) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0*a)
    }
}

fn color(r : Ray) -> Vec3{
    let mut t = hit_sphere(&r, Vec3(0.0,0.0,-1.0), 0.5);
    if (t > 0.0){
        let N = (r.point_at_parameter(t)
            .subtract(&Vec3(0.0, 0.0, -1.0)))
            .unit_vector();
        return N.add(Vec3(1.0, 1.0, 1.0)).scalar_multiply(0.5);
        //return Vec3(N.0+1.0, N.1+1.0, N.2+1.0).scalar_multiply(0.5);        
    }
    let unit_direction = (r.direction()).unit_vector();
    t = 0.5*(unit_direction.1 + 1.0);
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