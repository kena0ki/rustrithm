use std::convert::TryInto;
use super::modulo::ModU64;

pub struct Factorial<const M:u64> {
    fact: Vec<ModU64<M>>,
    ifact: Vec<ModU64<M>>,
    n: usize,
}
impl <const M:u64> Factorial<M>{
    pub fn new(n:usize) -> Self{
        let zero = ModU64::<M>::new(0);
        let mut fact = Vec::<ModU64<M>>::with_capacity(n+1);
        fact.push(zero+1);
        for i in 1..=n {
            fact.push(fact[i-1] * (i) as u64);
        }
        let mut ifact = vec![zero+1;n+1];
        ifact[n] = fact[n].inv();
        for i in (3..=n).rev() {
            ifact[i-1] = ifact[i] * i as u64;
        }
        return Self { fact, ifact, n};
    }

    pub fn kperm <T: TryInto<usize>>(&self,k:T) -> ModU64<M> {
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if self.n < k {
            return ModU64::<M>::new(0);
        }
        return self.fact[self.n]*self.ifact[self.n-k];
    }

    pub fn kcombin <T: TryInto<usize>>(&self, k:T) -> ModU64<M> {
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if self.n < k {
            return ModU64::<M>::new(0);
        }
        return self.fact[self.n]*self.ifact[k]*self.ifact[self.n-k];
    }

    pub fn fact(&self) -> &Vec<ModU64<M>> { &self.fact }
    pub fn ifact(&self) -> &Vec<ModU64<M>> { &self.ifact }
}

pub struct Permutations<T> {
    items: Vec<T>,
    swaps: Vec<usize>,
    i: usize,
    next: Option<Vec<T>>,
}

impl <T:Clone> Permutations<T> {
    pub fn new(items: Vec<T>) -> Permutations<T> {
        let swaps = vec![0; items.len()];
        let next = Some(items.clone());
        Permutations { items, swaps, i: 1, next }
    }
}

impl <T:Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.clone();
        loop {
            if self.i >= self.items.len() {
                self.next = None;
                return next;
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
        self.next = Some(self.items.clone());
        return next;
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_factorial() {
        let f = Factorial::<1009>::new(10);
        assert_eq!(720,f.kperm(3).val());
        assert_eq!(120,f.kcombin(3).val());
        let f = Factorial::<11>::new(10);
        assert_eq!(5,f.kperm(3).val());
        assert_eq!(10,f.kcombin(3).val());
    }

    #[test]
    fn test_permutation() {
        let p = Permutations::new((0..3).collect::<Vec<_>>());
        let expected = HashSet::from([
          vec![0, 1, 2],
          vec![0, 2, 1],
          vec![1, 0, 2],
          vec![1, 2, 0],
          vec![2, 0, 1],
          vec![2, 1, 0],
        ]);
        assert_eq!(expected,p.collect::<HashSet<_>>());

        let p = Permutations::new([0,0,1,2].to_vec());
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

        let p = Permutations::new([0,0,1,2].to_vec());
        assert_eq!(24,p.collect::<Vec<_>>().len());
    }
}
