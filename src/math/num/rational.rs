use std::{ops::{Add, Div, Mul, Neg, Sub}, fmt::{Display, Debug}};

use super::fast_gcd;

/// Represents a fraction reduced to lowest terms
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rational {
    pub num: i64,
    pub den: i64,
}
impl Rational {
    pub fn new(num: i64, den: i64) -> Self {
        if num == 0 && den == 0 {
            panic!("0/0 is illegal");
        }
        let sign = if den < 0 { -1 } else { 1 };
        let g = fast_gcd(num, den) * sign;
        Self {
            num: num / g,
            den: den / g,
        }
    }
    pub fn abs(self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den,
        }
    }
    pub fn recip(self) -> Self {
        let sign = if self.num < 0 { -1 } else { 1 };
        Self {
            num: self.den / sign,
            den: self.num / sign,
        }
    }
}
impl From<i64> for Rational {
    fn from(num: i64) -> Self {
        Self { num, den: 1 }
    }
}
impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            num: -self.num,
            den: self.den,
        }
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.num * other.den + self.den * other.num,
            self.den * other.den,
        )
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.num * other.den - self.den * other.num,
            self.den * other.den,
        )
    }
}
impl Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.num * other.num, self.den * other.den)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.den == 0 && other.den == 0 {
            return self.num.cmp(&other.num);
        }
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.num as f64 / self.den as f64);
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}/{}", self.num, self.den);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rational() {
        let three = Rational::from(3);
        let six = Rational::from(6);
        let three_and_half = three + three / six;

        assert_eq!(three_and_half.num, 7);
        assert_eq!(three_and_half.den, 2);
        assert_eq!(three_and_half, Rational::new(-35, -10));
        assert!(three_and_half > Rational::from(3));
        assert!(three_and_half < Rational::from(4));

        let minus_three_and_half = six - three_and_half + three / (-three / six);
        let zero = three_and_half + minus_three_and_half;

        assert_eq!(minus_three_and_half.num, -7);
        assert_eq!(minus_three_and_half.den, 2);
        assert_eq!(three_and_half, -minus_three_and_half);
        assert_eq!(zero.num, 0);
        assert_eq!(zero.den, 1);

        assert!(Rational::new(1,0) > Rational::new(i64::MAX, 1));
        assert!(Rational::new(1,0) > Rational::new(i64::MIN+1, -1));
        assert!(Rational::new(-1,0) < Rational::new(i64::MIN, 1));
        assert!(Rational::new(-1,0) < Rational::new(i64::MAX, -1));
        assert!(Rational::new(-1,0) < Rational::new(1,0));
    }

    #[test]
    #[should_panic]
    fn test_rational_0_0() {
        Rational::new(0,0);
    }
}
