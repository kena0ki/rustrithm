pub struct Lca {
    pub dp: Vec<Vec<usize>>,
    pub dist: Vec<usize>,
    pub n: usize,
    pub m: usize,
}

impl Lca {
    pub fn new(n:usize) -> Self {
        let mut m = 1;
        let mut x=n;
        while x > 0 {
            x/=2;
            m+=1;
        }
        return Self { dp: vec![vec![0;m];n], dist: vec![0;n], n, m };
    }
    pub fn init(&mut self, adj: &Vec<Vec<usize>>) {
        self.dfs(adj,0,0);
    }
    fn dfs(&mut self, adj: &Vec<Vec<usize>>, u: usize, p: usize) {
        self.dp[u][0] = p;
        for i in 1..self.m {
            self.dp[u][i] = self.dp[self.dp[u][i-1]][i-1];
        }
        for &v in &adj[u] {
            if v == p { continue; }
            self.dist[v] = self.dist[u]+1;
            self.dfs(adj,v,u);
        }
    }
    pub fn lca(&self, mut u:usize, mut v:usize) -> usize {
        if u>v { std::mem::swap(&mut u, &mut v) }
        let d = self.dist[v]-self.dist[u];
        for i in 0..self.m {
            if d>>i&1 == 1 {
                v = self.dp[v][i];
            }
        }
        if u == v {
            return u;
        }
        for i in (0..self.m).rev() {
            if self.dp[u][i] != self.dp[v][i] {
                u=self.dp[u][i];
                v=self.dp[v][i];
            }
        }
        return self.dp[u][0];
    }
    pub fn len(&self, u:usize, v:usize) -> usize {
        return self.dist[u]+self.dist[v]-2*self.dist[self.lca(u,v)];
    }
    pub fn is_between(&self, a:usize, u:usize, v:usize) -> bool {
        return self.len(u,v) == self.len(u,a)+self.len(a,v);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lca() {
        let tree = vec![
            (0,1),
            (1,2),
            (1,3),
            (2,4),
            (2,5),
            (3,6),
            (3,7),
            (6,8),
            (6,9),
            (7,10),
            (7,11),
            (9,12),
            (9,13),
            (10,14),
            (10,15),
            (10,16),
            (10,17),
        ];
        let n=tree.len()+1;
        let mut adj = vec![Vec::new();n];
        for i in 0..n-1 {
            let (u,v) = tree[i];
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut lca = Lca::new(n);
        lca.init(&adj);

        assert_eq!(3,lca.lca(17,12));
        assert_eq!(6,lca.lca(8,12));
        assert_eq!(1,lca.lca(1,5));
        assert_eq!(0,lca.lca(0,0));

        assert_eq!(6,lca.len(17,12));
        assert_eq!(3,lca.len(8,12));
        assert_eq!(2,lca.len(1,5));
        assert_eq!(0,lca.len(0,0));

        assert_eq!(true,lca.is_between(3,17,12));
        assert_eq!(true,lca.is_between(6,17,12));
        assert_eq!(false,lca.is_between(8,17,12));

    }
}
