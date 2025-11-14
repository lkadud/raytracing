mod vec3;
use crate::vec3::{Color, Point3, Vec3};

mod ray;
use crate::ray::Ray;

mod hittable;
use crate::hittable::{HitRecord, Hittable};

mod hittable_list;
use crate::hittable_list::HittableList;

mod sphere;
use crate::sphere::Sphere;

mod interval;
use crate::interval::Interval;

mod camera;
use crate::camera::Camera;

mod common;
use crate::common::degrees_to_radians;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
