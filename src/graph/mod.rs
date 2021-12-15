//! Basic graph module without explicit support for deletion.
//!
//! # Panics
//!
//! All methods will panic if given an out-of-bounds element index.
pub mod connectivity;
pub mod flow;
pub mod grid;
pub mod disjoint_set;
pub mod topo;
pub mod util;

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::cmp::Reverse;

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct WeightedEdge {
    pub u: usize,
    pub v: usize,
    pub weight: i64,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct FlowEdge {
    pub u: usize,
    pub v: usize,
    pub cap: i64,
    pub cost: i64,
    pub flow: i64,
}

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct AdjTo {
    pub edge_id: usize,
    pub v: usize,
}
impl Ord for AdjTo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.edge_id.cmp(&other.edge_id);
    }
}
impl PartialOrd for AdjTo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A compact graph representation.
#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct Graph<T> {
    adj: HashMap<usize,BTreeSet<AdjTo>>, // two edges for an undirected edge
    num_vert: usize,
    edges: Vec<T>, // one edge for an undirected edge
}

impl <T:std::fmt::Debug> Graph<T> {
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, emax_hint should be close to the number of
    /// edges that will be inserted.
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            adj: HashMap::with_capacity(emax_hint),
            num_vert: vmax,
            edges: Vec::with_capacity(emax_hint),
        }
    }

    /// Returns the number of vertices.
    pub fn num_v(&self) -> usize {
        return self.num_vert;
    }

    /// Returns the number of edges.
    pub fn num_e(&self) -> usize {
        return self.edges.len();
    }

    fn add_adj(&mut self, u: usize, v: usize) {
        let edge_id = self.num_e();
        self.adj.entry(u).or_default().insert(AdjTo{ edge_id, v });
    }

    fn add_undirected_adj(&mut self, u: usize, v: usize) {
        let edge_id = self.num_e();
        self.adj.entry(u).or_default().insert(AdjTo{ edge_id, v });
        self.adj.entry(v).or_default().insert(AdjTo{ edge_id, v:u });
    }

    /// Gets vertex u's adjacency list.
    pub fn adj_list(&self, u: usize) -> BTreeSet<AdjTo> {
        return self.adj.get(&u).unwrap_or(&BTreeSet::new()).to_owned();
    }

    /// Gets an edge
    pub fn edge(&self, edge_id: usize) -> &T {
        return &self.edges[edge_id];
    }

    pub fn debug_print(&self) {
        for e in &self.edges {
            println!("{:?}", e);
        }
    }
}

impl Graph<Edge> {
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.add_adj(u,v);
        self.edges.push(Edge { u, v });
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_undirected_adj(u,v);
        self.edges.push(Edge { u, v });
    }
}

impl Graph<WeightedEdge> {
    pub fn add_weighted_edge(&mut self, u: usize, v: usize, weight: i64) {
        self.add_adj(u,v);
        self.edges.push(WeightedEdge { u, v, weight });
    }

    pub fn add_weighted_undirected_edge(&mut self, u: usize, v: usize, weight: i64) {
        self.add_undirected_adj(u,v);
        self.edges.push(WeightedEdge { u, v, weight });
    }

    /// Kruskal's minimum spanning tree algorithm on an undirected graph.
    pub fn min_spanning_tree(&self) -> Vec<WeightedEdge> {
        let mut edges = self.edges.to_vec();
        edges.sort_unstable_by_key(|&e| e.weight);

        let mut components = disjoint_set::DisjointSets::new(self.num_v());
        return edges.into_iter()
            .filter(|&e| components.merge(e.u, e.v))
            .collect();
    }

    // Single-source shortest paths on a graph with non-negative weights
    pub fn dijkstra(&self, u: usize) -> (Vec<usize>, HashMap<usize,usize>) {
        let mut distance = vec![usize::max_value(); self.num_v()];
        let mut prev = HashMap::with_capacity(self.num_v());
        let mut heap = std::collections::BinaryHeap::new();

        distance[u] = 0;
        heap.push((Reverse(0), u));
        while let Some((Reverse(distance_u), u)) = heap.pop() {
            if distance[u] < distance_u || self.adj.get(&u).is_none() {
                continue;
            }
            let deg = self.adj.get(&u).unwrap();
            for &AdjTo{edge_id, v} in deg.iter() {
                let distance_v = distance_u + self.edges[edge_id].weight as usize;
                if distance[v] > distance_v {
                    prev.insert(v,u);
                    distance[v] = distance_v;
                    heap.push((Reverse(distance_v), v));
                }
            }
        }
        return (distance, prev);
    }

