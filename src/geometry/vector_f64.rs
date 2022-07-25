use std::{cmp::Ordering, ops::{Add, Sub, Mul, Div}};


#[derive(Debug,Default,Clone,Copy)]
pub struct VectorF64 {
    pub x:f64,
    pub y:f64,
}

impl VectorF64 {
    pub fn new(x: f64, y:f64) -> Self {
        Self { x, y }
    }
    pub fn dot(&self, rhs: Self) -> f64 {
        return (self.x * rhs.x) + (self.y * rhs.y);
    }
    pub fn cross(&self, rhs: Self) -> f64 {
        return (self.x * rhs.y) - (self.y * rhs.x);
    }
    pub fn norm2(&self) -> f64 {
        return self.x*self.x + self.y*self.y;
    }
    pub fn norm(&self) -> f64 {
        return ((self.x*self.x + self.y*self.y) as f64).sqrt();
    }
    pub fn orth(&self,eps:f64) -> i64 {
        if self.x.abs() < eps && self.y.abs() < eps { return 0 }
        else if self.x > 0f64 && self.y>=0f64 { return 1 }
        else if self.x <= 0f64 && self.y>0f64 { return 2 }
        else if self.x < 0f64 && self.y<=0f64 { return 3 }
        else { return 4 }
    }
    pub fn cmp_angle(&self, rhs: &Self, eps:f64) -> Ordering {
        if rhs.x.is_nan() || rhs.y.is_nan() {
            panic!("x and y shouldn't be NaN");
        }
        return self.partial_cmp_angle(rhs,eps).unwrap();
    }
    pub fn partial_cmp_angle(&self, rhs: &Self, eps:f64) -> Option<Ordering> {
        let o1 = self.orth(eps);
        let o2 = rhs.orth(eps);
        if o1 != o2 {
            return Some(o1.cmp(&o2));
        }
        let c = self.cross(*rhs);
        return 0f64.partial_cmp(&c);
    }
    /// Adds the angle and multiplies the length using another vector
    /// as if it is a complex number in complex plane.
    pub fn rotate_and_scale(&self, rhs: Self) -> Self {
        let x = self.x*rhs.x - self.y*rhs.y;
        let y = self.x*rhs.y + self.y*rhs.x;
        return Self { x, y };
    }
}

impl Add for VectorF64 {
    type Output=Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Sub for VectorF64 {
    type Output=Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<f64> for VectorF64 {
    type Output=Self;
    fn mul(self, rhs: f64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Div<f64> for VectorF64 {
    type Output=Self;
    fn div(self, rhs: f64) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

