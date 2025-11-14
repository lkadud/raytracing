use crate::{
    common,
    hittable::HitRecord,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

fn write_color(pixel_color: Color) {
    // maybe add ostream here
    let intensity = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * linear_to_gamma(intensity.clamp(pixel_color.x()))) as u8;
    let gbyte = (256.0 * linear_to_gamma(intensity.clamp(pixel_color.y()))) as u8;
    let bbyte = (256.0 * linear_to_gamma(intensity.clamp(pixel_color.z()))) as u8;
    println!("{rbyte} {gbyte} {bbyte}");
}

#[derive(Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        // Render
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\r Scanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(pixel_color * self.pixel_samples_scale);
            }
        }
        eprint!("\rDone.                      \n");
    }

    fn initialize(&mut self) {
        // Image setup
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::new();
        if world.hit(r, Interval::new(0.001, common::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit_vector();
            return self.ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.5;
        }

        let unit_direction = r.direction().unit_vector();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            common::random_number() - 0.5,
            common::random_number() - 0.5,
            0.0,
        )
    }
}
