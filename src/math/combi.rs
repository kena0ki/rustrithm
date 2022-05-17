use std::convert::TryInto;

pub struct Combi<const MOD:usize> {
    fact: Vec<usize>,
    ifact: Vec<usize>,
}
impl <const MOD:usize> Combi<MOD> {
    pub fn new(n:usize) -> Self{
        let mut fact = vec![0;n+1];
        fact[0]=1;
        for i in 0..n {
            fact[i+1] = fact[i]*(i+1)%MOD;
        }
        let mut ifact = vec![0;n+1];
        ifact[n] = Self::inv(fact[n]);
        for i in (0..n).rev() {
            ifact[i] = ifact[i+1]*(i+1)%MOD;
        }
        return Self { fact, ifact};
    }

    pub fn kperm<U: TryInto<usize>>(&self,n:U,k:U) -> usize {
        let n = n.try_into().ok().expect("Unable to cast n to usize");
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if n < k {
            return 0;
        }
        return self.fact[n]*self.ifact[n-k]%MOD;
    }

    pub fn kcombi<U: TryInto<usize>>(&self,n:U,k:U) -> usize {
        let n = n.try_into().ok().expect("Unable to cast n to usize");
        let k = k.try_into().ok().expect("Unable to cast k to usize");
        if n < k {
            return 0;
        }
        return self.fact[n]*self.ifact[k]%MOD*self.ifact[n-k]%MOD;
    }

    pub fn fact(&self) -> &Vec<usize> { &self.fact }
    pub fn ifact(&self) -> &Vec<usize> { &self.ifact }

    fn pow(val:usize, mut power: usize) -> usize {
        let mut square = val;
        let mut ret = 1;
        while 0 < power {
            if (power & 1) == 1{
                ret *= square;
                ret %= MOD;
            }
            square *= square;
            square %= MOD;
            power >>= 1;
        }
        return ret;
    }
    fn inv(val: usize) -> usize {
        return Self::pow(val, MOD - 2);
    }
}

impl <const MOD:usize> Default for Combi<MOD>{
    fn default() -> Self {
        return Self::new(1_000_000);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combi() {
        let f = Combi::<1009>::new(10);
        assert_eq!(720,f.kperm(10,3));
        assert_eq!(120,f.kcombi(10,3));
        assert_eq!(72,f.kperm(9,2));
        assert_eq!(36,f.kcombi(9,2));
        let f = Combi::<11>::new(10);
        assert_eq!(5,f.kperm(10,3));
        assert_eq!(10,f.kcombi(10,3));
    }

}
