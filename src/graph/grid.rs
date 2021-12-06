use super::Graph;
use super::Edge;
use super::WeightedEdge;
use super::flow::FlowGraph;

#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct Grid<T> {
    x_size: usize,
    y_size: usize,
    graph: T,
}

impl <T> Grid<T> {
    pub fn graph(&self) -> &T {
        return &self.graph;
    }
    pub fn x_size(&self) -> usize {
        return self.x_size;
    }
    pub fn y_size(&self) -> usize {
        return self.y_size;
    }
    pub fn new(x_size: usize, y_size:usize, graph: T) -> Self {
        return Self { x_size, y_size, graph };
    }
    pub fn coord_to_node(&self, x:usize, y:usize) -> usize {
        if x >= self.x_size || y >= self.y_size {
            panic!("x >= x_size: {:?} >= {:?} or y >= y_size: {:?} >= {:?}", x, self.x_size, y, self.y_size);
        }
        let mut offset_base = 1;
        let mut node = offset_base*x;
        offset_base *= self.x_size;
        node += offset_base*y;
        return node;
    }
    pub fn node_to_coord(&self, node:usize) -> (usize, usize) {
        if node >= self.x_size * self.y_size {
            panic!("node >= self.x_size * self.y_size: {:?} >= {:?} * {:?}", node, self.x_size, self.y_size);
        }
        let x= node % self.x_size;
        let y= node / self.x_size;
        return (x,y);
    }
    fn edges_from_node<F>(&mut self, x: usize, y:usize, delta_x: &[i64], delta_y: &[i64], should_skip: F) -> Vec<(usize,usize)>
        where F: Fn(usize,usize) -> bool {
        let mut edges = Vec::with_capacity(delta_x.len());
        for i in 0..delta_x.len() {
            let x2 = x as i64 + delta_x[i];
            let y2 = y as i64 + delta_y[i];
            if x2 < 0 || y2 < 0 {
                continue;
            }
            let x2 = x2 as usize;
            let y2 = y2 as usize;
            if x2 >= self.x_size || y2 >= self.y_size || should_skip(x2,y2) {
                continue;
            }
            let u = self.coord_to_node(x,y);
            let v = self.coord_to_node(x2,y2);
            edges.push((u,v));
        }
        return edges;
    }
}

impl Grid<Graph<Edge>> {
    pub fn add_edge(&mut self, u:usize,v:usize) {
        self.graph.add_edge(u,v);
    }
    pub fn construct_node<F>(&mut self, x: usize, y:usize, delta_x: &[i64], delta_y: &[i64], should_skip: F)
        where F: Fn(usize,usize) -> bool {
        for (u,v) in self.edges_from_node(x,y,delta_x,delta_y,should_skip) {
            self.add_edge(u,v);
        }
    }
    pub fn debug_print(&self) {
        for edge in &self.graph.edges {
            println!("{:?}: {:?} -> {:?}", edge, self.node_to_coord(edge.u), self.node_to_coord(edge.v));
        }
    }
}

impl Grid<Graph<WeightedEdge>> {
    pub fn add_weighted_edge(&mut self, u:usize,v:usize, weight: i64) {
        self.graph.add_weighted_edge(u,v, weight);
    }
    pub fn construct_node<F>(&mut self, x: usize, y:usize, weight: i64, delta_x: &[i64], delta_y: &[i64], should_skip: F)
        where F: Fn(usize,usize) -> bool {
        for (u,v) in self.edges_from_node(x,y,delta_x,delta_y,should_skip) {
            self.add_weighted_edge(u,v,weight);
        }
    }
    pub fn debug_print(&self) {
        for edge in &self.graph.edges {
            println!("{:?}: {:?} -> {:?}", edge, self.node_to_coord(edge.u), self.node_to_coord(edge.v));
        }
    }
}

