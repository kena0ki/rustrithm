//! Graph connectivity structures.

/// Helper struct that carries data needed for the depth-first searches in
/// ConnectivityGraph's constructor.
struct ConnectivityData {
    time: usize,
    vis: Box<[usize]>,
    low: Box<[usize]>,
    v_stack: Vec<usize>,
    e_stack: Vec<usize>,
    cc: Vec<usize>,
    vcc: Vec<usize>,
    num_cc: usize,
    num_vcc: usize,
}

impl ConnectivityData {
    fn new(num_v: usize, num_e:usize) -> Self {
        Self {
            time: 0,
            vis: vec![0; num_v].into_boxed_slice(),
            low: vec![0; num_v].into_boxed_slice(),
            v_stack: vec![],
            e_stack: vec![],
            cc: vec![0; num_v],
            vcc: vec![0; num_e],
            num_cc: 0,
            num_vcc: 0,
        }
    }

    fn visit(&mut self, u: usize) {
        self.time += 1;
        self.vis[u] = self.time;
        self.low[u] = self.time;
        self.v_stack.push(u);
    }

    fn lower(&mut self, u: usize, val: usize) {
        if self.low[u] > val {
            self.low[u] = val
        }
    }
}

/// Represents the decomposition of a graph into any of its constituent parts:
///
/// - Connected components (CC),
/// - Strongly connected components (SCC),
/// - 2-edge-connected components (2ECC),
/// - 2-vertex-connected components (2VCC)
///
/// Multiple-edges and self-loops are correctly handled.
pub struct ConnectivityGraph {
    /// Number of vertices
    pub num_v:usize,
    /// Adjacency list (edge_id, vertex_id edge point to).
    pub adj:Vec<Vec<(usize,usize)>>,
    /// Edges (u, v). One edge for undirected graph.
    pub edges:Vec<(usize,usize)>,
    /// ID of a vertex's CC, SCC or 2ECC, whichever applies. Range 1 to num_cc.
    pub cc: Vec<usize>,
    /// ID of an edge's 2VCC, where applicable. Ranges from 1 to num_vcc.
    pub vcc: Vec<usize>,
    /// Total number of CCs, SCCs or 2ECCs, whichever applies.
    pub num_cc: usize,
    /// Total number of 2VCCs, where applicable.
    pub num_vcc: usize,
}

impl ConnectivityGraph {
    pub fn new(n:usize) -> Self {
        Self {
            num_v: n,
            adj: vec![Vec::new();n],
            edges: vec![],
            cc: vec![],
            vcc: vec![],
            num_cc: 0,
            num_vcc: 0,
        }
    }
    pub fn add_edge(&mut self, u:usize, v:usize) {
        let eid = self.edges.len();
        self.adj[u].push((eid,v));
        self.edges.push((u,v));
    }
    pub fn add_undirected_edge(&mut self, u:usize, v:usize) {
        let eid = self.edges.len();
        self.adj[u].push((eid,v));
        self.adj[v].push((eid,u));
        self.edges.push((u,v));
    }
    /// If we think of each even-numbered vertex as a variable, and its
    /// odd-numbered successor as its negation, then we can build the
    /// implication graph corresponding to any 2-CNF formula.
    /// Note that u||v == !u -> v == !v -> u.
    pub fn add_two_sat_clause(&mut self, u: usize, v: usize) {
        self.add_edge(u ^ 1, v);
        self.add_edge(v ^ 1, u);
    }

    /// Computes CCs (connected components), SCCs (strongly connected
    /// components), 2ECCs (2-edge-connected components), and/or 2VCCs
    /// (2-vertex-connected components), depending on the parameter and graph:
    /// - is_directed == true on directed graph: SCCs in rev-topological order
    /// - is_directed == true on undirected graph: CCs
    /// - is_directed == false on undirected graph: 2ECCs and 2VCCs
    /// - is_directed == false on directed graph: undefined behavior
    pub fn build(&mut self, is_directed: bool) {
        let mut data = ConnectivityData::new(self.num_v, self.edges.len());
        for u in 0..self.num_v {
            if data.vis[u] > 0 { continue; }
            if is_directed {
                self.scc(&mut data, u);
            } else {
                let par = self.edges.len()+1;
                self.bcc(&mut data, u, par);
            }
        }
        self.cc = data.cc;
        self.vcc = data.vcc;
        self.num_cc = data.num_cc;
        self.num_vcc = data.num_vcc;
    }

    // This is based on Tarjan's strongly connected components algorithm
    // but slightly different about lowering.
    // Original version use v.index but this method not.
    //  https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
    // I think this logic comes from here.
    //  https://www.youtube.com/watch?v=wUgWX0nc4NY
    //  https://github.com/williamfiset/Algorithms/blob/86661d3daf3063eae2ea9329b069456b87490b62/src/main/java/com/williamfiset/algorithms/graphtheory/TarjanSccSolverAdjacencyList.java#L78
    fn scc(&self, data: &mut ConnectivityData, u: usize) {
        data.visit(u);
        for &(_,v) in &self.adj[u] {
            if data.vis[v] == 0 {
                self.scc(data, v);
            }
            if data.cc[v] == 0 {
                data.lower(u, data.low[v]);
            }
        }
        if data.vis[u] == data.low[u] {
            data.num_cc += 1;
            while let Some(v) = data.v_stack.pop() {
                data.cc[v] = data.num_cc;
                if v == u {
                    break;
                }
            }
        }
    }

