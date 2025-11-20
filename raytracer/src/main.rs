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

mod material;
use crate::material::{Dielectric, Lambertian, Material, Metal};

mod common;
use crate::common::degrees_to_radians;

use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 100;
    cam.render(&world);
}
