
use crate::vec3::{Color, Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray{origin: Point3, direction: Vec3}
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray{origin: origin, direction: direction}
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
pub fn ray_color(r: &Ray) -> Color {
    if (hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r)) {
        return Color::new(1.0, 0.0, 0.0)
    }
    let unit_direction = r.direction().unit_vector();
    let a  = (unit_direction.y() + 1.0)*0.5;
    Color::new(1.0, 1.0,1.0) * (1.0 - a) + Color::new(0.5, 0.7 ,1.0) * a    
}

fn hit_sphere(center: Point3, radius: f64, r:&Ray) ->bool {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant >= 0.0
}