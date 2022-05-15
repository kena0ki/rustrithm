use std::collections::BTreeMap;

pub struct Prime {
    n: usize,
    sieve: Vec<usize>,
    pub primes: Vec<usize>,
}

impl Prime {
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
        return Self {n, sieve, primes};
    }
    pub fn factorize(&self, mut x: usize) -> BTreeMap<usize,usize> {
        if x < 2 || x>self.n {
            panic!("x should be 2 <= x <= n.");
        }
        let mut facts = BTreeMap::new();
        while x > 1 {
            *facts.entry(self.sieve[x]).or_default() +=1;
            x = x / self.sieve[x];
        }
        return facts;
    }
    pub fn factorize_by_sqroot(&self, mut x: usize) -> BTreeMap<usize, usize> {
        let mut facts = BTreeMap::new();
        if x < 2 || x>self.n*self.n {
            panic!("x should be 2 <= x <= n*n.");
        }
        for &p in &self.primes {
            while x%p == 0 {
                *facts.entry(p).or_default()+=1;
                x/=p;
            }
        }
        if x>1 {
            facts.insert(x,1);
        }
        return facts;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prime() {
        let fct = Prime::new(20);
        assert_eq!(vec![2,3,5,7,11,13,17,19], fct.primes);
        let expect = BTreeMap::from([(2,2),(5,1)]);
        assert_eq!(expect, fct.factorize(20));
        let expect = BTreeMap::from([(2,1),(3,2)]);
        assert_eq!(expect, fct.factorize(18));
        let expect = BTreeMap::from([(2,4),(5,2)]);
        assert_eq!(expect, fct.factorize_by_sqroot(400));
        let expect = BTreeMap::from([(3,2),(43,1)]);
        assert_eq!(expect, fct.factorize_by_sqroot(387));
    }
}

