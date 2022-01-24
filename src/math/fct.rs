use std::collections::BTreeMap;

pub struct Factor {
    sieve: Vec<usize>,
    pub primes: Vec<usize>,
}

impl Factor {
    pub fn new(n:usize) -> Self{
        let mut sieve = vec![0;n+1];  // i=0,1 elements are not used.
        let mut primes = Vec::new();
        for i in 2..n {
            if sieve[i] > 0 {
                continue;
            }
            primes.push(i);
            sieve[i] = i;
            let mut j = i*i;
            while j <= n {
                if sieve[j] == 0 {
                    sieve[j] = i;
                }
                j+=i;
            }
        }
        return Self {sieve, primes};
    }
    pub fn factors(&self, mut x: usize) -> BTreeMap<usize,usize> {
        if x < 2 {
            panic!("x should be greater than 1.");
        }
        let mut facts = BTreeMap::new();
        while x > 1 {
            *facts.entry(self.sieve[x]).or_default() +=1;
            x = x / self.sieve[x];
        }
        return facts;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fct() {
        let fct = Factor::new(20);
        assert_eq!(vec![2,3,5,7,11,13,17,19], fct.primes);
        let expect = BTreeMap::from([(2,1),(3,2)]);
        assert_eq!(expect, fct.factors(18));
    }
}

