#[derive(Debug, Clone, Copy)]
struct vec3{x: f64, y: f64, z: f64} // Refactor to indexless struct
impl vec3 {
    fn length_squared(&self) -> f64 {
        return self.x*self.x +self.y*self.y +self.z*self.z
    }
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    fn cross(&self, other:Self) -> Self {
        Self{x: self.y*other.z - self.z*other.z,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x
        }
    }
    fn dot(&self, other:vec3) -> f64{
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    fn unit_vector(self) -> vec3 {
        self / self.length()
     }
}
impl std::ops::Neg for vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self{x: -self.x, y: -self.y, z: -self.z}
    }
}
impl std::ops::Sub<vec3> for vec3 {
    type Output = Self;
    fn sub(self, other: vec3) -> Self::Output {
        Self{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}
impl std::ops::Add<vec3> for vec3 {
    type Output = Self;
    fn add(self, other: vec3) -> Self::Output {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z+other.z}
    }
}
impl std::ops::Add<f64> for vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        Self{x: self.x + other, y: self.y + other, z: self.z+other}
    }
}
impl std::ops::AddAssign<vec3> for vec3 {
    fn add_assign(&mut self, other: vec3){
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl std::ops::MulAssign<vec3> for vec3 {
    fn mul_assign(&mut self, other: vec3){
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl std::ops::Mul<vec3> for vec3 {
    type Output = Self;
    fn mul(self, other: vec3) -> Self::Output {
        Self{x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl std::ops::Mul<f64> for vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self{x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl std::ops::Div<f64> for vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self{x: self.x / other, y: self.y / other, z: self.z / other}
    }
}
impl std::ops::DivAssign<vec3> for vec3 {
    fn div_assign(&mut self, other: vec3){
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

use vec3 as point3;
use vec3 as Color;

fn write_color(pixel_color: Color) { // maybe add ostream here
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let rbyte = (255.999 * r) as i8;
    let gbyte = (255.999 * g) as i8;
    let bbyte = (255.999 * b) as i8;

    print!("{rbyte} {gbyte} {bbyte}\n");
}

struct ray(point3, vec3);
impl ray {
    fn origin(self) -> point3 {
        self.0
    }
    fn direction(self) -> vec3 {
        self.1
    }
    fn at(self, t: f64) -> point3 {
        self.0 + self.1 * t
    }
/*     fn color(self) -> Color { // maybe aux function?
        let unit_direction = self.direction().unit_vector();
        let a  = (unit_direction.y + 1.0)*0.5;
        Color{x: 1.0, y: 1.0,z: 1.0} * (1.0 - a) * Color{x: 0.5, y: 0.7 ,z: 1.0} * a
    }*/
}
fn ray_color(r: ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    let a  = (unit_direction.y + 1.0)*0.5;
    Color{x: 1.0, y: 1.0,z: 1.0} * (1.0 - a) * Color{x: 0.5, y: 0.7 ,z: 1.0} * a    
}

fn main() {
    // Image setup
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if (image_height < 1) {1} else {image_height};

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width= viewport_height * (image_width / image_height) as f64; //or individual as?
    let camera_center = point3{x: 0.0, y: 0.0, z: 0.0}; 

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = vec3{x: viewport_width,y: 0.0,z: 0.0};
    let viewport_v = vec3{x: 0.0, y: -viewport_height, z: 0.0};

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center  - vec3{x:0.0, y:0.0, z:focal_length} - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v)*0.5;

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        eprint!("\r Scanlines remaining: {} ", image_height-j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = ray(camera_center, ray_direction);
            let pixel_color = ray_color(r);
            write_color(pixel_color);
                
        }
    }
    eprint!("\rDone.                      \n");
}
