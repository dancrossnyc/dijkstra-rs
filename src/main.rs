pub type Vertex = usize;
pub type Weight = i32;

#[derive(Debug)]
pub struct Graph<const N: usize> {
    edges: [Vec<(Vertex, Weight)>; N],
}

impl<const N: usize> Graph<N> {
    pub fn new(edges: &[Edge]) -> Graph<N> {
        let mut graph = Graph {
            edges: [const { Vec::new() }; N],
        };
        for edge in edges.into_iter() {
            graph.add_edge(edge);
        }
        graph
    }

    pub fn add_edge(&mut self, edge: &Edge) {
        self.edges[edge.start].push((edge.end, edge.weight));
    }

    pub fn dijkstra(&self, start: Vertex) {
        let mut dist = [Weight::MAX; N];
        let mut prev: [Option<Vertex>; N] = [None; N];
        let mut q = Heap::new();
        q.insert(WeightedVertex(start, 0));
        while !q.is_empty() {
            let WeightedVertex(u, w) = q.pop_min().unwrap();
            if w < dist[u] {
                dist[u] = w;
            }
            for &(v, wuv) in self.edges[u].iter() {
                if dist[u] + wuv < dist[v] {
                    prev[v] = Some(u);
                    dist[v] = dist[u] + wuv;
                    q.insert(WeightedVertex(v, dist[v]));
                }
            }
        }
        println!("dist={dist:?}, prev={prev:?}");
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Edge {
    start: Vertex,
    end: Vertex,
    weight: Weight,
}

impl Edge {
    pub fn new(edge: (Vertex, Vertex), weight: Weight) -> Edge {
        let (start, end) = edge;
        Edge { start, end, weight }
    }
}

#[derive(Debug)]
pub struct Heap<T: Copy + PartialOrd + PartialEq> {
    heap: Vec<T>,
}

impl<T> Heap<T>
where
    T: Copy + PartialOrd + PartialEq,
{
    pub fn new() -> Heap<T> {
        Heap { heap: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn insert(&mut self, v: T) {
        self.heap.push(v);
        let mut k = self.heap.len() - 1;
        while k > 0 {
            let pk = (k - 1) / 2;
            let parent = self.heap[pk];
            if v >= parent {
                break;
            }
            self.heap[pk] = v;
            self.heap[k] = parent;
            k = pk;
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let lk = self.heap.len() - 1;
        let min = self.heap[0];
        self.heap[0] = self.heap[lk];
        self.heap.pop();
        let mut k = 0;
        loop {
            let li = 2 * k + 1;
            let ri = 2 * k + 2;
            // li < ri && ri < si => li < si
            let si = if ri < lk && self.heap[ri] <= self.heap[li] {
                ri
            } else {
                li
            };
            if si >= lk || self.heap[k] < self.heap[si] {
                break;
            }
            if si < lk && self.heap[k] > self.heap[si] {
                self.heap.swap(k, si);
                k = si;
            }
        }
        Some(min)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WeightedVertex(pub Vertex, pub Weight);

impl PartialOrd for WeightedVertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl PartialEq for WeightedVertex {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

fn main() {
    let edges = [
        Edge::new((0, 1), 1),
        Edge::new((0, 2), 1),
        Edge::new((1, 3), 1),
    ];
    let graph = Graph::<4>::new(&edges);
    graph.dijkstra(0);
    println!("graph = {graph:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap0() {
        let mut heap = Heap::<Vertex>::new();
        assert!(heap.is_empty());
        assert!(heap.pop_min().is_none());
    }

    #[test]
    fn heap1() {
        let mut heap = Heap::new();
        heap.insert(1);
        assert_eq!(heap.pop_min(), Some(1));
    }

    #[test]
    fn heap2() {
        let mut heap = Heap::new();
        heap.insert(1);
        heap.insert(2);
        assert_eq!(heap.pop_min(), Some(1));
        assert_eq!(heap.pop_min(), Some(2));
        assert!(heap.pop_min().is_none());
    }

    #[test]
    fn heap2r() {
        let mut heap = Heap::new();
        heap.insert(2);
        heap.insert(1);
        assert_eq!(heap.pop_min(), Some(1));
        assert_eq!(heap.pop_min(), Some(2));
        assert!(heap.pop_min().is_none());
    }

    #[test]
    fn heap_many() {
        let mut heap = Heap::new();
        heap.insert(4);
        heap.insert(5);
        heap.insert(2);
        heap.insert(10);
        heap.insert(1);
        assert_eq!(heap.pop_min(), Some(1));
        assert_eq!(heap.pop_min(), Some(2));
        assert_eq!(heap.pop_min(), Some(4));
        assert_eq!(heap.pop_min(), Some(5));
        assert_eq!(heap.pop_min(), Some(10));
        assert!(heap.pop_min().is_none());
    }

    #[test]
    fn heap_many_ordered() {
        let mut heap = Heap::new();
        heap.insert(1);
        heap.insert(2);
        heap.insert(4);
        heap.insert(5);
        heap.insert(10);
        assert_eq!(heap.pop_min(), Some(1));
        assert_eq!(heap.pop_min(), Some(2));
        assert_eq!(heap.pop_min(), Some(4));
        assert_eq!(heap.pop_min(), Some(5));
        assert_eq!(heap.pop_min(), Some(10));
        assert!(heap.pop_min().is_none());
    }

    #[test]
    fn heap_priority_queue() {
        let mut heap = Heap::new();
        heap.insert(WeightedVertex(1, 5));
        heap.insert(WeightedVertex(2, 1));
        heap.insert(WeightedVertex(3, 4));
        assert_eq!(heap.pop_min(), Some(WeightedVertex(2, 1)));
        assert_eq!(heap.pop_min(), Some(WeightedVertex(3, 4)));
        assert_eq!(heap.pop_min(), Some(WeightedVertex(1, 5)));
        assert!(heap.is_empty());
    }
}
