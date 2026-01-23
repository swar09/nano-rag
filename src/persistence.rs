use core::hash;
use std::{fs::File, path::PathBuf};
use std::fs::{self, create_dir};
use std::io::Write;
use rayon::str::Bytes;
use rkyv::{deserialize, Deserialize, rancor::Error, Archive, Serialize, from_bytes};
use crate::HNSW;




pub struct PhotonDB {
    hnsw: HNSW,
    dim: usize,
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
            return PhotonDB { hnsw , dim,  path};
        } else {
            return PhotonDB { hnsw: HNSW::new(max_elements, dim), dim,  path };
        }
        
    }
    
    fn save(&self) {
        // let bytes = rkyv::from_bytes::<Error>(&self.hnsw).unwrap();
    //     3. **`save(&self)`**
    // - **Serialize**: `rkyv::to_bytes::<Error>(&self.hnsw)`.
    // - **Atomic Write**:
    //     1. Create `database.pho.tmp`.
    //     2. Write bytes.
    //     3. `fs::rename("database.pho.tmp", "database.pho")`.
    // - This ensures you never corrupt the DB if the program crashes while saving.
    }

    
}



// let mut file = File::create("/home/eleven/Rust/projects-jan/photon/src/database.pho");
    //     file.expect("REASON").write_all(&_bytes);

    // let bytes = fs::read("/home/eleven/Rust/projects-jan/photon/src/database.pho")
    // .expect("Failed to read bincode file");
    // let mut deserialized = from_bytes::<HNSW, Error>(&bytes).unwrap();
    // println!("{:?}", deserialized.insert(4, 4, 6, 5, 0.5));
    
    // println!("{:?}", deserialized.layers.upper_layers.len());


