use std::collections::{BTreeMap,BTreeSet};
pub fn coord_cmp<T:Ord+Clone+Copy>(a: &Vec<T>) -> (Vec<usize>, usize) {
    let mut set = BTreeSet::<T>::new();
    for i in 0..a.len() {
        set.insert(a[i]);
    }
    let mut size = 0;
    let mut mem = BTreeMap::<T,usize>::new();
    for key in set {
        mem.insert(key, size);
        size+=1;
    }
    let mut ret = vec![0; a.len()];
    for i in 0..a.len() {
        ret[i] = *mem.get(&a[i]).unwrap();
    }
    return (ret, size);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_coord_comp() {
        let mut coords = vec![16, 99, 45, 18];
        let comp = coord_cmp(&mut coords);
        assert_eq!(vec![0,3,2,1], comp.0);
    }
}
