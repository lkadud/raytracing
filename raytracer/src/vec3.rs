use std::ops::{Add, AddAssign,Sub, Div,DivAssign, Mul, MulAssign, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(f64,f64,f64);
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
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
        return self.0*self.0 +self.1*self.1 +self.2*self.2
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn cross(&self, other:Self) -> Self {
        Self(self.1*other.2 - self.2*other.2,
            self.2*other.0 - self.0*other.2,
            self.0*other.1 - self.1*other.0
        )
    }
    pub fn dot(&self, other:Vec3) -> f64{
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
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
        Self(self.0 + other.0, self.1 + other.1, self.2+other.2)
    }
}
impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        Self(self.0 + other, self.1 + other, self.2+other)
    }
}
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3){
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3){
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
    fn div_assign(&mut self, other: Vec3){
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

//use Vec3 as point3;
pub type Point3 = Vec3;
pub type Color = Vec3;