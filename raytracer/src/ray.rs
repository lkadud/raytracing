use crate::{
    common,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    vec3::{Color, Point3, Vec3},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

/*
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / (a)
    }
}
*/
