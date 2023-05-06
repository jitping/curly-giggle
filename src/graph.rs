use core::cmp::Reverse;
use std::collections::BinaryHeap;

pub type Vertex = usize;
pub type Distance = usize;
pub type Edge = (Vertex, Vertex, Distance);

#[derive(Debug, Copy, Clone)]
pub struct Outedge {
    vertex: Vertex,
    length: Distance,
}
type AdjacencyList = Vec<Outedge>;

#[derive(Debug)]
pub struct Graph {
    n: usize,
    outedges: Vec<AdjacencyList>,
}

impl Graph {
    pub fn create_directed(n: usize, edges: &Vec<Edge>) -> Graph {
        let mut outedges = vec![vec![]; n];
        for (u, v, length) in edges {
            outedges[*u].push(Outedge {
                vertex: *v,
                length: *length,
            });
        }
        Graph { n, outedges }
    }
    pub fn find_shortest_path(&self, start: Vertex, end: usize) -> usize {
        let mut distances: Vec<Option<Distance>> = vec![None; self.n];
        distances[start] = Some(0);
        let mut pq = BinaryHeap::<Reverse<(Distance, Vertex)>>::new();
        pq.push(Reverse((0, start)));
        while let Some(Reverse((dist, v))) = pq.pop() {
            for Outedge { vertex, length } in self.outedges[v].iter() {
                let new_dist = dist + *length;
                let update = match distances[*vertex] {
                    None => true,
                    Some(d) => new_dist < d,
                };
                if update {
                    distances[*vertex] = Some(new_dist);
                    pq.push(Reverse((new_dist, *vertex)));
                }
            }
        }
        return distances[end].expect(
            "No path (not even through secondary nodes) found between start and end point",
        );
    }
    pub fn shortest_path_from_vertex(&self, start: Vertex) -> Vec<Option<Distance>> {
        let mut distances: Vec<Option<Distance>> = vec![None; self.n];
        distances[start] = Some(0);
        let mut pq = BinaryHeap::<Reverse<(Distance, Vertex)>>::new();
        pq.push(Reverse((0, start)));
        while let Some(Reverse((dist, v))) = pq.pop() {
            for Outedge { vertex, length } in self.outedges[v].iter() {
                let new_dist = dist + *length;
                let update = match distances[*vertex] {
                    None => true,
                    Some(d) => new_dist < d,
                };
                if update {
                    distances[*vertex] = Some(new_dist);
                    pq.push(Reverse((new_dist, *vertex)));
                }
            }
        }
        return distances;
    }
}
