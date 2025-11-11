#[derive(Debug)]
struct vec3{x: f32, y: f32, z: f32}
impl vec3 {
    fn length_squared(&self) -> f32 {
        return self.x*self.x +self.y*self.y +self.z*self.z
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn cross(&self, other:Self) -> Self {
        Self{x: self.y*other.z - self.z*other.z,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x
        }
    }
    fn dot(&self, other:vec3) -> f32{
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    //fn unit_vector(&self) -> vec3 {
    //    self / self.length()
    // }
}
impl std::ops::Add<vec3> for vec3 {
    type Output = Self;
    fn add(self, other: vec3) -> Self::Output {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z+other.z}
    }
}
impl std::ops::Neg for vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self{x: -self.x, y: -self.y, z: -self.z}
    }
}
impl std::ops::Add<f32> for vec3 {
    type Output = Self;
    fn add(self, other: f32) -> Self::Output {
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

impl std::ops::Mul<f32> for vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self{x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl std::ops::Div<f32> for vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
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

use vec3 as Point;
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

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\r Scanlines remaining: {} ", image_height-j);
        for i in 0..image_width {
            let pixel_color = Color{x: (i as f32)/(image_width as f32 - 1.0), y: (j as f32)/(image_width as f32-1.0), z: 0.0};
            write_color(pixel_color);
                
        }
    }
    eprint!("\rDone.                      \n");
}
