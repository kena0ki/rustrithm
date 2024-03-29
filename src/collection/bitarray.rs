//! Implementation of a bit array.
//! This can be thought of as analogous to C++ bitset.
//!
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, ShrAssign, Shr};


#[derive(Debug,Clone)]
pub struct BitArray {
    bits: Vec<u128>,
    num_bits: usize,
    arr_size: usize,
}

impl BitArray {
    pub const BITS_PER_UNIT:usize = u128::BITS as usize;

    /// Initializes a bit array.
    pub fn new(size: usize) -> Self {
        let arr_size = size/Self::BITS_PER_UNIT + 1;
        return Self{
            bits: vec![0;arr_size],
            num_bits: size,
            arr_size,
        };
    }

    /// Gets the length of bits.
    pub fn len(&self) -> usize {
        return self.num_bits;
    }

    /// Sets the specified bit to true. Index is zero-based.
    pub fn set(&mut self, at: usize) {
        self.panic_if_out_of_range(at+1);
        self.bits[at/Self::BITS_PER_UNIT] |= 1<<(at%Self::BITS_PER_UNIT);
    }

    /// Unsets the specified bit to false. Index is zero-based.
    pub fn unset(&mut self, at: usize) {
        self.panic_if_out_of_range(at+1);
        self.bits[at/Self::BITS_PER_UNIT] ^= 1<<(at%Self::BITS_PER_UNIT);
    }

    /// Sets the bits in the range from the offset to the offset + 128 using the u128 number. Index is zero-based.
    pub fn set_bits_with_u128(&mut self, num: u128, offset: usize) {
        self.panic_if_out_of_range(offset + Self::BITS_PER_UNIT);
        let q = offset / Self::BITS_PER_UNIT;
        let m = offset % Self::BITS_PER_UNIT;
        let s = Self::BITS_PER_UNIT - m;
        if m == 0 {
            self.bits[q] = num;
        } else {
            self.bits[q] = (self.bits[q] >> s) << s;
            self.bits[q] |= num << m;
            self.bits[q+1] = (self.bits[q+1] << m) >> m;
            self.bits[q+1] |= num >> s;
        }
    }

    /// Tests whether the specified bit is true.
    pub fn test(&self, at: usize) -> bool {
        self.panic_if_out_of_range(at+1);
        return self.bits[at/Self::BITS_PER_UNIT] & (1<<(at%Self::BITS_PER_UNIT)) > 0;
    }

    /// Counts the number of ones.
    pub fn count_ones(&self) -> usize {
        return self.bits.iter().fold(0,|a,b|a+b.count_ones() as usize);
    }

    /// Counts the number of zeros.
    pub fn count_zeros(&self) -> usize {
        return self.bits.len() - self.bits.iter().fold(0,|a,b|a+b.count_ones() as usize);
    }

    fn panic_if_out_of_input_range(num_bits: usize, at:usize) {
        if at > num_bits {
            panic!("Index {} out of range: {}.", at, num_bits);
        }
    }

    fn panic_if_out_of_range(&self, at:usize) {
        Self::panic_if_out_of_input_range(self.num_bits, at);
    }

    /// Converts the bit array to a binary representative string.
    /// Note: The direction of the binary string is opposite from the direction of array.
    ///        e.g.) [true,false,true,true] -> 1101
    ///       Therefore the result may be confusing especially when you initialized the struct from
    ///       an array.
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.num_bits);
        let b = self.bits[self.arr_size-1];
        let sub = Self::BITS_PER_UNIT - self.num_bits%Self::BITS_PER_UNIT;
        s.push_str(&format!("{:0128b}", b).as_str()[sub..]);
        for b in self.bits.iter().rev().skip(1) {
            s.push_str(format!("{:0128b}", b).as_str());
        }
        return s;
    }
}

