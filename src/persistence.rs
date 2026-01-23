use crate::HNSW;
use core::hash;
use rayon::str::Bytes;
use rkyv::{Archive, Deserialize, Serialize, deserialize, from_bytes, rancor::Error};
use std::fs::{self, create_dir};
use std::io::Write;
use std::{fs::File, path::PathBuf};

pub struct PhotonDB {
    hnsw: HNSW,
    dim: usize,
    path: PathBuf,
}

impl PhotonDB {
    // fn new() -> Self {todo!()}

    pub fn load_or_create(path: PathBuf, max_elements: usize, dim: usize) -> PhotonDB {
        if path.exists() {
            println!("File exists !");
            println!("Loading from file! ");
            let bytes = fs::read(&path).expect("Failed to read file ");

            let mut hnsw = rkyv::from_bytes::<HNSW, Error>(&bytes).unwrap();
            return PhotonDB { hnsw, dim, path };
        } else {
            return PhotonDB {
                hnsw: HNSW::new(max_elements, dim),
                dim,
                path,
            };
        }
    }

    pub fn save(&self) {
        let bytes = rkyv::to_bytes::<Error>(&self.hnsw).unwrap();

        let mut file = File::create(&self.path)
            .expect("Failed to create temp db file ");
        fs::write(&self.path, bytes).expect("Error in creating db file ");
        // fs::rename("database.pho.tmp", "database.pho")
    }


}
