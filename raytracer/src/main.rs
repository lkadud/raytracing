#[derive(Debug, Clone, Copy)]
struct vec3(f64,f64,f64);
impl vec3 {
    fn length_squared(&self) -> f64 {
        return self.0*self.0 +self.1*self.1 +self.2*self.2
    }
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    fn cross(&self, other:Self) -> Self {
        Self(self.1*other.2 - self.2*other.2,
            self.2*other.0 - self.0*other.2,
            self.0*other.1 - self.1*other.0
        )
    }
    fn dot(&self, other:vec3) -> f64{
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }
    fn unit_vector(self) -> vec3 {
        self / self.length()
    }
}

impl std::ops::Neg for vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
impl std::ops::Sub<vec3> for vec3 {
    type Output = Self;
    fn sub(self, other: vec3) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl std::ops::Add<vec3> for vec3 {
    type Output = Self;
    fn add(self, other: vec3) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2+other.2)
    }
}
impl std::ops::Add<f64> for vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        Self(self.0 + other, self.1 + other, self.2+other)
    }
}
impl std::ops::AddAssign<vec3> for vec3 {
    fn add_assign(&mut self, other: vec3){
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl std::ops::MulAssign<vec3> for vec3 {
    fn mul_assign(&mut self, other: vec3){
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}
impl std::ops::Mul<vec3> for vec3 {
    type Output = Self;
    fn mul(self, other: vec3) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl std::ops::Mul<f64> for vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl std::ops::Div<f64> for vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}
impl std::ops::DivAssign<vec3> for vec3 {
    fn div_assign(&mut self, other: vec3){
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

use vec3 as point3;
use vec3 as Color;

fn write_color(pixel_color: Color) { // maybe add ostream here
    let rbyte = (255.999 * pixel_color.0) as u8;
    let gbyte = (255.999 * pixel_color.1) as u8;
    let bbyte = (255.999 * pixel_color.2) as u8;
    print!("{rbyte} {gbyte} {bbyte}\n");
}

#[derive(Debug, Clone, Copy)]
struct ray{origin: point3, direction: vec3}
impl ray {
    fn origin(&self) -> point3 {
        self.origin
    }
    fn direction(&self) -> vec3 {
        self.direction
    }
    fn at(&self, t: f64) -> point3 {
        self.origin + self.direction * t
    }
}
fn ray_color(r: &ray) -> Color {
    if (hit_sphere(point3(0.0, 0.0, -1.0), 0.5, &r)) {
        return Color(1.0, 0.0, 0.0)
    }
    let unit_direction = r.direction().unit_vector();
    let a  = (unit_direction.1 + 1.0)*0.5;
    Color(1.0, 1.0,1.0) * (1.0 - a) + Color(0.5, 0.7 ,1.0) * a    
}

fn hit_sphere(center: point3, radius: f64, r:&ray) ->bool {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant >= 0.0
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
    let viewport_width= viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = point3(0.0, 0.0, 0.0); 

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = vec3(viewport_width,0.0,0.0);
    let viewport_v = vec3(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center  - vec3(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v)*0.5;

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        eprint!("\r Scanlines remaining: {} ", image_height-j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = ray{origin: camera_center, direction: ray_direction};
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
                
        }
    }
    eprint!("\rDone.                      \n");
}
