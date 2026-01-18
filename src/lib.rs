use ordered_float::OrderedFloat;
#[warn(unused)]
// #[warn(dead_code)]
use rayon::prelude::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

// const N

pub struct VectorStore {
    data: Vec<f32>,
    dim: usize,
}

impl VectorStore {
    fn new(n: usize, dim: usize) -> Self {
        Self {
            data: Vec::with_capacity(n * dim),
            dim,
        }
    }

    fn insert(&mut self, vec: &[f32]) -> usize {
        let id = self.data.len() / self.dim;
        self.data.extend_from_slice(vec);
        id
    }

    fn squared_distance(&self, v1_id: usize, v2_id: usize) -> f32 {
        let vec1 = &self.data[v1_id * self.dim..(v1_id + 1) * self.dim];
        let vec2 = &self.data[v2_id * self.dim..(v2_id + 1) * self.dim];
        vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum()
    }

    fn squared_distance_to_query(&self, v1_id: usize, query: &[f32]) -> f32 {
        let vec1 = &self.data[v1_id * self.dim..(v1_id + 1) * self.dim];
        vec1.iter()
            .zip(query.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum()
    }
}

pub struct GraphLayers {
    base_layer: Vec<Vec<usize>>,
    upper_layers: Vec<HashMap<usize, Vec<usize>>>,
}

struct Distibution {}

impl GraphLayers {
    fn add_neighbors(&mut self) {
        // TO-DO
    }

    fn add_edge(&mut self, node_id_1: usize, node_id_2: usize, layer: usize, d: bool) {
        // Check if thye both exsist in the same layer ? ! :(
        if layer > 0 {
            // Handle the upper layer addedge logic ! i GUESS WE DONT NEED THIS
            // self.upper_layers[layer-1].get(&node_id_1);
        } else {
            self.base_layer[node_id_1].push(node_id_2);
            if !d {
                self.base_layer[node_id_2].push(node_id_1);
            }
        }
    }

    fn insert_node(&mut self) -> usize {
        let node_id = self.base_layer.len(); // index + 1 
        self.base_layer.push(Vec::new());
        node_id
    }

    fn insert_node_upper_layers(&mut self, layer: usize, node_id: usize) {
        self.upper_layers[layer - 1].insert(node_id, Vec::new());
    }

    fn get_neighbors(&self, layer: usize, node_id: usize) -> &[usize] {
        if layer == 0 {
            return &self.base_layer[node_id];
        } else {
            if layer <= self.upper_layers.len() {
                match self.upper_layers[layer - 1].get(&node_id) {
                    Some(slice) => slice,
                    None => &[],
                }
            } else {
                return &[];
            }
        }
    }

    fn new() -> Self {
        // Distrubution of nodes in the upper layer by exponetioal decay probalistic function
        // Choose random nodes and promote them to upper layers
        todo!()
    }
}

pub struct HNSW {
    pub layers: GraphLayers,
    pub vectors: VectorStore,
    pub entry_point: Option<usize>,
    pub max_level: usize,
    pub ef_construction: usize,
    pub m: usize,
}

impl HNSW {
    pub fn insert(&mut self, q: &[f32], m: usize, m_max: usize, efConstruction: usize, m_l: f32) {
        todo!()
    }

