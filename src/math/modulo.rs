use core::fmt;
use std::ops::{Add,Sub,Mul,Div};

type NumberType = usize;
pub struct ModUsizeFactory (NumberType);
impl ModUsizeFactory {
    pub fn new(modulus:usize) -> Self{
        return Self(modulus);
    }
    pub fn create0(self: &Self) -> ModUsize{
        return self.create(0);
    }
    pub fn create(self: &Self, val: NumberType) -> ModUsize{
        return ModUsize {
            modulus: self.0,
            val: val%self.0,
        };
    }
}

#[derive(Debug,Clone,Copy)]
pub struct ModUsize{
    modulus: NumberType,
    pub val: NumberType,
}
impl ModUsize {
    pub fn sibling(self: &Self, val:usize) -> Self {
        return Self {
            modulus: self.modulus,
            val: val%self.modulus,
        };
    }
    pub fn set_val(self: &mut Self, val: usize) {
        self.val = val %self.modulus;
    }
    pub fn add_by(self: &mut Self, rhs: NumberType) {
        self.val = self.add_premitive(self.val, rhs%self.modulus);
    }
    pub fn sub_by(self: &mut Self, rhs: NumberType) {
        self.val = self.sub_premitive(self.val, rhs%self.modulus);
    }
    pub fn mul_by(self: &mut Self, rhs: NumberType) {
        self.val = self.mul_premitive(self.val, rhs%self.modulus);
    }
    pub fn div_by(self: &mut Self, rhs: NumberType) {
        self.val = self.div_premitive(self.val, rhs%self.modulus);
    }
    pub fn pow(self: Self, mut power: NumberType) -> Self{
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
    fn add_premitive(self: &Self, mut lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
        lhs += rhs;
        if lhs >= self.modulus {
            lhs -= self.modulus;
        }
        return lhs;
    }
    fn sub_premitive(self: &Self, mut lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
        if lhs < rhs {
            lhs += self.modulus - rhs;
        } else {
            lhs -= rhs;
        }
        return lhs;
    }
    fn mul_premitive(self: &Self, lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
        return (lhs * rhs) % self.modulus;
    }
    fn div_premitive(self: &Self, mut lhs: NumberType, rhs: NumberType) -> NumberType{ // lhs and rhs should not be greater than modulus.
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
impl Add for ModUsize {
    type Output = Self;
    fn add(mut self: Self, rhs: Self) -> Self {
        self.val = self.add_premitive(self.val, rhs.val);
        return self;
    }
}
impl Sub for ModUsize {
    type Output = Self;
    fn sub(mut self: Self, rhs: Self) -> Self {
        self.val = self.sub_premitive(self.val, rhs.val);
        return self;
    }
}
impl Mul for ModUsize {
    type Output = Self;
    fn mul(mut self: Self, rhs: Self) -> Self {
        self.val = self.mul_premitive(self.val, rhs.val);
        return self;
    }
}
impl Div for ModUsize {
    type Output = Self;
    fn div(mut self: Self, rhs: Self) -> Self {
        self.val = self.div_premitive(self.val, rhs.val);
        return self;
    }
}
impl fmt::Display for ModUsize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",self.val);
    }
}
