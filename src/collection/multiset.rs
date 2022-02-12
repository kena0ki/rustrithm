use std::collections::BTreeSet;
use std::collections::btree_set::{Iter, Range};
use std::iter::Map;
use std::ops::Bound::{Included, self,Excluded};
use std::ops::RangeBounds;


#[derive(Debug,Clone)]
pub struct MultiSet<T:Ord+Copy> {
    s: BTreeSet<(T,usize)>
}

impl <T:Ord+Copy> MultiSet<T> {
    pub fn new() -> Self {
        return Self{s: BTreeSet::<(T,usize)>::new()};
    }
    pub fn insert(&mut self, val: T) {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::MAX))));
        if let Some(&v) = r.rev().next() {
            self.s.insert((val,v.1 +1));
        } else {
            self.s.insert((val,0));
        }
    }
    pub fn remove_one(&mut self, val: T) -> bool {
        let r = self.s.range((Included(&(val,0)),Included(&(val,usize::MAX))));
        if let Some(&v) = r.rev().next() {
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
            if let Some(&last) = r.rev().next() {
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
    pub fn last(&self) -> Option<T> {
        if let Some(v) = self.s.iter().rev().next() {
            return Some(v.0);
        } else {
            return None;
        }
    }
    pub fn first(&self) -> Option<T> {
        if let Some(v) = self.s.iter().next() {
            return Some(v.0);
        } else {
            return None;
        }
    }
    pub fn iter(&self) -> Map<Iter<'_, (T,usize)>, impl FnMut(&(T,usize)) -> T> {
        return self.s.iter().map(Self::filter);
    }
    fn filter(v: &(T,usize)) -> T{
        return v.0;
    }

    // this method is slow for some reason.
    pub fn range<R>(&self, range: R) -> Map<Range<'_, (T,usize)>, impl FnMut(&(T,usize)) -> T>
    where
        R: RangeBounds<T>,
    {
        let start = match range.start_bound() {
            Bound::Unbounded => Bound::Unbounded,
            Included(&b) => Included((b,0)),
            Excluded(&b) => Excluded((b,usize::MAX)),
        };
        let end = match range.end_bound() {
            Bound::Unbounded => Bound::Unbounded,
            Included(&b) => Included((b,usize::MAX)),
            Excluded(&b) => Excluded((b,0)),
        };
        return self.s.range((start,end)).map(Self::filter);
    }
    pub fn multiset(&self) -> &BTreeSet<(T,usize)> {
        return &self.s;
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
        assert_eq!(1,ms.first().unwrap());
        assert_eq!(4,ms.last().unwrap());
        let mut it = ms.iter();
        assert_eq!(Some(1),it.next());
        assert_eq!(Some(3),it.next());
        assert_eq!(Some(3),it.next());
        assert_eq!(Some(4),it.next());
        assert_eq!(None,it.next());
        let mut rg = ms.range(3..5);
        assert_eq!(Some(3),rg.next());
        assert_eq!(Some(3),rg.next());
        assert_eq!(Some(4),rg.next());
        assert_eq!(None,rg.next());
        let mut rg = ms.range(1..3);
        assert_eq!(Some(1),rg.next());
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
