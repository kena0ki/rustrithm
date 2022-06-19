use std::collections::BTreeMap;
pub fn coord_cmp<T:Ord+Clone+Copy>(va: &Vec<T>) -> (Vec<usize>, BTreeMap<T,usize>, Vec<T>, usize) {
    let mut to_val = va.clone();
    to_val.sort_unstable();
    to_val.dedup();
    let to_id:BTreeMap<T,usize> = to_val.iter().copied().enumerate().map(|(i,v)| (v,i)).collect();
    let size = to_val.len();
    let compressed = va.iter().map(|a| to_id[&a]).collect();
    return (compressed, to_id, to_val, size);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_coord_comp() {
        let mut coords = vec![16, 99, 45, 18, 45];
        let comp = coord_cmp(&mut coords);
        assert_eq!(vec![0,3,2,1,2], comp.0);
    }
}
