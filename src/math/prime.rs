use std::collections::BTreeMap;

pub struct Prime {
    n: usize,
    sieve: Vec<usize>,
    pub primes: Vec<usize>,
}

impl Prime {
    /// Initializes Prime.
    /// O(NlogN)
    pub fn new(n:usize) -> Self{
        let mut sieve = vec![0;n+1];  // i=0,1 elements are not used.
        let mut primes = Vec::new();
        for i in 2..n+1 {
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
    /// Factorizes the number.
    /// O(1) if 2 <= x <= n.
    /// O(M) if n < x <= n*n where M is the number of primes.
    /// panic otherwise.
    pub fn factorize(&self, mut x: usize) -> BTreeMap<usize,usize> {
        if x < 2 || x>self.n*self.n {
            panic!("x should be 2 <= x <= n*n, but it was {}", x);
        }
        let mut facts = BTreeMap::new();
        if x<=self.n {
            while x > 1 {
                *facts.entry(self.sieve[x]).or_default() +=1;
                x = x / self.sieve[x];
            }
        } else {
            for &p in &self.primes {
                while x%p == 0 {
                    *facts.entry(p).or_default()+=1;
                    x/=p;
                }
            }
            if x>1 {
                facts.insert(x,1);
            }
        }
        return facts;
    }
    /// Tests whether the number is prime.
    /// O(1) if 2 <= x <= n.
    /// O(M) if n < x <= n*n where M is the number of primes.
    /// panic otherwise.
    pub fn is_prime(&self, x:usize) -> bool {
        if x < 2 || x>self.n*self.n {
            panic!("x should be 2 <= x <= n*n, but it was {}", x);
        }
        if x<=self.n {
            return self.sieve[x]==x;
        }
        for &p in &self.primes {
            if x%p == 0 {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prime() {
        let prm = Prime::new(20);
        assert_eq!(vec![2,3,5,7,11,13,17,19], prm.primes);
        let expect = BTreeMap::from([(2,2),(5,1)]);
        assert_eq!(expect, prm.factorize(20));
        let expect = BTreeMap::from([(2,1),(3,2)]);
        assert_eq!(expect, prm.factorize(18));
        let expect = BTreeMap::from([(2,4),(5,2)]);
        assert_eq!(expect, prm.factorize(400));
        let expect = BTreeMap::from([(3,2),(43,1)]);
        assert_eq!(expect, prm.factorize(387));

        assert_eq!(true, prm.is_prime(19));
        assert_eq!(false, prm.is_prime(20));
        assert_eq!(true, prm.is_prime(397));
        assert_eq!(false, prm.is_prime(400));

        let prm = Prime::new(3);
        assert_eq!(vec![2,3], prm.primes);
    }
}

