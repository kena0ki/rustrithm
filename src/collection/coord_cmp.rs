use std::collections::{BTreeMap,BTreeSet};
pub fn coord_cmp<T:Ord+Clone+Copy>(a: &Vec<T>)
    -> (Vec<usize>, BTreeMap<T,usize>, BTreeMap<usize,T>, usize) {
    let mut set = BTreeSet::<T>::new();
    for i in 0..a.len() {
        set.insert(a[i]);
    }
    let mut size = 0;
    let mut to_id = BTreeMap::<T,usize>::new();
    let mut to_val = BTreeMap::<usize,T>::new();
    for val in set {
        to_id.insert(val, size);
        to_val.insert(size, val);
        size+=1;
    }
    let mut compressed = vec![0; a.len()];
    for i in 0..a.len() {
        compressed[i] = *to_id.get(&a[i]).unwrap();
    }
    return (compressed, to_id, to_val, size);
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
