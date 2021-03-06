//! Maximum flows, matchings, and minimum cuts.

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub struct FlowEdge {
    pub u: usize,
    pub v: usize,
    pub cap: i64,
    pub flow: i64,
}

/// Implementation of Dinic's algorithm
pub struct Dinic {
    adj: Vec<Vec<(usize,usize)>>,
    num_vert: usize,
    edges: Vec<FlowEdge>,
    distance: Vec<i64>,
}

impl Dinic {
    /// An upper limit to the flow.
    const INF: i64 = i64::max_value();

    /// Initializes an flow network with vmax vertices and no edges.
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            adj: vec![Vec::with_capacity(2*emax_hint/vmax);vmax],
            num_vert: vmax,
            edges: Vec::with_capacity(2*emax_hint),
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

    fn add_flow_edge(&mut self, u: usize, v: usize, cap: i64, rcap: i64) -> (usize,usize) {
        let edge_id = self.num_e();
        // add an edge
        self.adj[u].push((edge_id,v));
        self.edges.push(FlowEdge { u, v, cap, flow:0 });
        // add a residual edge
        self.adj[v].push((edge_id+1,u));
        self.edges.push(FlowEdge { v:u, u:v, cap:rcap, flow:0 });
        return (edge_id,edge_id+1);
    }

    /// Adds an edge with rcap == 0.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) -> (usize,usize) {
       return self.add_edge_rcap(u,v,cap,0);
    }

    /// Adds an edge with specified directional capacities per unit of
    /// flow. If only forward flow is allowed, rcap should be zero.
    /// Returns the IDs of the added edge and residual edge.
    pub fn add_edge_rcap(&mut self, u: usize, v: usize, cap: i64, rcap: i64) -> (usize,usize) {
        return self.add_flow_edge(u,v,cap,rcap);
    }

    /// Gets an edge by the edge id.
    pub fn get_edge(&self, i: usize) -> &FlowEdge{
        return &self.edges[i];
    }

    /// Gets edges in the graph including residual edges.
    pub fn edges(&self) -> &[FlowEdge]{
        return &*self.edges;
    }

    /// Gets iterator of edges in the graph excluding residual edges.
    pub fn non_residual_edges_iter(&self) -> std::iter::StepBy<std::slice::Iter<FlowEdge>> {
        return self.edges.iter().step_by(2);
    }

    /// Clears flow values that have been calculated before.
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
            let mut from = vec![0;self.num_v()];
            let mut f=Self::INF;
            while f>0 {
                f = self.dinic_augment(s, t, Self::INF, &mut from);
                max_flow+=f;
            }
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
            for &(e, v) in &self.adj[u] {
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
        from: &mut Vec<usize>,
    ) -> i64 {
        if u == t {
            return flow_input;
        }
        let mut flow_used = 0;

        let num_edges = self.adj[u].len();
        for i in from[u]..num_edges {
            let (e,v) = self.adj[u][i];
            let edge = &self.edges[e];
            let rem_cap = (edge.cap - edge.flow).min(flow_input - flow_used);// min(remaining capacity, remaining flow)
            if rem_cap > 0 && self.distance[v] == self.distance[u] + 1{
                // calculates maximum flow in a subtree (max_flow).
                // max_flow never exceeds the remaining flow since rem_cap is not greater than
                // the remaining flow.
                let max_flow = self.dinic_augment(v, t, rem_cap, from);
                self.augment_path(e, max_flow);
                flow_used += max_flow; // add the maximum flow in a subtree
                if flow_used == flow_input { // until the summary reaches to the input flow.
                    break;
                }
            }
            // The current edge is either saturated or blocked.
            from[u]+=1;
        }
        flow_used
    }

    /// Returns whether edges are minimum cut or not.
    /// After running maximum flow, use this to recover the dual minimum cut.
    pub fn min_cut(&self) -> Vec<bool> {
        (0..self.num_e())
            .map(|e| {
                let edge = &self.edges[e];
                self.distance[edge.u] < Self::INF && self.distance[edge.v] == Self::INF
            })
            .collect()
    }

    pub fn debug_print(&self, residual: bool) {
        if cfg!(debug_assertions) {
            let step = if residual { 1 } else { 2 };
            for e in self.edges.iter().step_by(step) {
                println!("{:?}", e);
            }
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
        assert_eq!(&[false,false,true,false], &*graph.min_cut());
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

        let mut matched_edges = graph.edges().iter()
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

