use std::{collections::BinaryHeap, cmp::Reverse};

//https://atcoder.jp/contests/abc127/submissions/32149254
/// `Median` is a data structure that can be used to find the median of a stream of numbers.
pub struct Median<T> {
    left: BinaryHeap<T>,
    right: BinaryHeap<Reverse<T>>,
}
impl <T:Ord+Copy> Median<T> {
    /// Creates a new `Median` structure.
    pub fn new() -> Self {
        return Self { left: BinaryHeap::new(), right: BinaryHeap::new() };
    }
    /// Adds a new number to the `Median` structure.
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
    /// Returns the median of the numbers that have been pushed to the `Median` structure.
    pub fn median(&self) -> Option<(T,T)> {
        if self.left.len() == 0 {
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

#[cfg(test)]
mod test {
    #[test]
    fn test_median() {
        let mut m = super::Median::new();
        m.push(1);
        m.push(3);
        m.push(5);
        let med = m.median();
        assert_eq!(med, Some((3,3)));
        m.push(6);
        let med = m.median();
        assert_eq!(med, Some((3,5)));
    }

}
