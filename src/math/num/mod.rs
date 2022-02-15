//! Rational and Complex numbers, safe modular arithmetic, and linear algebra,
//! implemented minimally for contest use.
//! If you need more features, you might be interested in crates.io/crates/num

mod rational;
mod complex;
mod matrix;
mod modulo;
pub use rational::*;
pub use complex::*;
pub use matrix::*;
pub use modulo::*;

pub use std::f64::consts::PI;
use std::{ops::{Add, Div, Mul, Neg, Sub}, fmt::Debug};

/// Fast iterative version of Euclid's GCD algorithm
pub fn fast_gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    }
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

/// Represents an element of the finite (Galois) field of prime order M, where
/// 1 <= M < 2^31.5. If M is not prime, ring operations are still valid
/// but recip() and division are not. Note that the latter operations are also
/// the slowest, so precompute any inverses that you intend to use frequently.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Modulo<const M: i64> {
    pub val: i64,
}
impl<const M: i64> Modulo<M> {
    /// Computes self^n in O(log n) time
    pub fn pow(mut self, mut n: u64) -> Self {
        let mut result = Self::from_small(1);
        while n > 0 {
            if n % 2 == 1 {
                result = result * self;
            }
            self = self * self;
            n /= 2;
        }
        result
    }
    /// Computes inverses of 1 to n in O(n) time
    pub fn vec_of_recips(n: i64) -> Vec<Self> {
        let mut recips = vec![Self::from(0), Self::from(1)];
        for i in 2..=n {
            let (md, dv) = (M % i, M / i);
            recips.push(recips[md as usize] * Self::from_small(-dv));
        }
        recips
    }
    /// Computes self^-1 in O(log M) time
    pub fn recip(self) -> Self {
        self.pow(M as u64 - 2)
    }
    /// Avoids the % operation but requires -M <= x < M
    fn from_small(s: i64) -> Self {
        let val = if s < 0 { s + M } else { s };
        Self { val }
    }
}
impl<const M: i64> From<i64> for Modulo<M> {
    fn from(val: i64) -> Self {
        // Self { val: val.rem_euclid(M) }
        Self::from_small(val % M)
    }
}
impl<const M: i64> Neg for Modulo<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from_small(-self.val)
    }
}
impl<const M: i64> Add for Modulo<M> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_small(self.val + other.val - M)
    }
}
impl<const M: i64> Sub for Modulo<M> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from_small(self.val - other.val)
    }
}
impl<const M: i64> Mul for Modulo<M> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::from(self.val * other.val)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl<const M: i64> Div for Modulo<M> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}

/// Prime modulus that's commonly used in programming competitions
pub const COMMON_PRIME: i64 = 998_244_353; // 2^23 * 7 * 17 + 1;
pub type CommonField = Modulo<COMMON_PRIME>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_field() {
        let base = CommonField::from(1234);
        let zero = base - base;
        let one = base.recip() * base;
        let two = CommonField::from(2 - 5 * COMMON_PRIME);

        assert_eq!(zero.val, 0);
        assert_eq!(one.val, 1);
        assert_eq!(one + one, two);
        assert_eq!(one / base * (base * base) - base / one, zero);
    }

    #[test]
    fn test_vec_of_recips() {
        let recips = CommonField::vec_of_recips(20);

        assert_eq!(recips.len(), 21);
        for i in 1..recips.len() {
            assert_eq!(recips[i], CommonField::from(i as i64).recip());
        }
    }

}
