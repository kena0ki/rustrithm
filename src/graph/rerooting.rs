
/// About usage, see mod test block below.
pub trait EntitySpec<T>: Copy {
    fn identity() -> Self;
    fn add(&self, rhs: Self, v:usize, adj: &Vec<Vec<usize>>, t: &T) -> Self;
    fn sub(&self, rhs: Self, v:usize, adj: &Vec<Vec<usize>>, t: &T) -> Self;
    fn add_root(&self, v:usize, adj: &Vec<Vec<usize>>, t: &T) -> Self;
}

pub struct Rerooting<T, E:EntitySpec<T>> {
    pub t:T,
    pub dp:Vec<E>,
    pub adj:Vec<Vec<usize>>,
}

impl <T, E:EntitySpec<T>> Rerooting<T, E> {
    pub fn new(n: usize, t:T) -> Self {
        Self {
            t,
            dp:vec![E::identity();n],
            adj:vec![Vec::with_capacity(n-1);n],
        }
    }
    pub fn add_edge(&mut self, u:usize, v:usize) {
        self.adj[u].push(v);
    }
    pub fn rerooting(&mut self) {
        Self::dfs1(0,usize::max_value(), &self.adj, &mut self.dp, &self.t);
        Self::dfs2(0,usize::max_value(), &self.adj, &mut self.dp, &self.t);
    }
    fn dfs1(u:usize, p:usize, adj: &Vec<Vec<usize>>, dp:&mut Vec<E>, t: &T) {
        let mut val = E::identity();
        for &v in &adj[u] {
            if p == v { continue; }
            Self::dfs1(v,u,adj,dp,t);
            let dp_v = dp[v].add_root(v,adj, t);
            val = val.add(dp_v, v, adj, t);
        }
        dp[u] = val;
    }
    fn dfs2(u:usize, p:usize, adj: &Vec<Vec<usize>>, dp:&mut Vec<E>, t:&T) {
        for &v in &adj[u] {
            if p == v { continue; }
            let dp_v = dp[v].add_root(v,adj,t);
            let dp_u = dp[u].sub(dp_v,v,adj,t);
            let dp_u = dp_u.add_root(v,adj,t);
            dp[v] = dp[v].add(dp_u,v,adj,t);
            Self::dfs2(v,u,adj,dp,t);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::math::{combin::Factorial, num::{ModU64, ZERO_MOD1000000007, MOD1000000007}};
    use super::*;

    type FactM = Factorial<MOD1000000007>;

    #[derive(Clone,Copy)]
    pub struct Entity {
        val:ModU64<MOD1000000007>,
        size:u64,
    }
    impl EntitySpec<FactM> for Entity {
        fn identity() -> Self {
            return Self { val:ZERO_MOD1000000007+1, size:0 };
        }
        fn add(&self, rhs: Self, _:usize, _adj: &Vec<Vec<usize>>, t:&FactM) -> Self {
            let newsize = self.size+rhs.size;
            let mut newval = self.val;
            newval *= rhs.val;
            newval *= t.combin(newsize, rhs.size);
            return Self { val:newval, size: newsize };
        }
        fn sub(&self, rhs: Self, _:usize, _adj: &Vec<Vec<usize>>, t:&FactM) -> Self {
            let mut newval = self.val;
            newval /= rhs.val;
            newval /= t.combin(self.size, rhs.size);
            let newsize = self.size-rhs.size;
            return Self { val:newval, size: newsize };
        }
        fn add_root(&self, _v:usize, _adj: &Vec<Vec<usize>>, _:&FactM) -> Self {
            return Self { val: self.val, size: self.size+1 };
        }
    }

    // https://atcoder.jp/contests/abc160/tasks/abc160_f
    #[test]
    fn test_rerooting() {
        let n = 8;
        let input = [
            (1,2),
            (2,3),
            (3,4),
            (3,5),
            (3,6),
            (6,7),
            (6,8),
        ];
        let fact = Factorial::<MOD1000000007>::new(2*n);
        let mut r = Rerooting::<_,Entity>::new(n,fact);
        for &(u,v) in &input {
            r.add_edge(u-1,v-1);
            r.add_edge(v-1,u-1);
        }
        r.rerooting();
        let expected = [
            40
            ,280
            ,840
            ,120
            ,120
            ,504
            ,72
            ,72
        ];
        for i in 0..n {
            assert_eq!(expected[i], r.dp[i].val.val());
        }
    }

}