    /// From the directed implication graph corresponding to a 2-SAT clause,
    /// finds a satisfying assignment if it exists or returns None otherwise.
    /// NOTE: call build() before use this method.
    pub fn two_sat_assign(&self) -> Option<Vec<bool>> {
        (0..self.num_v / 2)
            .map(|i| {
                let scc_true = self.cc[2 * i];
                let scc_false = self.cc[2 * i + 1];
                if scc_true == scc_false {
                    None
                } else {
                    Some(scc_true < scc_false)
                }
            })
            .collect()
    }

    /// Gets the vertices of a graph according to a topological order of the
    /// strongly connected components. Most often used on DAGs.
    /// NOTE: call build() before use this method.
    pub fn topological_sort(&self) -> Vec<usize> {
        let mut vertices = (0..self.num_v).collect::<Vec<_>>();
        vertices.sort_unstable_by_key(|&u| self.num_cc - self.cc[u]);
        vertices
    }

    fn bcc(&self, data: &mut ConnectivityData, u: usize, par: usize) {
        data.visit(u);
        for &(e,v) in &self.adj[u] {
            if data.vis[v] == 0 {
                data.e_stack.push(e);
                self.bcc(data, v, e);
                data.lower(u, data.low[v]);
                if data.vis[u] <= data.low[v] { // no back-edge in subtree
                    // u is a cut vertex unless it's a one-child root
                    data.num_vcc += 1;
                    while let Some(top_e) = data.e_stack.pop() {
                        data.vcc[top_e] = data.num_vcc;
                        if e == top_e {
                            break;
                        }
                    }
                }
            } else if data.vis[v] < data.vis[u] && e != par { // found a back-edge and u is not parent of v.
                data.lower(u, data.vis[v]);
                data.e_stack.push(e);
            } else if v == u {
                // e is a self-loop
                data.num_vcc += 1;
                data.vcc[e] = data.num_vcc;
            }
        }
        if data.vis[u] == data.low[u] {
            // par is a cut edge unless par==-1
            data.num_cc += 1;
            while let Some(v) = data.v_stack.pop() {
                data.cc[v] = data.num_cc;
                if v == u {
                    break;
                }
            }
        }
    }

    /// In an undirected graph, determines whether u is an articulation vertex.
    /// NOTE: call build() before use this method.
    pub fn is_cut_vertex(&self, u: usize) -> bool {
        if let Some(&(first_e,_)) = self.adj[u].get(0) {
            self.adj[u].iter().skip(1).any(|&(e,_)| self.vcc[first_e] != self.vcc[e])
        } else {
            false
        }
    }

    /// In an undirected graph, determines whether e is a bridge
    /// NOTE: call build() before use this method.
    pub fn is_cut_edge(&self, e: usize) -> bool {
        if let Some(&(u,v)) = self.edges.get(e) {
            return self.cc[u] != self.cc[v];
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scc() {
        let mut graph = ConnectivityGraph::new(6);
        graph.add_edge(1,4);
        graph.add_edge(5,2);
        graph.add_edge(3,0);
        graph.add_edge(5,5);
        graph.add_edge(4,1);
        graph.add_edge(0,3);
        graph.add_edge(4,2);

        graph.build(true);
        assert_eq!(
            graph.cc,
            vec![1, 3, 2, 1, 3, 4]
        );
    }

    #[test]
    fn test_toposort() {
        let mut graph = ConnectivityGraph::new(4);
        graph.add_edge(0, 0);
        graph.add_edge(0, 2);
        graph.add_edge(3, 2);
        graph.add_edge(3, 1);
        graph.add_edge(1, 0);

        graph.build(true);
        assert_eq!(
            graph.topological_sort(),
            vec![3, 1, 0, 2]
        );
    }

    #[test]
    fn test_two_sat() {
        let mut graph = ConnectivityGraph::new(6);
        let (x, y, z) = (0, 2, 4);

        graph.add_two_sat_clause(x, z);
        graph.add_two_sat_clause(y ^ 1, z ^ 1);
        graph.add_two_sat_clause(y, y);
        graph.build(true);
        assert_eq!(
            graph.two_sat_assign(),
            Some(vec![true, true, false])
        );

        graph.add_two_sat_clause(z, z);
        graph.build(true);
        assert_eq!(graph.two_sat_assign(), None);
    }

    #[test]
    fn test_biconnected() {
        let mut graph = ConnectivityGraph::new(3);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(1, 2);
        graph.add_undirected_edge(1, 2);

        graph.build(false);
        let bridges = (0..graph.edges.len())
            .filter(|&e| graph.is_cut_edge(e))
            .collect::<Vec<_>>();
        let articulation_points = (0..graph.num_v)
            .filter(|&u| graph.is_cut_vertex(u))
            .collect::<Vec<_>>();

        assert_eq!(bridges, vec![0]);
        assert_eq!(articulation_points, vec![1]);
    }
}
