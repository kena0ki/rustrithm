use std::{collections::BinaryHeap, cmp::Reverse};

//https://atcoder.jp/contests/abc127/submissions/32149254
pub struct Median<T> {
    left: BinaryHeap<T>,
    right: BinaryHeap<Reverse<T>>,
}
impl <T:Ord+Copy> Median<T> {
    pub fn new() -> Self {
        return Self { left: BinaryHeap::new(), right: BinaryHeap::new() };
    }
    pub fn push(&mut self, val: T) {
        let l = self.left.peek();
        if l.is_none() {
            self.left.push(val);
            return;
        }
        let l = l.copied().unwrap();
        if val<l {
            self.left.push(val);
        } else {
            self.right.push(Reverse(val));
        }
        let len_l = self.left.len();
        let len_r = self.right.len();
        if len_l < len_r {
            let Reverse(r) = self.right.pop().unwrap();
            self.left.push(r);
        } else if len_l - len_r >= 2 {
            let l = self.left.pop().unwrap();
            self.right.push(Reverse(l));
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        let val = self.left.pop();
        let len_l = self.left.len();
        let len_r = self.right.len();
        if len_l - len_r >= 2 {
            let l = self.left.pop().unwrap();
            self.right.push(Reverse(l));
        }
        return val;
    }
    pub fn median(&self) -> Option<(T,T)> {
        if self.left.len() ==0 {
            return None;
        } else if self.left.len() == self.right.len() {
            let l =  self.left.peek().copied();
            let r = self.right.peek().copied().map(|Reverse(v)| v);
            return Some((l.unwrap(),r.unwrap()));
        } else {
            let l =  self.left.peek().copied();
            let l = l.unwrap();
            return Some((l,l));
        }
    }
}

