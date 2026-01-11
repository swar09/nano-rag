use std::{
    cmp::{max, min},
    f32,
};
use std::collections::VecDeque;

#[test]
fn unit() {
    let mut vec1 = Vec::with_capacity(10);
    let mut vec2 = Vec::with_capacity(10);
    vec1.resize(99, 1.45657);
    vec2.resize(99, 4.65757);

    let distance = Graph::distance(&vec1, &vec2); // pas the slice not the vec
    // println!("{}", distance);
    assert_eq!(distance, 31.849545);

    let modulus = Graph::modulus(&vec1);
    assert_eq!(modulus, 14.492692);
}

#[derive(Debug)]
struct Graph {
    adj_list: Vec<Vec<usize>>,
    vectors: Vec<Vec<f32>>,
}

struct Cosine {
    sim: f32,
    dist: f32,
}

impl Graph {
    fn new(n: usize) -> Graph {
        let mut adj_list = Vec::with_capacity(n);
        let mut vectors = Vec::with_capacity(n);
        for _ in 0..n {
            adj_list.push(Vec::new());
            vectors.push(Vec::<f32>::new());
        }
        return Graph { adj_list, vectors };
    }

    fn new_node() {} // do new node and test cases 

    fn add_edge(&mut self, u: usize, v: usize, d: bool) {
        if d {
            // Directed
            self.adj_list[u].push(v);
            println!("{} --> {}", u, v);
        } else {
            // Undirected
            self.adj_list[u].push(v);
            self.adj_list[v].push(u);
            println!("{} <--> {}", u, v);
        }
    }

    fn bfs_traversal(&self, s: usize) {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut vist: Vec<bool> = vec![false; self.adj_list.len()];

        que.push_back(s);
        vist[s] = true;

        while let Some(s) = que.pop_front() {
            for &v in &self.adj_list[s] {
                if !vist[v] {
                    vist[v] = true;
                    que.push_back(v);
                }
            }
        }
    }  
    // show the traversal logic properly here last time it was not visible

    fn distance(vec1: &[f32], vec2: &[f32]) -> f32 {
        let mut sum: f32 = 0.0;
        // Check for empty vector here first handlethe error properly
        // n and m ds vectors
        if !(vec1.len() == vec2.len()) {
            let len = max(vec1.len(), vec2.len());
            // vec1.resize(len, 0.0);
            // vec2.resize(len, 0.0);
        }
        for i in 0..vec1.len() {
            let diff = vec1[i] - vec2[i];
            sum = sum + diff * diff;
        }
        return sum.sqrt();
    }

    fn modulus(vec1: &[f32]) -> f32 {
        let mut sum: f32 = 0.0;
        for i in 0..vec1.len() {
            let diff = vec1[i];
            sum = sum + diff * diff;
        }
        return sum.sqrt();
    }

    fn brute_force_search(&self, query: &[f32]) -> &Vec<f32> {
        let (best_vec, min_dist) = self
            .vectors
            .iter()
            .map(|v| (v, Graph::distance(v, query)))
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap();

        best_vec
    }

    fn greedy_search() {}

    fn cosine(vec1: &[f32], vec2: &[f32]) -> Cosine {
        let mut dot_sum: f32 = 0.0;
        // Dot product
        if vec1.len() == vec2.len() {
            for i in 0..vec1.len() {
                dot_sum = dot_sum + (vec1[i] * vec2[i]);
            }
        }

        let a: f32 = Graph::modulus(vec1);
        let b: f32 = Graph::modulus(vec2);
        let sim: f32 = dot_sum / (a * b); // CosT
        let dist: f32 = 1.0 - sim; // sim + dist = 1
        return Cosine { sim, dist };
    }

    fn print(&self) {
        println!("{:?}", &self);
    }
}

fn main() {}
