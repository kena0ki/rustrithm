use std::collections::HashMap;

/// Represents a union of disjoint sets. Each set's elements are arranged in a
/// tree, whose root is the set's representative.
#[derive(Debug,Default,Clone)]
pub struct DisjointSets {
    parent: Vec<usize>,
    num_nodes: HashMap<usize,usize>,
}

impl DisjointSets {
    /// Initializes disjoint sets containing one element each.
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            num_nodes: HashMap::<_,_>::with_capacity(size),
        }
    }

    /// Finds the set's representative. Do path compression along the way to make
    /// future queries faster.
    pub fn find(&mut self, u: usize) -> usize {
        let pu = self.parent[u];
        if pu != u {
            self.parent[u] = self.find(pu);
        }
        self.parent[u]
    }

    /// Merges the sets containing u and v into a single set containing their
    /// union. Returns true if u and v were previously in different sets.
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let (pu, cu) = self.find_and_count(u);
        let (pv, cv) = self.find_and_count(v);
        let diff = pu != pv;
        if diff {
            self.num_nodes.remove(&pu);
            self.num_nodes.insert(pv, cv+cu);
        }
        self.parent[pu] = pv;
        diff
    }

    /// Returns the set's representative with the number of nodes in the set.
    pub fn find_and_count(&mut self, v:usize) -> (usize, usize) {
        let p = self.find(v);
        if let Some(&num) = self.num_nodes.get(&p) {
            return (p, num);
        }
        return (v, 1);
    }

    /// Returns the number of nodes in the set.
    pub fn count(&mut self, v:usize) -> usize {
        return self.find_and_count(v).1;
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