    pub fn dijkstra_to(&self, src: usize, dest: usize) -> (usize, Vec<usize>) {
        let (dists, prev) = self.dijkstra(src);
        let mut v = dest;
        let mut que = VecDeque::from([v]);
        while let Some(&u) = prev.get(&v) {
            que.push_front(u);
            v=u;
        }
        return (dists[dest], que.into());
    }
}

#[cfg(test)]
mod graph_test {
    use super::*;

    // https://www.geeksforgeeks.org/kruskals-minimum-spanning-tree-algorithm-greedy-algo-2/
    #[test]
    fn min_spanning_tree() {
        let mut graph = Graph::new(9,28);
        graph.add_weighted_undirected_edge(0, 1, 4 );
        graph.add_weighted_undirected_edge(0, 7, 8 );
        graph.add_weighted_undirected_edge(1, 2, 8 );
        graph.add_weighted_undirected_edge(1, 7, 11);
        graph.add_weighted_undirected_edge(2, 3, 7 );
        graph.add_weighted_undirected_edge(2, 5, 4 );
        graph.add_weighted_undirected_edge(3, 4, 9 );
        graph.add_weighted_undirected_edge(3, 5, 14);
        graph.add_weighted_undirected_edge(5, 4, 10);
        graph.add_weighted_undirected_edge(6, 5, 2 );
        graph.add_weighted_undirected_edge(7, 6, 1 );
        graph.add_weighted_undirected_edge(7, 8, 7 );
        graph.add_weighted_undirected_edge(8, 2, 2 );
        graph.add_weighted_undirected_edge(8, 6, 6 );
        let min_tree = graph.min_spanning_tree();
        println!("{:?}", min_tree);
        let expected = "[WeightedEdge { u: 7, v: 6, weight: 1 }, WeightedEdge { u: 6, v: 5, weight: 2 }, WeightedEdge { u: 8, v: 2, weight: 2 }, WeightedEdge { u: 0, v: 1, weight: 4 }, WeightedEdge { u: 2, v: 5, weight: 4 }, WeightedEdge { u: 2, v: 3, weight: 7 }, WeightedEdge { u: 0, v: 7, weight: 8 }, WeightedEdge { u: 3, v: 4, weight: 9 }]";
        assert_eq!(expected,format!("{:?}", min_tree));
    }

    #[test]
    fn dijkstra() {
        let mut graph = Graph::new(3, 3);
        graph.add_weighted_edge(0, 1, 7);
        graph.add_weighted_edge(1, 2, 3);
        graph.add_weighted_edge(2, 0, 5);

        let (dist, prev) = graph.dijkstra(1);
        assert_eq!(dist, vec![8, 0, 3]);
        let expected_prev = HashMap::from([(0,2),(2,1)]);
        assert_eq!(expected_prev, prev);
    }

    // https://www.geeksforgeeks.org/dijkstras-algorithm-for-adjacency-list-representation-greedy-algo-8/
    #[test]
    fn dijkstra_to() {
        let mut graph = Graph::new(9,28);
        graph.add_weighted_undirected_edge(0, 1, 4);
        graph.add_weighted_undirected_edge(0, 7, 8);
        graph.add_weighted_undirected_edge(1, 2, 8);
        graph.add_weighted_undirected_edge(1, 7, 11);
        graph.add_weighted_undirected_edge(2, 3, 7);
        graph.add_weighted_undirected_edge(2, 8, 2);
        graph.add_weighted_undirected_edge(2, 5, 4);
        graph.add_weighted_undirected_edge(3, 4, 9);
        graph.add_weighted_undirected_edge(3, 5, 14);
        graph.add_weighted_undirected_edge(4, 5, 10);
        graph.add_weighted_undirected_edge(5, 6, 2);
        graph.add_weighted_undirected_edge(6, 7, 1);
        graph.add_weighted_undirected_edge(6, 8, 6);
        graph.add_weighted_undirected_edge(7, 8, 7);
        let (dist, path) = graph.dijkstra_to(0, 8);
        assert_eq!(14, dist);
        assert_eq!([0, 1, 2, 8], &*path);
    }
}
