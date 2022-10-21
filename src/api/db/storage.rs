use std::{
    path,
    fs,
    io::*
};
use sha1::{Sha1, Digest};
use zstd;

use crate::{
    dir, 
    core::components::CommentStamped
};

pub struct Storage {
    proj_dir: path::PathBuf,
    dc_dir: path::PathBuf,
}

impl Storage {
    pub fn new(proj_dir: path::PathBuf) -> Self {
        Self {
            proj_dir: proj_dir.clone(),
            dc_dir: proj_dir.join(dir::DC_DIR_PATH).clone(),
        }
    }

    pub fn proj_dir(&self) -> &path::PathBuf {
        &self.proj_dir
    }
    
    pub fn dc_dir(&self) -> &path::PathBuf {
        &self.dc_dir
    }
    
    pub fn compare_and_add(&self, _file_path: &path::PathBuf, comment_vec: Vec<CommentStamped>) {
        let compressed = Storage::serialize_and_compress(comment_vec);
        
        let mut bin_file = fs::File::options()
            .create_new(true)
            .write(true)
            .open(self.proj_dir.join(dir::OBJECTS_PATH).join(Storage::hash(&compressed)))
            .unwrap();

        bin_file.write_all(&compressed).unwrap();
    }

    fn serialize_and_compress(comment_vec: Vec<CommentStamped>) -> Vec<u8> {
        let serialized = bincode::serialize(&comment_vec).unwrap();
        zstd::encode_all(serialized.as_slice(), 3).unwrap()
    }

    fn hash(data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
