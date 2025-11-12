mod vec3;
use crate::vec3::{Color, Point3, Vec3};

mod ray;
use crate::ray::Ray;
use crate::ray::ray_color;

mod hittable;
use crate::hittable::{HitRecord, Hittable};

mod hittable_list;
use crate::hittable_list::HittableList;

mod sphere;
use crate::sphere::Sphere;

mod common;
use crate::common::degrees_to_radians;

fn write_color(pixel_color: Color) {
    // maybe add ostream here
    let rbyte = (255.999 * pixel_color.x()) as u8;
    let gbyte = (255.999 * pixel_color.y()) as u8;
    let bbyte = (255.999 * pixel_color.z()) as u8;
    println!("{rbyte} {gbyte} {bbyte}");
}

fn main() {
    // Image setup
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        eprint!("\r Scanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
    eprint!("\rDone.                      \n");
}