impl BitAnd for &BitArray {
    type Output = BitArray;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        for i in 0..self.arr_size.min(rhs.arr_size) {
            new.bits[i] = self.bits[i] & rhs.bits[i];
        }
        return new;
    }
}
impl BitAndAssign<&Self> for BitArray {
    fn bitand_assign(&mut self, rhs: &Self) {
        let new = (&*self).bitand(&rhs);
        self.bits = new.bits;
    }
}
impl BitOr for &BitArray {
    type Output = BitArray;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        for i in 0..self.arr_size.min(rhs.arr_size) {
            new.bits[i] = self.bits[i] | rhs.bits[i];
        }
        return new;
    }
}
impl BitOrAssign<&Self> for BitArray {
    fn bitor_assign(&mut self, rhs: &Self) {
        let new = (&*self).bitor(&rhs);
        self.bits = new.bits;
    }
}
impl BitXor for &BitArray {
    type Output = BitArray;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new = BitArray::new(self.num_bits);
        for i in 0..self.arr_size.min(rhs.arr_size) {
            new.bits[i] = self.bits[i] ^ rhs.bits[i];
        }
        return new;
    }
}
impl BitXorAssign<&Self> for BitArray {
    fn bitxor_assign(&mut self, rhs: &Self) {
        let new = (&*self).bitxor(&rhs);
        self.bits = new.bits;
    }
}

impl Shl<usize> for &BitArray {
    type Output = BitArray;
    fn shl(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self.clone();
        }

        let mut new = BitArray::new(self.num_bits);
        let shift = rhs / Self::Output::BITS_PER_UNIT;
        let offset = rhs % Self::Output::BITS_PER_UNIT;
        let sub_offset = Self::Output::BITS_PER_UNIT - offset;

        if shift>=self.arr_size {
            return new;
        }

        if offset == 0 {
            for i in (shift..self.arr_size).rev() {
                new.bits[i] = self.bits[i - shift];
            }
        } else {
            for i in (shift+1..self.arr_size).rev() {
                new.bits[i] = (self.bits[i - shift] << offset)
                     | (self.bits[i - shift - 1] >> sub_offset);
            }
            new.bits[shift] = self.bits[0] << offset;
        }

        //new.bits[0..shift].fill(0);
        let unused_range = Self::Output::BITS_PER_UNIT - self.num_bits%Self::Output::BITS_PER_UNIT;
        new.bits[self.arr_size-1] &= !0 >> unused_range;

        return new;
    }
}

impl Shr<usize> for &BitArray {
    type Output = BitArray;
    fn shr(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self.clone();
        }

        let mut new = BitArray::new(self.num_bits);
        let shift = rhs / Self::Output::BITS_PER_UNIT;
        let offset = rhs % Self::Output::BITS_PER_UNIT;
        let sub_offset = Self::Output::BITS_PER_UNIT - offset;

        if shift>=self.arr_size {
            return new;
        }

        if offset == 0 {
            for i in shift..self.arr_size {
                new.bits[i-shift] = self.bits[i];
            }
        } else {
            for i in shift..self.arr_size-1 {
                new.bits[i-shift] = (self.bits[i + 1] << sub_offset)
                     | (self.bits[i] >> offset);
            }
            new.bits[self.arr_size-shift-1] = self.bits[self.arr_size-1] >> offset;
        }

        for i in self.arr_size-(shift.max(1))..self.arr_size-1 {
            new.bits[i] = 0;
        }
        //new.bits[self.arr_size-(shift.max(1))..self.arr_size-1].fill(0);


        return new;
    }
}

impl ShlAssign<usize> for BitArray {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        let new = (&*self) << rhs;
        self.bits = new.bits;
    }
}
impl ShrAssign<usize> for BitArray {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        let new = (&*self) >> rhs;
        self.bits = new.bits;
    }
}

