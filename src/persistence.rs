// As of now implementing the Ram only solution and i will implement complete disk zero copy persistance when completed
// Os internals , how database works ?? watch some tuts .

use crate::HNSW;
// use memmap2::*;
use rkyv::rancor::Error;
// use rkyv::Archive;
use std::io::Write;
// use std::path::Path;
use std::io::Read;
use std::{fs::File, path::PathBuf};

const DB_NAME: &str = "main_hnsw_database.pho";

#[derive(Debug)]
pub struct PhotonDB {
    pub hnsw: HNSW,
    pub dim: usize,
    pub path: PathBuf,
}

impl PhotonDB {
    pub fn save(&self) -> Result<bool, String> {
        let mut file = File::create(&self.path).expect("Failed to create db file");
        // let mut file = File::open(&self.path).unwrap();
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&self.hnsw).unwrap();
        file.write_all(&bytes).expect("Error in writing");
        Ok(true)
    }

    pub fn load(path: PathBuf, dim: usize) -> Result<PhotonDB, String> {
        let dir_path = path.parent().unwrap();
        let db_path = dir_path.join(DB_NAME);
        if db_path.exists() {
            let mut file = File::open(&db_path).unwrap();
            // let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&player).unwrap();
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).expect("Failed to read file");

            let hnsw = rkyv::from_bytes::<HNSW, Error>(&bytes).unwrap();

            Ok(PhotonDB {
                hnsw,
                dim,
                path: db_path,
            })
        } else {
            // println!("Error: {:?}", db_path);
            Err("Database Curpted".to_string())
        }
    }

    pub fn create(path: PathBuf, max_elements: usize, dim: usize) -> Result<PhotonDB, String> {
        let dir_path = path.parent().unwrap();
        let db_path = dir_path.join(DB_NAME);

        Ok(PhotonDB {
            hnsw: HNSW::new(max_elements, dim),
            dim,
            path: db_path,
        })
    }

    pub fn add(&mut self, vec: &[f32]) {
        let id = self.hnsw.vectors.insert(vec);
        let m = self.hnsw.m;
        let m_max = m;
        let ef_construction = self.hnsw.ef_construction;
        let m_l = 1.0 / (m as f32).ln();

        self.hnsw.insert(id, m, m_max, ef_construction, m_l);
    }
}