    // greedy beam search
    pub fn search_layer(&self, q: &[f32], lc: usize) -> Vec<usize> {
        let ep = self.entry_point.expect("ENTRY POINT ERROR");
        let sq_dist = VectorStore::squared_distance_to_query(&self.vectors, ep, q);
        // Candidates is Min Que
        let mut candidates: BinaryHeap<Reverse<(OrderedFloat<f32>, usize)>> = BinaryHeap::new(); // (Dist , node_id)
        // Found Neighbors is Max Que
        let mut found_neighbours: BinaryHeap<(OrderedFloat<f32>, usize)> = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();

        visited.insert(ep);
        candidates.push(Reverse((OrderedFloat(sq_dist), ep)));
        found_neighbours.push((OrderedFloat(sq_dist), ep));

        while !candidates.is_empty() {
            let Reverse((OrderedFloat(dist_c), closest_candidate)) = candidates.pop().unwrap();

            let (OrderedFloat(dist_worst), furthest_element) = *found_neighbours.peek().unwrap();

            if dist_c > dist_worst {
                break;
            }

            for e in GraphLayers::get_neighbors(&self.layers, lc, closest_candidate) {
                if !visited.contains(e) {
                    let dist_e = VectorStore::squared_distance_to_query(&self.vectors, *e, q);
                    let dist_e_wrapped = OrderedFloat(dist_e);

                    let (OrderedFloat(current_worst_dist), _) = *found_neighbours.peek().unwrap();
                    visited.insert(*e);
                    if dist_e < current_worst_dist || found_neighbours.len() < self.ef_construction
                    {
                        candidates.push(Reverse((OrderedFloat(dist_e), *e)));
                        found_neighbours.push((OrderedFloat(dist_e), *e));
                        if found_neighbours.len() > self.ef_construction {
                            found_neighbours.pop();
                        }
                    }
                }
            }
        }
        found_neighbours.into_iter().map(|(_, idx)| idx).collect()
    }
}

#[derive(Debug)]
// pub struct Graph {
//     adj_list: Vec<Vec<usize>>, // size_t
//     vectors: Vec<Vec<f32>>,
// }

// #[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct ScoredNode {
    id: usize,
    score: f32,
}

impl PartialEq for ScoredNode {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for ScoredNode {}
impl PartialOrd for ScoredNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}
impl Ord for ScoredNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

// impl Graph {
//     pub fn new(n: usize, dim: usize) -> Graph {
//         // Initialize with random vectors for demo purposes
//         // let mut _rng = rand::rng();
//         let vectors: Vec<Vec<f32>> = (0..n)
//             .map(|_| (0..dim).map(|_| rand::random::<f32>()).collect())
//             .collect();

//         Graph {
//             adj_list: vec![Vec::new(); n],
//             vectors,
//         }
//     }

//     // Will add edge between nodes
//     pub fn add_edge(&mut self, u: usize, v: usize, d: bool) {
//         if d {
//             self.adj_list[u].push(v);
//         } else {
//             self.adj_list[u].push(v);
//             self.adj_list[v].push(u);
//         }
//         // println!("{} --> {}", u, v);
//     }

//     pub fn insert_hnsw(&mut self, u: usize, v: usize) {
//         // Step1 Find the actual nearest neighobours
//         // let nearest_n =
//         // add edge between them
//         // What about NAvigable > HNSW ??
//     }

//     pub fn distance(vec1: &[f32], vec2: &[f32]) -> f32 {
//         if vec1.len() != vec2.len() {
//             panic!(
//                 "Vector dimension mismatch: {} vs {}",
//                 vec1.len(),
//                 vec2.len()
//             );
//         }

//         vec1.iter()
//             .zip(vec2.iter())
//             .map(|(a, b)| (a - b).powi(2))
//             .sum::<f32>()
//             .sqrt()
//     }

//     pub fn brute_force_search(&self, query: &[f32]) -> Option<(usize, f32)> {
//         // rayon is used here because we are searching MANY vectors
//         self.vectors
//             .par_iter()
//             .enumerate()
//             .map(|(i, v)| (i, Graph::distance(v, query)))
//             .min_by(|(_, a), (_, b)| a.total_cmp(b))
//     }

//     pub fn greedy_search(&self, query: &[f32]) -> Option<usize> {
//         if self.vectors.is_empty() {
//             return None;
//         }

//         let mut current_node = rand::random_range(0..self.vectors.len());
//         let mut min_dist = Graph::distance(&self.vectors[current_node], query);

//         // Simple Greedy Descent
//         loop {
//             let mut best_neighbor = None;

//             // Check all neighbors of current node
//             for &neighbor in &self.adj_list[current_node] {
//                 let d = Graph::distance(&self.vectors[neighbor], query);
//                 if d < min_dist {
//                     min_dist = d;
//                     best_neighbor = Some(neighbor);
//                 }
//             }

//             match best_neighbor {
//                 Some(n) => current_node = n,
//                 None => break,
//             }
//         }

//         Some(current_node)
//     }
// }
