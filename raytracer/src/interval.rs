use crate::common::INFINITY;

pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    //pub fn new() -> Interval {
    //    Interval {
    //        min: -INFINITY,
    //        max: INFINITY,
    //    }
    //}
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min: min, max: max }
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        };
        if x > self.max {
            return self.max;
        };
        return x;
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
}
