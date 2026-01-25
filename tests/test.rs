use photon::HNSW;
use photon::persistence::PhotonDB;
use photon::VectorStore;
use rand::Rng;
use std::fs;
// use std::path::PathBuf;

fn generate_random_vector(dim: usize) -> Vec<f32> {
    let mut rng = rand::rng();
    (0..dim).map(|_| rng.random::<f32>()).collect()
}

#[test]
fn test_vector_store() {
    let dim = 4;
    let mut store = VectorStore::new(10, dim);
    let v1 = vec![1.0, 2.0, 3.0, 4.0];
    let v2 = vec![5.0, 6.0, 7.0, 8.0];

    let id1 = store.insert(&v1);
    let id2 = store.insert(&v2);

    assert_eq!(id1, 0);
    assert_eq!(id2, 1);
    assert_eq!(store.data.len(), 8);

    let dist = store.squared_distance(id1, id2);
    assert!((dist - 64.0).abs() < 1e-6);

    let query = vec![1.0, 2.0, 3.0, 4.0];
    let dist_q = store.squared_distance_to_query(id1, &query);
    assert!((dist_q - 0.0).abs() < 1e-6);
}

#[test]
fn test_hnsw_basic() {
    let dim = 128;
    let max_elements = 100;
    let mut hnsw = HNSW::new(max_elements, dim);
    let m = 16;
    let ef_construction = 64;
    let m_l = 1.0 / (m as f32).ln();

    let v1 = generate_random_vector(dim);
    let id = hnsw.vectors.insert(&v1);
    hnsw.insert(id, m, m, ef_construction, m_l);

    let results = hnsw.search(&v1, 1, 64);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1, id);
    assert!(results[0].0 < 1e-6);
}

#[test]
fn test_hnsw_recall() {
    let dim = 32;
    let n = 200;
    let mut hnsw = HNSW::new(n, dim);
    let m = 16;
    let ef_construction = 64;
    let m_l = 1.0 / (m as f32).ln();

    let _rng = rand::rng();
    let mut vectors = Vec::new();

    for _ in 0..n {
        let v = generate_random_vector(dim);
        let id = hnsw.vectors.insert(&v);
        hnsw.insert(id, m, m, ef_construction, m_l);
        vectors.push(v);
    }

    let k = 10;
    let query_count = 20;
    let mut correct_count = 0;

    for _ in 0..query_count {
        let query = generate_random_vector(dim);
        
        let bf_results = hnsw.brute_force_search(&query, k);
        let hnsw_results = hnsw.search(&query, k, 100);

        let bf_ids: Vec<usize> = bf_results.iter().map(|(_, id)| *id).collect();
        let hnsw_ids: Vec<usize> = hnsw_results.iter().map(|(_, id)| *id).collect();

        // Calculate intersection
        let mut found = 0;
        for id in &hnsw_ids {
            if bf_ids.contains(id) {
                found += 1;
            }
        }
        if found >= (k * 8 / 10) { // 80% recall per query threshold
             correct_count += 1;
        }
    }

    let recall = correct_count as f32 / query_count as f32;
    println!("Recall: {}", recall);
    assert!(recall >= 0.9, "Recall should be at least 90%");
}

#[test]
fn test_persistence() {
    // Setup temporary directory
    let temp_dir = std::env::temp_dir().join("photon_test_persistence");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).unwrap();
    }
    fs::create_dir(&temp_dir).unwrap();
    
    let db_path = temp_dir.join("test_db.pho"); 
    
    let dim = 4;
    let max_elements = 100;
    let mut db = PhotonDB::create(db_path.clone(), max_elements, dim).unwrap();

    let v1 = vec![1.0, 1.0, 1.0, 1.0];
    let v2 = vec![2.0, 2.0, 2.0, 2.0];

    db.add(&v1);
    db.add(&v2);

    assert!(db.save().unwrap());

    // Load
    let loaded_db = PhotonDB::load(db_path.clone(), dim).unwrap();
    
    assert_eq!(loaded_db.hnsw.vectors.data.len(), 8); // 2 vectors * 4 dim
    
    let results = loaded_db.hnsw.search(&v1, 1, 10);
    assert_eq!(results.len(), 1);
    
    // Cleanup
    fs::remove_dir_all(temp_dir).unwrap();
}
