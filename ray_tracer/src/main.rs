use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

//(everything is in one file becouse I got way to confused with how to access things and gave up)

//I did fail lin-algen but here is some vector math 
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

//This is kind of where the rust borrow non OOP everything confusion hit a wall I could not get over 
struct Hit_record{
    t: f64,
    p: Vec3,
    normal: Vec3
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

//This function looks really weird becouse it is this was just my attemt att tricking rust into doing what i wanted 
//becouse I could not get hit record to make any sense 
impl Sphere{
    pub fn hit (&self, r: &Ray, t_min: f64, t_max: f64, mut rec: Hit_record) -> (bool, Vec3){
        let oc = r.origin().subtract(&self.center);
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(r.origin().subtract(&self.center)) - self.radius * self.radius;
        let discriminant = b*b - a*c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min{
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.subtract(&self.center)).scalar_division(&self.radius);
                return (true, rec.normal)
            } 
            temp = (-b + (b*b-a*c)) / a;
            if temp < t_max && temp > t_min{
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.subtract(&self.center)).scalar_division(&self.radius);
                return (true, rec.normal)
            }
        }
        return (false, rec.normal);
    }
}

/*struct Hitable_list {
    vec: Vec<Sphere>,
    list_size: usize
}

impl Hitable_list {
    pub fn hit (self, r: Ray, t_min: f64, t_max: f64, rec: Hit_record) -> bool{
        let temp_rec: Hit_record;
        let hit_anything = false;
        let closest_so_far = t_max;
        for i in 0..self.list_size {
            if self.vec[i].hit(r, t_min, closest_so_far, temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        return hit_anything;
    }
}*/

//If it has rays it must be a ray tracer right
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

    pub fn point_at_parameter(&self, t : f64) -> Vec3{
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

//this is me halfway through trying to be able to implement more than one sphere
fn color2 (r: Ray, world_list: Vec<Sphere>) -> Vec3{
    let rec = Hit_record{
        t: 0.0,
        p: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0)
    };
    /*for world in world_list {
        let thing = world.hit(&r, 0.0, f64::MAX, rec);
        if thing.0{
           return thing.1.add(Vec3(1.0, 1.0, 1.0)).scalar_multiply(0.5);
        }
    }*/   
    let unit_direction = (r.direction()).unit_vector();
            let t = 0.5*(unit_direction.1 + 1.0);
            return Vec3(1.0, 1.0, 1.0)
                .scalar_multiply(1.0-t)
                .add(Vec3(0.5, 0.7, 1.0)
                .scalar_multiply(t));
}

fn color(r : Ray) -> Vec3{
    let mut t = hit_sphere(&r, Vec3(0.0,0.0,-1.0), 0.5);
    if (t > 0.0){
        let N = (r.point_at_parameter(t)
            .subtract(&Vec3(0.0, 0.0, -1.0)))
            .unit_vector();
        return N.add(Vec3(1.0, 1.0, 1.0)).scalar_multiply(0.5);
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
                b: lower_left_corner
                    .add(horizontal.scalar_multiply(u))
                    .add(vertical.scalar_multiply(v)) 
            };

            /*let mut world_list:Vec<Sphere> = Vec::new();
            world_list.push(Sphere{
                center: Vec3(0.0,0.0,-1.0),
                radius: 0.5});
            world_list.push(Sphere{
                center: Vec3(0.0,-100.5,-1.0),
                radius: 100.0});*/
            
            //let col = color2(r, world_list);

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