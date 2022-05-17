
pub struct Permutation<T> {
    items: Vec<T>,
    swaps: Vec<usize>,
    i: usize,
}

impl <T:Clone> Permutation<T> {
    pub fn new(items: Vec<T>) -> Permutation<T> {
        let swaps = vec![0; items.len()];
        Permutation { items, swaps, i: 0 }
    }
}

impl <T:Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.items.clone());
        }
        loop {
            if self.i >= self.items.len() {
                return None;
            }
            if self.swaps[self.i] < self.i {
                break;
            }
            self.swaps[self.i] = 0;
            self.i += 1;
        }
        self.items.swap(self.i, (self.i & 1) * self.swaps[self.i]);
        self.swaps[self.i] += 1;
        self.i = 1;
        return Some(self.items.clone());
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_permutation() {
        let p = Permutation::new((0..3).collect::<Vec<_>>());
        let expected = HashSet::from([
          vec![0, 1, 2],
          vec![0, 2, 1],
          vec![1, 0, 2],
          vec![1, 2, 0],
          vec![2, 0, 1],
          vec![2, 1, 0],
        ]);
        assert_eq!(expected,p.collect::<HashSet<_>>());

        let p = Permutation::new([0,0,1,2].to_vec());
        let expected = HashSet::from([
          vec![0, 0, 1, 2],
          vec![0, 0, 2, 1],
          vec![0, 1, 0, 2],
          vec![0, 1, 2, 0],
          vec![0, 2, 0, 1],
          vec![0, 2, 1, 0],
          vec![1, 0, 0, 2],
          vec![1, 0, 2, 0],
          vec![1, 2, 0, 0],
          vec![2, 0, 0, 1],
          vec![2, 0, 1, 0],
          vec![2, 1, 0, 0],
        ]);
        assert_eq!(expected,p.collect::<HashSet<_>>());

        let p = Permutation::new([0,0,1,2].to_vec());
        assert_eq!(24,p.collect::<Vec<_>>().len());
    }
}
