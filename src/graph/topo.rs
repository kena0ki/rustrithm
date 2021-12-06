
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

use super::Graph;
use super::Edge;
use super::InDegree;

pub struct TopoGraph {
    indeg_cnt: HashMap<usize, usize>,
    graph: Graph<Edge>,
}

impl TopoGraph {
    pub fn new(vmax:usize, emax_hint: usize) -> Self {
        return Self {
            indeg_cnt: HashMap::with_capacity(vmax),
            graph: Graph::new(vmax,emax_hint),
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.graph.add_adj(u,v);
        self.graph.edges.push(Edge { u, v });
        *self.indeg_cnt.entry(v).or_default()+=1;
    }

    /// topological sort in lexicographical order
    pub fn topological_sort(&mut self) -> Result<Vec<usize>, ()> {
        let mut heap = (0..self.graph.num_v())
            .filter(|i| ! self.indeg_cnt.contains_key(i))
            .map(|i| Reverse(i))
            .collect::<BinaryHeap<_>>();
        let mut sorted_nodes = Vec::with_capacity(self.graph.num_v());
        while let Some(Reverse(i)) = heap.pop() {
            sorted_nodes.push(i);
            for InDegree { v, .. } in self.graph.adj_list(i) {
                *self.indeg_cnt.get_mut(&v).unwrap()-=1;
                if self.indeg_cnt[&v] == 0 {
                    heap.push(Reverse(v));
                }
            }
        }
        return if sorted_nodes.len() == self.graph.num_v() {
            Ok(sorted_nodes)
        } else {
            Err(())
        };
    }
}

#[cfg(test)]
mod graph_test {
    use super::*;

    #[test]
    fn topo_sort() {
        let mut graph = TopoGraph::new(5,5);
        let input = [
            (5,4),
            (2,1),
            (3,4),
            (2,4),
            (5,3),
        ];
        for (u,v) in input {
            graph.add_edge(u-1,v-1);
        }

        let sorted = graph.topological_sort().unwrap().iter().map(|i| i+1).collect::<Vec<_>>();
        assert_eq!(&[2,1,5,3,4], &*sorted);

    }
}
