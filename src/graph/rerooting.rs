
/// About usage, see mod test block below.
pub trait EntitySpec<T>: Copy+Default {
    /// Operation between two child nodes.
    fn op(&self, rhs: Self, v:usize, e:usize, t: &T) -> Self;
    /// Event for adding the root node after merge all child nodes.
    fn add_root(&self, v:usize, to: &Vec<(usize,usize)>, parent: usize, t: &T) -> Self;
}

pub struct Rerooting<T, E:EntitySpec<T>> {
    pub t:T,
    pub dp:Vec<Vec<E>>,
    pub result:Vec<E>,
    pub adj:Vec<Vec<(usize,usize)>>,
    esize:usize,
}

impl <T, E:EntitySpec<T>> Rerooting<T, E> {
    pub fn new(n: usize, t:T) -> Self {
        Self {
            t,
            dp:vec![Vec::new();n],
            result:vec![E::default();n],
            adj:vec![Vec::with_capacity(n-1);n],
            esize: 0,
        }
    }
    pub fn add_edge(&mut self, u:usize, v:usize) {
        self.esize +=1;
        self.adj[u].push((v,self.esize));
    }
    pub fn rerooting(&mut self) {
        Self::dfs1(0,usize::max_value(), &self.adj, &mut self.dp, &self.t);
        Self::dfs2(0,usize::max_value(), &self.adj, &mut self.dp, &self.t, &mut self.result);
    }
    fn dfs1(u:usize, p:usize, adj: &Vec<Vec<(usize,usize)>>, dp:&mut Vec<Vec<E>>, t: &T)
        -> E {
        let mut res = E::default();
        dp[u] = vec![E::default();adj[u].len()];
        for (i,&(v,e)) in adj[u].iter().enumerate() {
            if p == v { continue; }
            let dp_v = Self::dfs1(v,u,adj,dp,t);
            dp[u][i] = dp_v;
            res = res.op(dp_v,v,e,t);
        }
        return res.add_root(u,&adj[u],p,t);
    }
    fn dfs2(u:usize, p:usize, adj: &Vec<Vec<(usize,usize)>>, dp:&mut Vec<Vec<E>>, t:&T
        , result: &mut Vec<E>) {
        let len = adj[u].len();
        let mut dp_l = vec![E::default();len+1];
        for (i,&(v,e)) in adj[u].iter().enumerate() {
            dp_l[i+1] = dp_l[i].op(dp[u][i],v,e,t);
        }
        let mut dp_r = vec![E::default();len+1];
        for (i,&(v,e)) in adj[u].iter().enumerate().rev() {
            dp_r[i] = dp_r[i+1].op(dp[u][i],v,e,t);
        }
        for (i,&(v,e)) in adj[u].iter().enumerate() {
            if p == v { continue; }
            let dp_u = dp_l[i].op(dp_r[i+1],v,e,t);
            let dp_u = dp_u.add_root(u,&adj[u],v,t);
            for (i,&(w,_)) in adj[v].iter().enumerate() {
                if w == u {
                    dp[v][i] = dp_u;
                    break;
                }
            }
            Self::dfs2(v,u,adj,dp,t,result);
        }
        result[u] = dp_l[len];
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
    impl Default for Entity {
        fn default() -> Self {
            return Self { val:ZERO_MOD1000000007+1, size:0 };
        }
    }
    impl EntitySpec<FactM> for Entity {
        fn op(&self, rhs: Self, _:usize, _e: usize, t:&FactM) -> Self {
            let newsize = self.size+rhs.size;
            let mut newval = self.val;
            newval *= rhs.val;
            newval *= t.combin(newsize, rhs.size);
            return Self { val:newval, size: newsize };
        }
        fn add_root(&self, _v:usize, _adj: &Vec<(usize,usize)>,_:usize,_:&FactM) -> Self {
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
            assert_eq!(expected[i], r.result[i].val.val());
        }
    }

}


