use crate::HNSW;
use rkyv::{from_bytes, to_bytes, rancor::Error};
use std::fs::{self};
use std::path::PathBuf;

pub struct db {
    hnsw: HNSW,
    path: PathBuf,
}

impl db {
    pub fn new(path: String, _dim: usize, _max_elements: usize) -> Self {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            Self::load_from_path(path_buf)
        } else {
             let hnsw = HNSW::new(_max_elements, _dim);
             db { hnsw, path: path_buf }
        }
    }

    fn load_from_path(path: PathBuf) -> Self {
        let bytes = fs::read(&path).expect("Failed to read database file");
        let hnsw: HNSW = from_bytes::<HNSW, Error>(&bytes).expect("Failed to deserialize database");
        db { hnsw, path }
    }

    pub fn add(&mut self, vec: Vec<f32>) {
        if vec.len() != self.hnsw.vectors.dim {
            panic!("Vector dimension mismatch");
        }
        
    
        let id = self.hnsw.vectors.insert(&vec);

        
        let m = self.hnsw.m;
        let m_max = m; 
        let ef_construction = self.hnsw.ef_construction;
        let m_l = 1.0 / (m as f32).ln();

        self.hnsw.insert(id, m, m_max, ef_construction, m_l);
    }

    pub fn search(&self, query: Vec<f32>, k: usize) -> Vec<(f32, usize)> {
        
        let ef_search = 100; 
        self.hnsw.search(&query, k, ef_search)
    }

    // pub fn delete(&mut self) {
    
    // }

    // pub fn get(&self, _id: usize) {
        
    // }

    pub fn save(&self) {
        let bytes = to_bytes::<Error>(&self.hnsw).expect("Failed to serialize database");
        fs::write(&self.path, bytes).expect("Failed to write database file");
    }

    // pub fn load() {
        
    // }

    // pub fn export_vectors(&self) {
         
    // }

    // pub fn search_by_id(&self) {
         
    // }
    
    pub fn getneighbors(&self, layer: usize, node: usize) -> Vec<usize> {
        // GraphLayers fields are public in lib.rs so this is possible
        if layer == 0 {
            if node < self.hnsw.layers.base_layer.len() {
                self.hnsw.layers.base_layer[node].clone()
            } else {
                Vec::new()
            }
        } else {
            let method_layer_idx = layer - 1;
            if method_layer_idx < self.hnsw.layers.upper_layers.len() {
                self.hnsw.layers.upper_layers[method_layer_idx]
                    .get(&node)
                    .cloned()
                    .unwrap_or_default()
            } else {
                Vec::new()
            }
        }
    }

    // pub fn update_vector(&mut self) {
        
    // }

    // pub fn merge_vector(&self) {
        
    // }

    pub fn count(&self) -> usize {
        self.hnsw.layers.base_layer.len()
    }

    pub fn stats(&self) {
        println!("HNSW Stats:");
        println!("  Max Level: {}", self.hnsw.max_level);
        println!("  Current Max Layer: {}", self.hnsw.layers.upper_layers.len());
        println!("  Total Vectors (Base Layer Nodes): {}", self.hnsw.layers.base_layer.len());
        println!("  Entry Point: {:?}", self.hnsw.entry_point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_db_flow() {
        let path = "test_db.pho";
        if std::path::Path::new(path).exists() {
            fs::remove_file(path).unwrap();
        }

        let dim = 4;
        let max_elements = 100;
        let mut database = db::new(path.to_string(), dim, max_elements);

        let v1 = vec![1.0, 1.0, 1.0, 1.0];
        let v2 = vec![2.0, 2.0, 2.0, 2.0];
        let v3 = vec![1.1, 1.1, 1.1, 1.1]; 

        database.add(v1.clone());
        database.add(v2.clone());
        database.add(v3.clone());

        assert_eq!(database.count(), 3);

        let query = vec![1.0, 1.0, 1.0, 1.0];
        let results = database.search(query, 2);
        
        assert!(results.len() > 0);
        
        assert_eq!(results[0].0, 0.0);

        database.save();
        
        let loaded_db = db::new(path.to_string(), dim, max_elements);
        assert_eq!(loaded_db.count(), 3);
        
        fs::remove_file(path).unwrap();
    }
}