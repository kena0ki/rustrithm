//! Maximum flows, matchings, and minimum cuts.
use super::{Graph, AdjTo,FlowEdge};

impl Graph<FlowEdge> {
    pub fn add_flow_edge(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost: i64) -> (usize,usize){
        let edge_id = self.num_e();
        // add an edge
        self.adj.entry(u).or_default().insert(AdjTo{ edge_id, v });
        self.edges.push(FlowEdge { u, v, cap, cost, flow:0 });
        // add a residual edge
        self.adj.entry(v).or_default().insert(AdjTo{ edge_id: edge_id+1, v:u });
        self.edges.push(FlowEdge { v:u, u:v, cap:rcap, cost: -cost, flow:0 });
        return (edge_id,edge_id+1);
    }
}

/// Representation of a network flow problem with (optional) costs.
pub struct FlowGraph {
    /// Owned graph, managed by this FlowGraph object.
    pub graph: Graph<FlowEdge>,
    distance: Vec<i64>,
}

impl FlowGraph {
    /// An upper limit to the flow.
    const INF: i64 = i64::MAX;

    /// Initializes an flow network with vmax vertices and no edges.
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            graph: Graph::new(vmax, 2 * emax_hint),
            distance: vec![],
        }
    }

    /// Adds an edge with rcap == 0.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost:i64) -> (usize,usize) {
       return self.add_edge_rcap(u,v,cap,0,cost);
    }

    /// Adds an edge with specified directional capacities per unit of
    /// flow. If only forward flow is allowed, rcap should be zero.
    /// Returns the IDs of the added edge and residual edge.
    pub fn add_edge_rcap(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost:i64) -> (usize,usize) {
        return self.graph.add_flow_edge(u,v,cap,rcap,cost);
    }

    /// Gets an edge by the edge id.
    pub fn get_edge(&self, i: usize) -> &FlowEdge{
        return &self.graph.edges[i];
    }

    /// Gets edges in the graph including residual edges.
    pub fn edges(&self) -> &[FlowEdge]{
        return &*self.graph.edges;
    }

    /// Gets iterator of edges in the graph excluding residual edges.
    pub fn non_residual_edges_iter(&self) -> std::iter::StepBy<std::slice::Iter<FlowEdge>> {
        return self.graph.edges.iter().step_by(2);
    }

    /// clear flow value once they are calculated.
    pub fn clear_flow(&mut self) {
        for e in self.graph.edges.iter_mut() {
            e.flow = 0;
        }
    }

    fn augment_path(&mut self, e: usize, flow: i64) {
        self.graph.edges[e].flow += flow;
        self.graph.edges[e ^ 1].flow -= flow;
    }

    /// Among all s-t maximum flows, finds one with minimum cost, assuming
    /// s != t and no negative-cost cycles.
    ///
    /// # Panics
    ///
    /// Panics if the flow or cost overflow a 64-bit signed integer.
    pub fn mcf(&mut self, s: usize, t: usize) -> (i64, i64) {
        return self.mcf_flow_limit(s,t,i64::MAX);
    }

    /// Finds minimum cost flow with a flow limitation.
    pub fn mcf_flow_limit(&mut self, s: usize, t: usize, mut flow_limit: i64) -> (i64, i64) {
        let mut pot = vec![0; self.graph.num_v()];

        // Bellman-Ford deals with negative-cost edges at initialization.
        for _ in 1..self.graph.num_v() {
            for e in 0..self.graph.num_e() {
                let edge = &self.graph.edges[e];
                if edge.cap > 0 {
                    pot[edge.v] = pot[edge.v].min(pot[edge.u] + edge.cost);
                }
            }
        }

        let (mut min_cost, mut max_flow) = (0, 0);
        loop {
            if flow_limit <= 0 {
                break;
            }
            let par = self.mcf_search(s, &mut pot); // find shortest path from s to t.
            if par[t] == None {
                break;
            }
            let (dc, df) = self.mcf_augment(t, &par,flow_limit);
            min_cost += dc;
            max_flow += df;
            flow_limit-=df;
        }
        (min_cost, max_flow)
    }

    // Maintains Johnson's potentials to prevent negative-cost residual edges.
    // This allows running Dijkstra instead of the slower Bellman-Ford.
    fn mcf_search(&mut self, s: usize, pot: &mut [i64]) -> Vec<Option<usize>> {
        let mut vis = vec![false; self.graph.num_v()];
        self.distance = vec![Self::INF; self.graph.num_v()];
        let mut par = vec![None; self.graph.num_v()];

        self.distance[s] = 0;
        while let Some(u) = (0..self.graph.num_v())
            .filter(|&u| !vis[u] && self.distance[u] < Self::INF)
            .min_by_key(|&u| self.distance[u] - pot[u])
        {
            vis[u] = true;
            pot[u] = self.distance[u];
            for AdjTo{edge_id:e, v} in self.graph.adj_list(u) {
                let edge = &self.graph.edges[e];
                if self.distance[v] > self.distance[u] + edge.cost && edge.flow < edge.cap {
                    self.distance[v] = self.distance[u] + edge.cost;
                    par[v] = Some(e);
                }
            }
        }
        par
    }

    // Pushes flow along an augmenting path of minimum cost.
    fn mcf_augment(&mut self, t: usize, par: &[Option<usize>], flow_limit: i64) -> (i64, i64) {
        let (mut dc, mut df) = (0, Self::INF);
        let mut u = t;
        while let Some(e) = par[u] {
            let edge = &self.graph.edges[e];
            df = df.min(edge.cap - edge.flow).min(flow_limit);
            u = edge.u;
        }
        u = t;
        while let Some(e) = par[u] {
            self.augment_path(e, df);
            let edge = &self.graph.edges[e];
            dc += df * edge.cost;
            u = edge.u;
        }
        (dc, df)
    }

    pub fn debug_print(&self, residual: bool) {
        let step = if residual { 1 } else { 2 };
        for e in self.graph.edges.iter().step_by(step) {
            println!("{:?}", e);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cost_flow() {
        let mut graph = FlowGraph::new(4, 4);
        graph.add_edge(0, 1, 10, -10);
        graph.add_edge(1, 2, 7, 8);
        graph.add_edge(2, 3, 7, 8);
        graph.add_edge(1, 3, 7, 10);

        let (cost, flow) = graph.mcf(0, 3);
        assert_eq!(cost, 18);
        assert_eq!(flow, 10);
    }

    #[test]
    // https://atcoder.jp/contests/practice2/tasks/practice2_e
    // https://atcoder.github.io/ac-library/production/document_en/mincostflow.html
    pub fn mcf_matching() {
        // case 1
        let input: &[&[i64]]= &[
            &[5,3,2],
            &[1,4,8],
            &[7,6,9],
        ];
        let (sum, result) = fnc(3,1,input);
        let expected = vec![
            "X..",
            "..X",
            ".X.",
        ];
        assert_eq!(19, sum);
        assert_eq!(expected, result);

        // case 2
        let input: &[&[i64]]= &[
            &[10,10,1],
            &[10,10,1],
            &[1,1,10],
        ];
        let (sum, result) = fnc(3,2,input);
        let expected = vec![
            "XX.",
            "XX.",
            "..X",
        ];
        assert_eq!(50, sum);
        assert_eq!(expected, result);

        fn fnc(n:usize, k:usize, input: &[&[i64]]) -> (i64, Vec<String>){
            let mut graph = FlowGraph::new(2*n+2,n*n+2*n);
            let big=1_000_000_000;
            let ni64 = n as i64;
            let ki64 = k as i64;
            let s=2*n;
            let t=2*n+1;
            graph.add_edge(s,t,ni64 *ki64,big);
            for i in 0..n {
                graph.add_edge(s,i,ki64,0);
                graph.add_edge(n+i,t,ki64,0);
            }
            for i in 0..n {
                for j in 0..n {
                    graph.add_edge(i,n+j,1,big-input[i][j]);
                }
            }

            let (min_cost, _max_flow) = graph.mcf_flow_limit(s,t,ni64 * ki64);
            graph.debug_print(false);

            let sum = ni64 * ki64 * big - min_cost;
            return unsafe {
                let mut result = vec![String::from_utf8_unchecked(vec![b'.';n]);n];
                for e in graph.non_residual_edges_iter() {
                    if e.u == s || e.v == t || e.flow == 0 {
                        continue;
                    }
                    result[e.u].as_bytes_mut()[e.v-n] = b'X';
                }
                (sum, result)
            };
        }

    }
}

