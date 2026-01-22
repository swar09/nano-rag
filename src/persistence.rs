use core::hash;
use std::{fs::File, path::PathBuf};
use std::fs::{self, create_dir};
use std::io::Write;
use rayon::str::Bytes;
use rkyv::{deserialize, Deserialize, rancor::Error, Archive, Serialize, from_bytes};
use crate::HNSW;




pub struct PhotonDB {
    hnsw: HNSW,
    path: PathBuf,

}


impl PhotonDB {
    // fn new() -> Self {todo!()}
    fn load_or_create( path: PathBuf, max_elements: usize, dim: usize) -> PhotonDB {
        if path.exists() {
            println!("File exists !");
            println!("Loading from file! ");
            let bytes =  fs::read(&path).expect("Failed to read file ");

            let mut hnsw  = rkyv::from_bytes::<HNSW, Error>(&bytes).unwrap();
            return PhotonDB { hnsw , path};
        } else {
            return PhotonDB { hnsw: HNSW::new(max_elements, dim), path };
        }
        
    }
    
    fn save(&self) {
        // let bytes = rkyv::from_bytes::<Error>(&self.hnsw).unwrap();
        
    }
}



// let mut file = File::create("/home/eleven/Rust/projects-jan/photon/src/database.pho");
    //     file.expect("REASON").write_all(&_bytes);

    // let bytes = fs::read("/home/eleven/Rust/projects-jan/photon/src/database.pho")
    // .expect("Failed to read bincode file");
    // let mut deserialized = from_bytes::<HNSW, Error>(&bytes).unwrap();
    // println!("{:?}", deserialized.insert(4, 4, 6, 5, 0.5));
    
    // println!("{:?}", deserialized.layers.upper_layers.len());


