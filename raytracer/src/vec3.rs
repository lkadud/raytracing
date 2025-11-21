use crate::common::{random_number, random_number_interval};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3(f64, f64, f64);
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }
    pub fn random() -> Vec3 {
        Vec3(random_number(), random_number(), random_number())
    }
    pub fn random_interval(min: f64, max: f64) -> Vec3 {
        Vec3(
            random_number_interval(min, max),
            random_number_interval(min, max),
            random_number_interval(min, max),
        )
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
    pub fn dot(&self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_interval(-1., 1.);
            if p.length_squared() <= 1.0 {
                //Small number error?
                return p.unit_vector();
            }
        }
    }
    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * v.dot(n) * 2.0
    }
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -uv.dot(n).min(1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_para = -n * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_para
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        Self(self.0 + other, self.1 + other, self.2 + other)
    }
}
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

//use Vec3 as point3;
pub type Point3 = Vec3;
pub type Color = Vec3;
