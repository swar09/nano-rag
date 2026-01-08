use std::collections::HashMap;

pub struct VectorStore {
    pub data: HashMap<String, Vec<f32>>,
}