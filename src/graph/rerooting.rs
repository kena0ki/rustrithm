

/// About usage, see mod test block below.
pub trait EntitySpec<T>: Copy+Default {
    /// Operation between two child nodes.
    fn op(&self, rhs: Self, v:usize, e:usize, t: &T) -> Self;
    /// Method for adding the edge after the corresponding dfs.
    #[allow(unused_variables)]
    fn add_edge(&self, v:usize, e:usize, t: &T) -> Self {
        return *self;
    }
    #[allow(unused_variables)]
    /// Method for adding the root node after all child nodes is merged.
    fn add_root(&self, v:usize, to: &Vec<(usize,usize)>, parent: usize, t: &T) -> Self {
        return *self;
    }
}

pub struct Rerooting<T, E:EntitySpec<T>> {
    pub t:T,
    pub dp:Vec<Vec<E>>,
    pub result:Vec<E>,
    pub adj:Vec<Vec<(usize,usize)>>,
}

impl <T, E:EntitySpec<T>> Rerooting<T, E> {
    pub fn new(n: usize, t:T) -> Self {
        Self {
            t,
            dp:vec![Vec::new();n],
            result:vec![E::default();n],
            adj:vec![Vec::with_capacity(n-1);n],
        }
    }
    pub fn add_edge(&mut self, u:usize, v:usize,e:usize) {
        self.adj[u].push((v,e));
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
            let dp_v = dp_v.add_edge(v,e,t);
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
            let dp_u = dp_u.add_edge(u,e,t);
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
    use crate::math::combi::Combi;
    use super::*;

    const MOD:usize = 1000000007;

    type CombiM = Combi<MOD>;

    #[derive(Clone,Copy)]
    pub struct Entity {
        val:usize,
        size:usize,
    }
    impl Default for Entity {
        fn default() -> Self {
            return Self { val:1, size:0 };
        }
    }
    impl EntitySpec<CombiM> for Entity {
        fn op(&self, rhs: Self, _:usize, _e: usize, t:&CombiM) -> Self {
            let newsize = self.size+rhs.size;
            let mut newval = self.val;
            newval *= rhs.val;
            newval %= MOD;
            newval *= t.kcombi(newsize, rhs.size);
            newval %= MOD;
            return Self { val:newval, size: newsize };
        }
        fn add_root(&self, _v:usize, _adj: &Vec<(usize,usize)>,_:usize,_:&CombiM) -> Self {
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
        let fact = Combi::<1000000007>::new(2*n);
        let mut r = Rerooting::<_,Entity>::new(n,fact);
        for (i,&(u,v)) in input.iter().enumerate() {
            r.add_edge(u-1,v-1,i);
            r.add_edge(v-1,u-1,i);
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
            assert_eq!(expected[i], r.result[i].val);
        }
    }

}


