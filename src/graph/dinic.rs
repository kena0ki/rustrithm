//! Maximum flows, matchings, and minimum cuts.
use std::{collections::{btree_set::IntoIter, HashMap, BTreeSet}, iter::StepBy};

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct FlowEdge {
    pub u: usize,
    pub v: usize,
    pub cap: i64,
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

/// Implementation of Dinic's algorithm
pub struct Dinic {
    adj: HashMap<usize,BTreeSet<AdjTo>>, // two edges for an undirected edge
    num_vert: usize,
    edges: Vec<FlowEdge>, // one edge for an undirected edge
    distance: Vec<i64>,
}

impl Dinic {
    /// An upper limit to the flow.
    const INF: i64 = i64::MAX;

    /// Initializes an flow network with vmax vertices and no edges.
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            adj: HashMap::with_capacity(emax_hint),
            num_vert: vmax,
            edges: Vec::with_capacity(emax_hint),
            distance: vec![],
        }
    }

    /// Returns the number of vertices.
    fn num_v(&self) -> usize {
        return self.num_vert;
    }

    /// Returns the number of edges.
    fn num_e(&self) -> usize {
        return self.edges.len();
    }

    /// Gets vertex u's adjacency list.
    fn adj_list(&self, u: usize) -> BTreeSet<AdjTo> {
        return self.adj.get(&u).unwrap_or(&BTreeSet::new()).to_owned();
    }

    fn add_flow_edge(&mut self, u: usize, v: usize, cap: i64, rcap: i64) {
        let edge_id = self.num_e();
        // add an edge
        self.adj.entry(u).or_default().insert(AdjTo{ edge_id, v });
        self.edges.push(FlowEdge { u, v, cap, flow:0 });
        // add a residual edge
        self.adj.entry(v).or_default().insert(AdjTo{ edge_id: edge_id+1, v:u });
        self.edges.push(FlowEdge { v:u, u:v, cap:rcap, flow:0 });
    }

    /// Adds an edge with specified directional capacities per unit of
    /// flow. If only forward flow is allowed, rcap should be zero.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        self.add_flow_edge(u,v,cap,0);
    }

    pub fn add_edge_rcap(&mut self, u: usize, v: usize, cap: i64, rcap: i64, ) {
        self.add_flow_edge(u,v,cap,rcap);
    }

    /// Iterator of the edges not including residual edges.
    pub fn edge_iter(&self) -> StepBy<std::slice::Iter<FlowEdge>>{
        return self.edges.iter().step_by(2);
    }

    /// Get an nth edge. The specified index corresponds to the order of adding edges.
    pub fn get_edge(&self, n: usize) -> &FlowEdge{
        return &self.edges[n*2];
    }

    /// Underlying edges in the graph including residual edges.
    pub fn edges_including_residual_edges(&self) -> &[FlowEdge]{
        return &*self.edges;
    }

    /// clear flow value once they are calculated.
    pub fn clear_flow(&mut self) {
        for e in self.edges.iter_mut() {
            e.flow = 0;
        }
    }

    fn augment_path(&mut self, e: usize, flow: i64) {
        self.edges[e].flow += flow;
        self.edges[e ^ 1].flow -= flow;
    }

    /// Dinic's algorithm to find the maximum flow from s to t where s != t.
    /// Generalizes the Hopcroft-Karp maximum bipartite matching algorithm.
    /// V^2E in general, min(V^(2/3),sqrt(E))E when all edges are unit capacity,
    /// sqrt(V)E when all vertices are unit capacity as in bipartite graphs.
    ///
    /// # Panics
    ///
    /// Panics if the maximum flow is 2^63 or larger.
    pub fn dinic(&mut self, s: usize, t: usize) -> i64 {
        let mut max_flow = 0;
        loop {
            self.dinic_search(s);
            if self.distance[t] == Self::INF {
                break;
            }
            // Keep track of adjacency lists to avoid revisiting blocked edges.
            let mut adj_iters = (0..self.num_v())
                .map(|u| self.adj_list(u).into_iter().peekable())
                .collect::<Vec<_>>();
            max_flow += self.dinic_augment(s, t, Self::INF, &mut adj_iters);
        }
        max_flow
    }

    // Compute BFS distances to restrict attention to shortest path edges.
    fn dinic_search(&mut self, s: usize) {
        let mut q = ::std::collections::VecDeque::new();
        self.distance = vec![Self::INF; self.num_v()];
        self.distance[s] = 0;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for AdjTo{edge_id:e, v} in self.adj_list(u) {
                if self.distance[v] == Self::INF && self.edges[e].flow < self.edges[e].cap {
                    self.distance[v] = self.distance[u] + 1;
                    q.push_back(v);
                }
            }
        }
    }

    // Pushes a blocking flow that increases the residual's s-t distance.
    fn dinic_augment(
        &mut self,
        u: usize,
        t: usize,
        flow_input: i64,
        adj: &mut [::std::iter::Peekable<IntoIter<AdjTo>>],
    ) -> i64 {
        if u == t {
            return flow_input;
        }
        let mut flow_used = 0;

        while let Some(&AdjTo{edge_id:e, v}) = adj[u].peek() {
            let edge = &self.edges[e];
            let rem_cap = (edge.cap - edge.flow).min(flow_input - flow_used);// min(remaining capacity, remaining flow)
            if rem_cap > 0 && self.distance[v] == self.distance[u] + 1 {
                // calculates maximum flow in a subtree (max_flow).
                // max_flow never exceeds the remaining flow since rem_cap is not greater than
                // the remaining flow.
                let max_flow = self.dinic_augment(v, t, rem_cap, adj);
                self.augment_path(e, max_flow);
                flow_used += max_flow; // add the maximum flow in a subtree
                if flow_used == flow_input { // until the summary reaches to the input flow.
                    break;
                }
            }
            // The current edge is either saturated or blocked.
            adj[u].next();
        }
        flow_used
    }

    /// After running maximum flow, use this to recover the dual minimum cut.
    pub fn min_cut(&self) -> Vec<usize> {
        (0..self.num_e())
            .filter(|&e| { // filter blocked edges
                let edge = &self.edges[e];
                self.distance[edge.u] < Self::INF && self.distance[edge.v] == Self::INF
            })
            .collect()
    }

    pub fn debug_print(&self, out: &mut impl ::std::io::Write, residual: bool) {
        let step = if residual { 1 } else { 2 };
        for e in self.edges.iter().step_by(step) {
            writeln!(out, "{:?}", e).ok();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dinic() {
        let mut graph = Dinic::new(5, 5);
        graph.add_edge(0, 1, 3);
        graph.add_edge(1, 2, 2);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 4, 2);
        graph.add_edge(3, 4, 2);

        let max = graph.dinic(0, 4);
        assert_eq!(max, 3);
    }

    #[test]
    fn test_dinic_min_cut() {
        let mut graph = Dinic::new(3, 2);
        graph.add_edge(0, 1, 4);
        graph.add_edge(1, 2, 3);

        let max = graph.dinic(0, 2);
        println!("max: {:?}", max);
        assert_eq!(max, 3);
        assert_eq!(&[2], &*graph.min_cut());
    }

    #[test]
    fn test_dinic_max_matching() {
        let mut graph = Dinic::new(14, 4);

        let source = 0;
        let sink = 13;

        //Vertex indices of "left hand side" of bipartite graph go from [left_start, right_start)
        let left_start = 1;
        //Vertex indices of "right hand side" of bipartite graph go from [right_start, sink)
        let right_start = 7;

        //Initialize source / sink connections; both left & right have 6 nodes
        for lhs_vertex in left_start..left_start + 6 {
            graph.add_edge(source, lhs_vertex, 1);
        }

        for rhs_vertex in right_start..right_start + 6 {
            graph.add_edge(rhs_vertex, sink, 1);
        }

        graph.add_edge(left_start + 0, right_start + 1, 1);
        graph.add_edge(left_start + 0, right_start + 2, 1);
        graph.add_edge(left_start + 2, right_start + 0, 1);
        graph.add_edge(left_start + 2, right_start + 3, 1);
        graph.add_edge(left_start + 3, right_start + 2, 1);
        graph.add_edge(left_start + 4, right_start + 2, 1);
        graph.add_edge(left_start + 4, right_start + 3, 1);
        graph.add_edge(left_start + 5, right_start + 5, 1);

        let flow_amt = graph.dinic(source, sink);
        assert_eq!(flow_amt, 5);

        let mut matched_edges = graph.edge_iter()
            .filter(|&e| e.flow>0 && e.u != source && e.v != sink);
        assert_eq!(FlowEdge { u: 1, v: 8, cap: 1, flow: 1 },  *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 3, v: 7, cap: 1, flow: 1 },  *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 4, v: 9, cap: 1, flow: 1 },  *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 5, v: 10, cap: 1, flow: 1 }, *matched_edges.next().unwrap());
        assert_eq!(FlowEdge { u: 6, v: 12, cap: 1, flow: 1 }, *matched_edges.next().unwrap());

        // //L->R edges in maximum matching
        // let left_right_edges = flow
        //     .into_iter()
        //     .enumerate()
        //     .filter(|&(_e, f)| f > 0)
        //     //map to u->v
        //     .map(|(e, _f)| (graph.graph.edges[e].u, graph.graph.edges[e].v))
        //     //leave out source and sink nodes
        //     .filter(|&(u, v)| u != source && v != sink)
        //     .collect::<Vec<_>>();

        // assert_eq!(
        //     left_right_edges,
        //     vec![(1, 8), (3, 7), (4, 9), (5, 10), (6, 12)]
        // );
    }
}

