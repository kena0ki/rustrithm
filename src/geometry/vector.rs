use std::{cmp::Ordering, ops::{Add, Sub, Mul, Div}};


#[derive(Debug,Default,Clone,Copy)]
pub struct Vector {
    pub x:i64,
    pub y:i64,
}

impl Vector {
    pub fn new(x: i64, y:i64) -> Self {
        Self { x, y }
    }
    pub fn dot(&self, rhs: Self) -> i64 {
        return (self.x * rhs.x) + (self.y * rhs.y);
    }
    pub fn cross(&self, rhs: Self) -> i64 {
        return (self.x * rhs.y) - (self.y * rhs.x);
    }
    pub fn norm2(&self) -> i64 {
        return self.x*self.x + self.y*self.y;
    }
    pub fn orth(&self) -> i64 {
        if self.x.abs() == 0 && self.y.abs() == 0 { return 0 }
        else if self.x > 0 && self.y>=0 { return 1 }
        else if self.x <= 0 && self.y>0 { return 2 }
        else if self.x < 0 && self.y<=0 { return 3 }
        else { return 4 }
    }
    pub fn cmp_angle(&self, rhs: &Self) -> Ordering {
        let o1 = self.orth();
        let o2 = rhs.orth();
        if o1 != o2 {
            return o1.cmp(&o2);
        }
        let c = self.cross(*rhs);
        return 0i64.cmp(&c);
    }
    /// Adds the angle and multiplies the length using another vector
    /// as if it is a complex number in complex plane.
    pub fn rotate_and_scale(&self, rhs: Self) -> Self {
        let x = self.x*rhs.x - self.y*rhs.y;
        let y = self.x*rhs.y + self.y*rhs.x;
        return Self { x, y };
    }
}

impl Add for Vector {
    type Output=Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Sub for Vector {
    type Output=Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<i64> for Vector {
    type Output=Self;
    fn mul(self, rhs: i64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Div<i64> for Vector {
    type Output=Self;
    fn div(self, rhs: i64) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

