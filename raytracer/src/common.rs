use rand::Rng;
pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_number() -> f64 {
    let mut rng = rand::rng();
    let num: f64 = rng.random();
    num
}

pub fn random_number_interval(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    let num: f64 = rng.random_range(min..max);
    num
}
