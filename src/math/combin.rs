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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combin() {
        let f = Factorial::<1009>::new(10);
        assert_eq!(720,f.kperm(3).val());
        assert_eq!(120,f.kcombin(3).val());
        let f = Factorial::<11>::new(10);
        assert_eq!(5,f.kperm(3).val());
        assert_eq!(10,f.kcombin(3).val());
    }
}
