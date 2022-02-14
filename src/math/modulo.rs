//! Helper for modulo calculation.

use std::ops::{Add,Sub,Mul,Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

pub const MOD998244353:u64 = 998244353;
pub const MOD1000000007:u64 = 1000000007;

pub const ZERO_MOD998244353:ModU64<MOD998244353> = ModU64::<MOD998244353>::new(0);
pub const ZERO_MOD1000000007:ModU64<MOD1000000007> = ModU64::<MOD1000000007>::new(0);

/// Represents a mod N number.
///
/// # Example
/// ```
/// use rustrithm::math::modulo::ModU64;
/// // modulus = 5
/// let m1 = ModU64::<5>::new(2);
/// let m2 = m1.sibling(4);
///
/// assert_eq!(m1.sibling(1), m1+m2);
/// assert_eq!(m1.sibling(3), 9*m1);
/// assert_eq!(m1.sibling(4), ModU64::<5>::from_i64(-1));
///
/// ```
#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct ModU64<const N:u64>{
    modulus: u64,
    val: u64,
}
impl <const N:u64> ModU64<N> {
    /// Creates a new instance.
    pub const fn new(val: u64) -> Self {
        return Self{ modulus: N, val: val%N };
    }
    /// Creates a new instance from i64.
    /// The result value is guaranteed to be positive by adding the modulus if the given value is negative.
    pub const fn from_i64(val: i64) -> Self {
        let val = val%N as i64;
        let val = if val < 0 { val + N as i64 } else { val };
        return Self { val: val as u64, modulus: N };
    }
    /// Creates a new instance using the same modulus of the current instance.
    pub const fn sibling(self: &Self, val:u64) -> Self {
        return Self {
            modulus: self.modulus,
            val: val%self.modulus,
        };
    }
    /// Gets the underlying value as u64.
    pub const fn val(&self) -> u64 {
        return self.val;
    }
    /// Gets the power of this value.
    pub const fn pow(&self, mut power: u64) -> Self{
        let mut square = self.val;
        let mut ret = 1;
        while 0 < power {
            if (power & 1) == 1{
                ret *= square;
                ret %= self.modulus;
            }
            square *= square;
            square %= self.modulus;
            power >>= 1;
        }
        return Self {
            val:ret,
            modulus: self.modulus,
        };
    }
    /// Gets the inverse of this value.
    pub const fn inv(&self) -> Self {
        return self.pow(self.modulus - 2);
    }
    const fn add_u64(&self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        lhs += rhs;
        if lhs >= self.modulus {
            lhs -= self.modulus;
        }
        return lhs;
    }
    const fn sub_u64(&self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        if lhs < rhs {
            lhs += self.modulus - rhs;
        } else {
            lhs -= rhs;
        }
        return lhs;
    }
    const fn mul_u64(&self, lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        return (lhs * rhs) % self.modulus;
    }
    // a^(-1) ≡ a^(p-2)  (mod p)  where p is prime
    // https://en.wikipedia.org/wiki/Modular_arithmetic#Properties
    const fn div_u64(&self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        let mut power = self.modulus - 2;
        let mut square = rhs;
        while 0 < power {
            if (power & 1) == 1{
                lhs *= square;
                lhs %= self.modulus;
            }
            square *= square;
            square %= self.modulus;
            power >>= 1;
        }
        return lhs;
    }
}

impl <const N:u64> From<ModU64<N>> for u64 {
    fn from(mu: ModU64<N>) -> Self {
        return mu.val;
    }
}

impl <const N:u64> fmt::Display for ModU64<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
}

impl <const N:u64> fmt::Debug for ModU64<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
}

impl <const N:u64> Default for ModU64<N> {
    fn default() -> Self {
        return Self { modulus: N, val:0 };
    }
}

