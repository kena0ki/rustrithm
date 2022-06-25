/// Represents a union of disjoint sets. Each set's elements are arranged in a
/// tree, whose root is the set's representative.
#[derive(Debug,Default,Clone)]
pub struct DisjointSets {
    parent: Vec<usize>,
    size_nodes: Vec<usize>,
    num_sets: usize,
}

impl DisjointSets {
    /// Initializes disjoint sets containing one element each.
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size_nodes: vec![1;size],
            num_sets: size,
        }
    }

    /// Finds the set's representative. Do path compression along the way to make
    /// future queries faster.
    pub fn find(&mut self, u: usize) -> usize {
        let su = self.size_nodes[u];
        if su>0 {
            return u;
        }
        self.parent[u] = self.find(self.parent[u]);
        return self.parent[u];
    }

    /// Merges the sets containing u and v into a single set containing their
    /// union. Returns true if u and v were previously in different sets.
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let mut pu = self.find(u);
        let mut pv = self.find(v);
        if pu == pv {
            return false;
        }
        let su = self.size_nodes[pu];
        let sv = self.size_nodes[pv];
        if su<sv {
            std::mem::swap(&mut pu,&mut pv);
        }
        self.size_nodes[pu] += self.size_nodes[pv];
        self.size_nodes[pv] = 0;
        self.num_sets-=1;
        self.parent[pv] = pu;
        return true;
    }
    /// Returns the number of nodes in the set.
    pub fn count(&mut self, v:usize) -> usize {
        let p = self.find(v);
        return self.size_nodes[p];
    }
    /// Returns the number of sets in the graph.
    pub fn count_sets(&mut self) -> usize {
        return self.num_sets;
    }
    /// Tests if two vertices are in the same set.
    pub fn same(&mut self, u:usize, v:usize) -> bool {
        return self.find(u)==self.find(v);
    }
    /// Tests if the vertex is a representative node.
    pub fn leader(&mut self, v:usize) -> bool {
        return self.size_nodes[v]>0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dsu_count() {
        let mut du = DisjointSets::new(5);
        assert_eq!(1, du.count(0));
        assert_eq!(1, du.count(4));

        du.merge(1,3);
        du.merge(3,2);
        assert_eq!(3, du.count(1));
        assert_eq!(3, du.count(2));
        assert_eq!(3, du.count(3));
        assert_eq!(1, du.count(0));
        assert_eq!(1, du.count(4));

        du.merge(3,2);
        assert_eq!(3, du.count(2));
        assert_eq!(3, du.count(3));

        du.merge(0,4);
        du.merge(3,4);
        assert_eq!(5, du.count(0));
    }
}
