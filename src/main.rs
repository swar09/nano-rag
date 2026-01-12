use rayon::prelude::*;
use std::cmp::Ordering;


const EPSILON: f32 = 1e-5;

#[derive(Debug)]
struct Graph {
    adj_list: Vec<Vec<usize>>,
    vectors: Vec<Vec<f32>>,
}


#[derive(Debug, Clone, Copy)]
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

impl Graph {
    fn new(n: usize, dim: usize) -> Graph {
        // Initialize with random vectors for demo purposes
        let mut rng = rand::rng();
        let vectors: Vec<Vec<f32>> = (0..n)
            .map(|_| (0..dim).map(|_| rand::random::<f32>()).collect())
            .collect();
        
        Graph {
            adj_list: vec![Vec::new(); n],
            vectors,
        }
    }

    
    fn distance(vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.len() != vec2.len() {
            panic!("Vector dimension mismatch: {} vs {}", vec1.len(), vec2.len());
        }
        
        vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    fn brute_force_search(&self, query: &[f32]) -> Option<(usize, f32)> {
        // Rayon IS used here because we are searching MANY vectors
        self.vectors
            .par_iter()
            .enumerate()
            .map(|(i, v)| (i, Graph::distance(v, query)))
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
    }

    fn greedy_search(&self, query: &[f32]) -> Option<usize> {
        if self.vectors.is_empty() { return None; }

        let mut current_node = rand::random_range(0..self.vectors.len());
        let mut min_dist = Graph::distance(&self.vectors[current_node], query);

        // Simple Greedy Descent
        loop {
            let mut best_neighbor = None;
            
            // Check all neighbors of current node
            for &neighbor in &self.adj_list[current_node] {
                let d = Graph::distance(&self.vectors[neighbor], query);
                if d < min_dist {
                    min_dist = d;
                    best_neighbor = Some(neighbor);
                }
            }

            match best_neighbor {
                Some(n) => current_node = n,
                None => break,
            }
        }
        
        Some(current_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_logic() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];
        
        let dist = Graph::distance(&vec1, &vec2);
        
        assert!((dist - 5.196152).abs() < EPSILON);
    }
}

fn main() {
    let graph = Graph::new(100, 10);
    println!("Graph created with 100 nodes");
}