impl From<&[bool]> for BitArray {
    fn from(bits: &[bool]) -> Self {
        let mut new = Self::new(bits.len());
        for i in 0..new.arr_size {
            let start = i*Self::BITS_PER_UNIT;
            let end = bits.len().min(start+Self::BITS_PER_UNIT);
            for j in start..end {
                new.bits[i] |= (bits[j] as u128) << (j-start);
            }
        }
        return new;
    }
}
impl From<&Vec<bool>> for BitArray {
    fn from(bits: &Vec<bool>) -> Self {
        return Self::from(&bits[..]);
    }
}
impl <const N:usize> From<&[bool; N]> for BitArray {
    fn from(bits: &[bool; N]) -> Self {
        return Self::from(&bits[..]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn barr_set() {
        let mut ba = BitArray::new(4);
        ba.set(3);
        ba.set(1);
        assert_eq!("1010",ba.to_string());
        ba.unset(3);
        assert_eq!("0010",ba.to_string());
    }

    #[test]
    fn barr_bitor() {
        let mut left = BitArray::new(200);
        left.set_bits_with_u128(!0 - (1<<2) - (1<<80), 0);
        let mut right = BitArray::new(200);
        right.set_bits_with_u128(!0 - (1<<2) - (1<<80), 60);
        left |= &right;
        let expected = "00000000000011111111111111111111111111111111111111111111111011111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111011";
        assert_eq!(expected, left.to_string());
    }

    #[test]
    fn barr_bitand() {
        let mut left = BitArray::new(200);
        left.set_bits_with_u128(!0 - (1<<2) - (1<<80), 30);
        let mut right = BitArray::new(200);
        right.set_bits_with_u128(!0 - (1<<2) - (1<<80), 60);
        left &= &right;
        let expected = "00000000000000000000000000000000000000000011111111111111111011111111111111111111111111111011111111111111111111111111111111111111111111111011000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(expected, left.to_string());
    }

    #[test]
    fn barr_bitxor() {
        let mut left = BitArray::new(200);
        left.set_bits_with_u128(!0 - (1<<2) - (1<<80), 30);
        let mut right = BitArray::new(200);
        right.set_bits_with_u128(!0 - (1<<2) - (1<<80), 60);
        left ^= &right;
        let expected = "00000000000011111111111111111111111111111100000000000000000100000000000000000000000000000100000000000000000000000000000000000000000000000100111111111111111111111111111011000000000000000000000000000000";
        assert_eq!(expected, left.to_string());
    }

    #[test]
    fn barr_shift_left() {
        let mut barr = BitArray::new(200);
        barr.set_bits_with_u128(!0 - (1<<2) - (1<<80), 10);
        barr = &barr << 100;
        let expected = "11111111101111111111111111111111111111111111111111111111111111111111111111111111111111101100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_shift_left_assign() {
        let mut barr = BitArray::new(200);
        barr.set_bits_with_u128(!0 - (1<<2) - (1<<80), 10);
        barr <<= 50;
        barr <<= 0;
        let expected = "00000000000011111111111111111111111111111111111111111111111011111111111111111111111111111111111111111111111111111111111111111111111111111011000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_shift_right() {
        let mut barr = BitArray::new(200);
        barr.set_bits_with_u128(!0 - (1<<2) - (1<<80), 72);
        barr = &barr >> 100;
        let expected = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001111111111111111111111111111111111111111111111101111111111111111111111111111111111111111111111111111";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_shift_right_assign() {
        let mut barr = BitArray::new(200);
        barr.set_bits_with_u128(!0 - (1<<2) - (1<<80), 72);
        barr >>= 50;
        barr >>= 0;
        let expected = "00000000000000000000000000000000000000000000000000111111111111111111111111111111111111111111111110111111111111111111111111111111111111111111111111111111111111111111111111111110110000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }

    #[test]
    fn barr_from_u8slice() {
        let mut barr = BitArray::from(&[false;200]);
        barr.set_bits_with_u128(!0 - (1<<2) - (1<<80), 60);
        let expected = "00000000000011111111111111111111111111111111111111111111111011111111111111111111111111111111111111111111111111111111111111111111111111111011000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(expected, barr.to_string());
    }
}

