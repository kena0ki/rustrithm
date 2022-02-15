
use std::{ops::{Add, Div, Mul, Neg, Sub}, fmt::Debug};

/// Represents a complex number using floating-point arithmetic
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}
impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    pub fn from_polar(r: f64, th: f64) -> Self {
        Self::new(r * th.cos(), r * th.sin())
    }
    pub fn abs_square(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    pub fn argument(self) -> f64 {
        self.imag.atan2(self.real)
    }
    pub fn conjugate(self) -> Self {
        Self::new(self.real, -self.imag)
    }
    pub fn recip(self) -> Self {
        let denom = self.abs_square();
        Self::new(self.real / denom, -self.imag / denom)
    }
}
impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Self::new(real, 0.0)
    }
}
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.real, -self.imag)
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imag + other.imag)
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.real - other.real, self.imag - other.imag)
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.imag * other.real + self.real * other.imag;
        Self::new(real, imag)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_complex() {
        let four = Complex::new(4.0, 0.0);
        let two_i = Complex::new(0.0, 2.0);

        assert_eq!(four / two_i, -two_i);
        assert_eq!(two_i * -two_i, four);
        assert_eq!(two_i - two_i, Complex::from(0.0));
        assert_eq!(four.abs_square(), 16.0);
        assert_eq!(two_i.abs_square(), 4.0);
        assert_eq!((-four).argument(), -PI);
        assert_eq!((-two_i).argument(), -PI / 2.0);
        assert_eq!(four.argument(), 0.0);
        assert_eq!(two_i.argument(), PI / 2.0);
    }
}
