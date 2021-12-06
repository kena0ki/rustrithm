use std::collections::btree_set::IntoIter;

use super::{Graph, AdjTo, Edge};

impl Graph<Edge> {
    /// Finds the sequence of edges in an Euler path starting from u, assuming
    /// it exists and that the graph is directed. Undefined behavior if this
    /// precondition is violated. To extend this to undirected graphs, maintain
    /// a visited array to skip the reverse edge.
    pub fn euler_path(&self, u: usize) -> Vec<usize> {
        let mut adj_iters = (0..self.num_v())
            .map(|u| self.adj_list(u).into_iter())
            .collect::<Vec<_>>();
        let mut edges = Vec::with_capacity(self.num_e());
        self.euler_recurse(u, &mut adj_iters, &mut edges);
        edges.reverse();
        edges
    }

    // Helper function used by euler_path. Note that we can't use a for-loop
    // that would consume the adjacency list as recursive calls may need it.
    fn euler_recurse(&self, u: usize, adj: &mut [IntoIter<AdjTo>], edges: &mut Vec<usize>) {
        while let Some(AdjTo{edge_id:e, v}) = adj[u].next() {
            self.euler_recurse(v, adj, edges);
            edges.push(e);
        }
    }

    pub fn dfs(&self, root: usize) -> DfsIterator {
        let mut visited = vec![false; self.num_v()];
        visited[root] = true;
        let adj_iters = (0..self.num_v())
            .map(|u| self.adj_list(u).into_iter())
            .collect::<Vec<_>>();

        DfsIterator {
            visited,
            stack: vec![root],
            adj_iters,
        }
    }
}

pub struct DfsIterator {
    visited: Vec<bool>,
    stack: Vec<usize>,
    adj_iters: Vec<IntoIter<AdjTo>>,
}

impl Iterator for DfsIterator {
    type Item = (usize, usize);

    /// Returns next edge and vertex in the depth-first traversal
    // Refs: https://www.geeksforgeeks.org/iterative-depth-first-traversal/
    //       https://en.wikipedia.org/wiki/Depth-first_search
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let &u = self.stack.last()?;
            while let Some(AdjTo{edge_id:e, v}) = self.adj_iters[u].next() {
                if !self.visited[v] {
                    self.visited[v] = true;
                    self.stack.push(v);
                    return Some((e, v));
                }
            }
            self.stack.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euler() {
        let mut graph = Graph::new(3, 4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 0);
        graph.add_edge(1, 2);
        graph.add_edge(2, 1);

        assert_eq!(graph.euler_path(0), vec![0, 2, 3, 1]);
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new(4, 6);
        // 0 --> 1
        //  ^ \-> 2
        //   
        graph.add_edge(0, 2);
        graph.add_edge(2, 0);
        graph.add_edge(1, 2);
        graph.add_edge(0, 1);
        graph.add_edge(3, 3);
        graph.add_edge(2, 3);

        let dfs_root = 2;
        let dfs_traversal = std::iter::once(dfs_root)
            .chain(graph.dfs(dfs_root).map(|(_, v)| v))
            .collect::<Vec<_>>();

        assert_eq!(dfs_traversal, vec![2, 0, 1, 3]);
    }

    #[test]
    fn test_dfs2() {
        let mut graph = Graph::new(5, 6);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);
        graph.add_edge(1, 0);
        graph.add_edge(0, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);

        let dfs_root = 0;
        let dfs_traversal = std::iter::once(dfs_root)
            .chain(graph.dfs(dfs_root).map(|(_, v)| v))
            .collect::<Vec<_>>();

        assert_eq!(dfs_traversal, vec![0, 2, 1, 3, 4]);
    }

    #[test]
    fn test_dfs_space_complexity() {
        let num_v = 20;
        let mut graph = Graph::new(num_v, 0);
        for i in 0..num_v {
            for j in 0..num_v {
                graph.add_undirected_edge(i, j);
            }
        }

        let dfs_root = 7;
        let mut dfs_search = graph.dfs(dfs_root);
        let mut dfs_check = vec![dfs_root];
        for _ in 1..num_v {
            dfs_check.push(dfs_search.next().unwrap().1);
            assert!(dfs_search.stack.len() <= num_v + 1);
        }

        dfs_check.sort();
        dfs_check.dedup();
        assert_eq!(0, dfs_check[0]);
        assert_eq!(num_v, dfs_check.len());
        assert_eq!(num_v - 1, dfs_check[num_v - 1]);
    }
}
