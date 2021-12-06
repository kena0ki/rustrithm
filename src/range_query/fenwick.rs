use std::ops::Add;

#[derive(Debug)]
pub struct FenwickTree<T>{
    ini_val: T,
    n: usize,
    bit: Vec<T>,
}
impl <T:Clone+Copy+Add<Output=T>> FenwickTree<T>{
    pub fn new(n: usize, ini_val: T) -> FenwickTree<T> {
        return Self {
            ini_val,
            n,
            bit: vec![ini_val; n+1],
        };
    }
    pub fn addition(x: T, y: T) -> T{
        return x+y;
    }
    pub fn add(self: &mut Self, mut idx: usize,a: T){
        idx+=1;
        loop {
            if idx > self.n {
                break;
            }
            self.bit[idx] = Self::addition(self.bit[idx],a);
            let idx64 = idx as i64;
            idx+=(idx64 & -idx64) as usize;
        }
    }
    pub fn sum(self: &mut Self, mut idx: usize) -> T {
        idx+=1;
        let mut ret = self.ini_val;
        loop {
            if idx<=0 {
                break;
            }
            ret = Self::addition(ret,self.bit[idx]);
            let idx64 = idx as i64;
            idx-=(idx64 & -idx64) as usize;
        }
        return ret;
    }
}