impl Grid<FlowGraph> {
    pub fn add_flow_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) {
        self.graph.add_edge(u,v,cap,cost);
    }
    pub fn add_flow_edge_rcap(&mut self, u: usize, v: usize, cap: i64, rcap: i64, cost: i64) {
        self.graph.add_edge_rcap(u,v,cap,rcap,cost);
    }
    pub fn construct_node<F>(&mut self, x: usize, y:usize, cap: i64, cost: i64,
        delta_x: &[i64], delta_y: &[i64], should_skip: F)
        where F: Fn(usize,usize) -> bool {
        self.construct_node_rcap(x,y,cap,0,cost,delta_x,delta_y,should_skip);
    }
    pub fn construct_node_rcap<F>(&mut self, x: usize, y:usize, cap: i64, rcap: i64, cost: i64,
        delta_x: &[i64], delta_y: &[i64], should_skip: F)
        where F: Fn(usize,usize) -> bool {
        for (u,v) in self.edges_from_node(x,y,delta_x,delta_y,should_skip) {
            self.graph.add_edge_rcap(u,v,cap,rcap,cost);
        }
    }
    pub fn debug_print(&self) {
        for edge in &self.graph.graph.edges {
            println!("{:?}: {:?} -> {:?}", edge, self.node_to_coord(edge.u), self.node_to_coord(edge.v));
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use super::*;
    #[test]
    fn grid() {
        let x=3;
        let y=3;
        let dx = [-1,1,0,0];
        let dy = [0,0,-1,1];
        let graph = Graph::<Edge>::new(x*y, (x*y)*dx.len());
        let mut grid = Grid::new(x, y, graph);
        for x1 in 0..x {
            for y1 in 0..y {
                grid.construct_node(x1,y1,&dx,&dy,|_,_| false);
            }
        }
        let expectd = &[
        //  u      , v
            ((0, 0), (1, 0)),
            ((0, 0), (0, 1)),
            ((0, 1), (1, 1)),
            ((0, 1), (0, 0)),
            ((0, 1), (0, 2)),
            ((0, 2), (1, 2)),
            ((0, 2), (0, 1)),
            ((1, 0), (0, 0)),
            ((1, 0), (2, 0)),
            ((1, 0), (1, 1)),
            ((1, 1), (0, 1)),
            ((1, 1), (2, 1)),
            ((1, 1), (1, 0)),
            ((1, 1), (1, 2)),
            ((1, 2), (0, 2)),
            ((1, 2), (2, 2)),
            ((1, 2), (1, 1)),
            ((2, 0), (1, 0)),
            ((2, 0), (2, 1)),
            ((2, 1), (1, 1)),
            ((2, 1), (2, 0)),
            ((2, 1), (2, 2)),
            ((2, 2), (1, 2)),
            ((2, 2), (2, 1)),
        ];
        for i in 0..grid.graph.edges.len() {
            let Edge { u, v } = grid.graph.edges[i];
            assert_eq!(expectd[i].0, grid.node_to_coord(u));
            assert_eq!(expectd[i].1, grid.node_to_coord(v));
        }
    }

    #[test]
    fn grid_shortest_path() {
        let x=3;
        let y=3;
        let dx = [-1,1,0,0];
        let dy = [0,0,-1,1];
        let weight = [
            [1,1,1],
            [2,5,1],
            [1,3,1],
        ];
        let graph = Graph::<WeightedEdge>::new(x*y, (x*y)*dx.len());
        let mut grid = Grid::new(x, y, graph);
        for x1 in 0..x {
            for y1 in 0..y {
                grid.construct_node(x1,y1,weight[y1][x1],&dx,&dy,|_,_| false);
            }
        }
        let (dists, prev) = grid.graph.dijkstra(grid.coord_to_node(2,2));
        assert_eq!(4, dists[0]);
        let mut v = 0;
        let mut que = VecDeque::from([grid.node_to_coord(v)]);
        while let Some(&u) = prev.get(&v) {
            que.push_front(grid.node_to_coord(u));
            v=u;
        }
        let expected_path = [(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)];
        assert_eq!(expected_path, que.make_contiguous());
    }

    #[test]
    // https://atcoder.jp/contests/practice2/tasks/practice2_d
    // https://atcoder.github.io/ac-library/production/document_en/maxflow.html
    fn max_flow_matching() {
        let x_size=3;
        let y_size=3;
        let source=x_size*y_size;
        let sink=x_size*y_size+1;
        let graph = FlowGraph::new(x_size*y_size+2,x_size*y_size*4);
        let mut grid = Grid::new(x_size,y_size, graph);
        let input = &mut [
            "#..".to_string(),
            "..#".to_string(),
            "...".to_string(),
        ];
        let delta_x = &[-1,1,0,0];
        let delta_y = &[0,0,-1,1];
        let should_skip = |x:usize,y:usize| {
            return input[y].as_bytes()[x] == '#' as u8;
        };
        for i in 0..x_size {
            for j in 0..y_size {
                if should_skip(i,j) {
                    continue;
                }
                let v = grid.coord_to_node(i,j);
                if (i&1 == 0) && (j&1 == 0) {
                    grid.add_flow_edge(source, v, 1, 0);
                    grid.construct_node(i,j, 1, 0, delta_x,delta_y,should_skip);
                } else {
                    grid.add_flow_edge(v, sink, 1, 0);
                }
            }
        }

        let max_flow = grid.graph.dinic(source, sink);

        for e in grid.graph.edge_iter() {
            if e.u == source || e.v == sink {
                continue;
            }
            let (x1,y1) = grid.node_to_coord(e.u);
            let (x2,y2) = grid.node_to_coord(e.v);
            if y1 == y2 {
                unsafe {
                    input[y1.min(y2)].as_bytes_mut()[x1.min(x2)] = '>' as u8;
                    input[y1.max(y2)].as_bytes_mut()[x1.max(x2)] = '<' as u8;
                }
            } else {
                unsafe {
                    input[y1.min(y2)].as_bytes_mut()[x1.min(x2)] = 'V' as u8;
                    input[y1.max(y2)].as_bytes_mut()[x1.max(x2)] = '^' as u8;
                }
            }
        }

        assert_eq!(3, max_flow);
        let expected = &[
            "#><",
            "V.#",
            "^><",
        ];
        assert_eq!(expected, input);
    }
}
