use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct FenwickTree<T>{
    identity: T,
    n: usize,
    bit: Vec<T>,
}
impl <T:Clone+Copy+Add<Output=T>+Sub<Output=T>> FenwickTree<T>{
    pub fn new(n: usize, identity: T) -> FenwickTree<T> {
        return Self {
            identity,
            n,
            bit: vec![identity; n+1],
        };
    }
    /// Adds the value to the given index.
    pub fn add(&mut self, mut idx: usize,a: T){
        if idx >= self.n {
            panic!("Index out of bound. length:{}, but idx:{}.", self.n, idx);
        }
        idx+=1;
        loop {
            if idx > self.n {
                break;
            }
            self.bit[idx] = self.bit[idx]+a;
            let idx64 = idx as i64;
            idx+=(idx64 & -idx64) as usize;
        }
    }
    /// Returns the summary of values between l and r-1.
    pub fn sum(&self, l:usize, r:usize) -> T {
        if l>r {
            panic!("Invalid range. l:{} > r:{}", l, r);
        }
        return self.sum0(r) - self.sum0(l);
    }
    fn sum0(&self, mut idx: usize) -> T {
        //idx+=1;
        let mut ret = self.identity;
        loop {
            if idx<=0 {
                break;
            }
            ret = ret+self.bit[idx];
            let idx64 = idx as i64;
            idx-=(idx64 & -idx64) as usize;
        }
        return ret;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fenwck() {
        let mut bit = FenwickTree::new(10,0);
        bit.add(4,3);
        bit.add(5,3);
        bit.add(6,3);
        assert_eq!(9,bit.sum(4,7));
        bit.add(5,4);
        assert_eq!(13,bit.sum(3,7));
        bit.add(7,-4);
        assert_eq!(9,bit.sum(4,10));
        bit.add(9,-1);
        assert_eq!(-1,bit.sum(9,10));
        assert_eq!(8,bit.sum(0,10));
        bit.add(0,2);
        assert_eq!(10,bit.sum(0,10));
        assert_eq!(2,bit.sum(0,1));
        assert_eq!(0,bit.sum(1,2));
        assert_eq!(0,bit.sum(0,0));
    }
}
