use std::collections::BTreeSet;
use std::collections::btree_set::{Iter, Range};
use std::ops::Bound::Included;
use std::ops::RangeBounds;


pub struct MultiSet<T:Ord+Copy> {
    s: BTreeSet<(T,usize)>
}

impl <T:Ord+Copy> MultiSet<T> {
    pub fn new() -> Self {
        return Self{s: BTreeSet::<(T,usize)>::new()};
    }
    pub fn insert(&mut self, val: T) {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::MAX))));
        if let Some(&v) = r.last() {
            self.s.insert((val,v.1 +1));
        } else {
            self.s.insert((val,0));
        }
    }
    pub fn remove_one(&mut self, val: T) -> bool {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::MAX))));
        if let Some(&v) = r.last() {
            return self.s.remove(&v);
        }
        return false;
    }
    pub fn remove_all(&mut self, val: T) -> usize {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::MAX))));
        let len = self.s.len();
        let vec = r.copied().collect::<Vec<_>>();
        for v in &vec {
            self.s.remove(v);
        }
        return len - self.s.len();
    }
    pub fn get(&self, val: T) -> Option<T> {
        if let Some(v) = self.s.get(&(val,0)) {
            return Some(v.0);
        }
        return None;
    }
    pub fn count(&self, val: T) -> usize {
        let mut r = self.s.range((Included(&(val,0)),Included(&(val,usize::MAX))));
        if let Some(&first) = r.next() {
            if let Some(&last) = r.last() {
                return last.1 - first.1 + 1;
            }
            return 1;
        }
        return 0;
    }
    pub fn contains(&self, val: T) -> bool {
        return self.get(val).is_some();
    }
    pub fn len(&self) -> usize {
        return self.s.len();
    }
    pub fn is_empty(&self) -> bool {
        return self.s.is_empty();
    }
    pub fn iter(&self) -> Iter<'_, (T,usize)> {
        return self.s.iter();
    }

    pub fn range<R>(&self, range: R) -> Range<'_, (T,usize)>
    where
        R: RangeBounds<(T,usize)>,
    {
        return self.s.range(range);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn multiset_test() {
        let mut ms = MultiSet::new();
        ms.insert(4);
        ms.insert(3);
        ms.insert(1);
        ms.insert(3);
        ms.remove_one(3);
        ms.insert(3);
        let mut it = ms.iter();
        assert_eq!(Some(&(1,0)),it.next());
        assert_eq!(Some(&(3,0)),it.next());
        assert_eq!(Some(&(3,1)),it.next());
        assert_eq!(Some(&(4,0)),it.next());
        assert_eq!(None,it.next());
        let mut rg = ms.range((3,0)..(5,0));
        assert_eq!(Some(&(3,0)),rg.next());
        assert_eq!(Some(&(3,1)),rg.next());
        assert_eq!(Some(&(4,0)),rg.next());
        assert_eq!(None,rg.next());
        let mut rg = ms.range((1,0)..(3,1));
        assert_eq!(Some(&(1,0)),rg.next());
        assert_eq!(Some(&(3,0)),rg.next());
        assert_eq!(None,rg.next());

        assert_eq!(2,ms.count(3));
        assert_eq!(Some(3),ms.get(3));
        ms.remove_one(3);
        assert_eq!(1,ms.count(3));
        assert_eq!(Some(3),ms.get(3));
        ms.remove_one(3);
        assert_eq!(0,ms.count(3));
        assert_eq!(None,ms.get(3));

        ms.insert(4);
        assert_eq!(2, ms.remove_all(4));
    }
}