macro_rules! assign_binop {
    (impl $imp:ident, $method:ident for $t:ident, $internal_method:ident) => {
        impl <const N:u64> $imp for $t<N> {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                self.val = self.$internal_method(self.val, rhs.val);
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ident, $u:ty, $internal_method:ident) => {
        impl <const N:u64> $imp<$u> for $t<N> {
            #[inline]
            fn $method(&mut self, rhs: $u) {
                self.val = self.$internal_method(self.val, rhs%self.modulus);
            }
        }
    };
}

assign_binop!(impl AddAssign, add_assign for ModU64, add_u64);
assign_binop!(impl SubAssign, sub_assign for ModU64, sub_u64);
assign_binop!(impl MulAssign, mul_assign for ModU64, mul_u64);
assign_binop!(impl DivAssign, div_assign for ModU64, div_u64);
assign_binop!(impl AddAssign, add_assign for ModU64, u64, add_u64);
assign_binop!(impl SubAssign, sub_assign for ModU64, u64, sub_u64);
assign_binop!(impl MulAssign, mul_assign for ModU64, u64, mul_u64);
assign_binop!(impl DivAssign, div_assign for ModU64, u64, div_u64);

macro_rules! binop {
    (impl $imp:ident, $method:ident for $t:ident, $internal_method:ident) => {
        impl <const N:u64> $imp for $t<N> {
            type Output = Self;
            #[inline]
            fn $method(mut self: Self, rhs: Self) -> Self {
                self.val = self.$internal_method(self.val, rhs.val);
                return self;
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ident, $u:ty, $internal_method:ident) => {
        impl <const N:u64> $imp<$u> for $t<N> {
            type Output = Self;
            #[inline]
            fn $method(mut self: Self, rhs: $u) -> Self {
                self.val = self.$internal_method(self.val, rhs%self.modulus);
                return self;
            }
        }
        impl <const N:u64> $imp<$t<N>> for $u {
            type Output = $t<N>;
            #[inline]
            fn $method(self: Self, mut rhs: $t<N>) -> $t<N> {
                rhs.val = rhs.$internal_method(self%rhs.modulus, rhs.val);
                return rhs;
            }
        }
    };
}
binop!(impl Add, add for ModU64, add_u64);
binop!(impl Sub, sub for ModU64, sub_u64);
binop!(impl Mul, mul for ModU64, mul_u64);
binop!(impl Div, div for ModU64, div_u64);
binop!(impl Add, add for ModU64, u64, add_u64);
binop!(impl Sub, sub for ModU64, u64, sub_u64);
binop!(impl Mul, mul for ModU64, u64, mul_u64);
binop!(impl Div, div for ModU64, u64, div_u64);


// https://stackoverflow.com/questions/38811387/how-to-implement-idiomatic-operator-overloading-for-values-and-references-in-rus/38815035#38815035
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident) => {
        impl<'a, const N:u64> $imp<$t<N>> for &'a $t<N> {
            type Output = <$t<N> as $imp<$t<N>>>::Output;

            #[inline]
            fn $method(self, other: $t<N>) -> <$t<N> as $imp<$t<N>>>::Output {
                $imp::$method(*self, other)
            }
        }
        impl<'a, const N:u64> $imp<&'a $t<N>> for $t<N> {
            type Output = <$t<N> as $imp<$t<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $t<N>) -> <$t<N> as $imp<$t<N>>>::Output {
                $imp::$method(self, *other)
            }
        }
        impl<'a, 'b, const N:u64> $imp<&'a $t<N>> for &'b $t<N> {
            type Output = <$t<N> as $imp<$t<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $t<N>) -> <$t<N> as $imp<$t<N>>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
    (impl $imp:ident, $method:ident for $t:ident, $u:ty) => {
        impl<'a, const N:u64> $imp<$u> for &'a $t<N> {
            type Output = <$t<N> as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t<N> as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }
        impl<'a, const N:u64> $imp<&'a $u> for $t<N> {
            type Output = <$t<N> as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t<N> as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }
        impl<'a, 'b, const N:u64> $imp<&'a $u> for &'b $t<N> {
            type Output = <$t<N> as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t<N> as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

forward_ref_binop! {impl Add, add for ModU64}
forward_ref_binop! {impl Sub, sub for ModU64}
forward_ref_binop! {impl Mul, mul for ModU64}
forward_ref_binop! {impl Div, div for ModU64}
forward_ref_binop! {impl Add, add for ModU64, u64}
forward_ref_binop! {impl Sub, sub for ModU64, u64}
forward_ref_binop! {impl Mul, mul for ModU64, u64}
forward_ref_binop! {impl Div, div for ModU64, u64}


#[cfg(test)]
mod test {
    use super::*;
    const MODULUS:u64 = 5;
    #[test]
    fn md_test() {
        let m1 = ModU64::<MODULUS>::new(2);
        let m2 = m1.sibling(4);

        assert_eq!(ModU64::<MODULUS>::new(1), m1+m2);
        assert_eq!(ModU64::<MODULUS>::new(1), m1+&m2);
        assert_eq!(ModU64::<MODULUS>::new(1), &m1+m2);
        assert_eq!(ModU64::<MODULUS>::new(1), &m1+&m2);

        assert_eq!(ModU64::<MODULUS>::new(1), 9+m1);

        assert_eq!(ModU64::<MODULUS>::new(3), &m1-&m2);
        assert_eq!(ModU64::<MODULUS>::new(1), &m2*&m2);
        assert_eq!(ModU64::<MODULUS>::new(2), &m2/&m1);
        assert_eq!(ModU64::<MODULUS>::new(2), &m1/&m2*m2);
    }
}
