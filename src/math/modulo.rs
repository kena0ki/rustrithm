use core::fmt;
use std::ops::{Add,Sub,Mul,Div};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ModU64<const N:u64>{
    modulus: u64,
    pub val: u64,
}
impl <const N:u64> ModU64<N> {
    pub fn new(val: u64) -> Self {
        return Self{ modulus: N, val: val%N };
    }
    pub fn sibling(self: &Self, val:u64) -> Self {
        return Self {
            modulus: self.modulus,
            val: val%self.modulus,
        };
    }
    pub fn set_val(self: &mut Self, val: u64) {
        self.val = val %self.modulus;
    }
    pub fn add_by(self: &mut Self, rhs: u64) {
        self.val = self.add_premitive(self.val, rhs%self.modulus);
    }
    pub fn sub_by(self: &mut Self, rhs: u64) {
        self.val = self.sub_premitive(self.val, rhs%self.modulus);
    }
    pub fn mul_by(self: &mut Self, rhs: u64) {
        self.val = self.mul_premitive(self.val, rhs%self.modulus);
    }
    pub fn div_by(self: &mut Self, rhs: u64) {
        self.val = self.div_premitive(self.val, rhs%self.modulus);
    }
    pub fn pow(self: Self, mut power: u64) -> Self{
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
    pub fn inv(self: Self) -> Self {
        return self.pow(self.modulus - 2);
    }
    fn add_premitive(self: &Self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        lhs += rhs;
        if lhs >= self.modulus {
            lhs -= self.modulus;
        }
        return lhs;
    }
    fn sub_premitive(self: &Self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        if lhs < rhs {
            lhs += self.modulus - rhs;
        } else {
            lhs -= rhs;
        }
        return lhs;
    }
    fn mul_premitive(self: &Self, lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
        return (lhs * rhs) % self.modulus;
    }
    // a^(-1) ≡ a^(p-2)  (mod p)  where p is a prim
    // https://en.wikipedia.org/wiki/Modular_arithmetic#Properties
    fn div_premitive(self: &Self, mut lhs: u64, rhs: u64) -> u64{ // lhs and rhs should not be greater than modulus.
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
impl <const N:u64> Add for ModU64<N> {
    type Output = Self;
    #[inline]
    fn add(mut self: Self, rhs: Self) -> Self {
        self.val = self.add_premitive(self.val, rhs.val);
        return self;
    }
}
impl <const N:u64> Sub for ModU64<N> {
    type Output = Self;
    #[inline]
    fn sub(mut self: Self, rhs: Self) -> Self {
        self.val = self.sub_premitive(self.val, rhs.val);
        return self;
    }
}
impl <const N:u64> Mul for ModU64<N> {
    type Output = Self;
    #[inline]
    fn mul(mut self: Self, rhs: Self) -> Self {
        self.val = self.mul_premitive(self.val, rhs.val);
        return self;
    }
}
impl <const N:u64> Div for ModU64<N> {
    type Output = Self;
    #[inline]
    fn div(mut self: Self, rhs: Self) -> Self {
        self.val = self.div_premitive(self.val, rhs.val);
        return self;
    }
}
impl <const N:u64> fmt::Display for ModU64<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
}


// https://stackoverflow.com/questions/38811387/how-to-implement-idiomatic-operator-overloading-for-values-and-references-in-rus/38815035#38815035
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident, $u:ident) => {
        impl<'a, const N:u64> $imp<$u<N>> for &'a $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: $u<N>) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(*self, other)
            }
        }
        impl<'a, const N:u64> $imp<&'a $u<N>> for $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $u<N>) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(self, *other)
            }
        }
        impl<'a, 'b, const N:u64> $imp<&'a $u<N>> for &'b $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $u<N>) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

forward_ref_binop! {impl Add, add for ModU64, ModU64}
forward_ref_binop! {impl Sub, sub for ModU64, ModU64}
forward_ref_binop! {impl Mul, mul for ModU64, ModU64}
forward_ref_binop! {impl Div, div for ModU64, ModU64}


#[cfg(test)]
mod test {
    use super::*;
    const MODULUS:u64 = 5;
    #[test]
    fn md_test() {
        let m1 = ModU64::<MODULUS>::new(2);
        let m2 = ModU64::<MODULUS>::new(4);

        assert_eq!(ModU64::<MODULUS>::new(1), m1+m2);
        assert_eq!(ModU64::<MODULUS>::new(1), m1+&m2);
        assert_eq!(ModU64::<MODULUS>::new(1), &m1+m2);
        assert_eq!(ModU64::<MODULUS>::new(1), &m1+&m2);

        assert_eq!(ModU64::<MODULUS>::new(3), &m1-&m2);
        assert_eq!(ModU64::<MODULUS>::new(1), &m2*&m2);
        assert_eq!(ModU64::<MODULUS>::new(2), &m2/&m1);
        assert_eq!(ModU64::<MODULUS>::new(2), &m1/&m2*m2);
    }
}
