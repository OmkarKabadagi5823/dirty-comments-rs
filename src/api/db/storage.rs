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
    
    pub fn compare_and_add(&self, file_path: &path::PathBuf, comment_vec: Vec<CommentStamped>) {
        let compressed = Storage::serialize_and_compress(comment_vec);
        
        let hash = Storage::hash(&compressed);

        let mut bin_file = fs::File::options()
            .create_new(true)
            .write(true)
            .open(self.proj_dir.join(dir::OBJECTS_PATH).join(&hash))
            .unwrap();

        bin_file.write_all(&compressed).unwrap();

        self.update_index(file_path, &hash);
    }

    pub fn populate(&self) {
        let index_file = fs::File::options()
            .read(true)
            .open(self.proj_dir.join(dir::INDEX_PATH))
            .unwrap();

        let mut index_reader = BufReader::new(index_file);
        let mut buf = String::new();

        while index_reader.read_line(&mut buf).unwrap() > 0 {
            let mut index_entry = buf.trim().split_whitespace();
            let file_path_rel_proj = index_entry.next().unwrap();
            let hash = index_entry.next().unwrap();
            println!("{}: {}", file_path_rel_proj, hash);

            let bin_file = fs::File::options()
                .read(true)
                .open(self.proj_dir.join(dir::OBJECTS_PATH).join(&hash))
                .unwrap();

            let mut bin_reader = BufReader::new(bin_file);
            let mut compressed = Vec::new();
            bin_reader.read_to_end(&mut compressed).unwrap();

            let decompressed = Storage::decompress_and_deserialize(&compressed);
            println!("{:#?}", decompressed);

            buf.clear();
        }
    }

    fn serialize_and_compress(comment_vec: Vec<CommentStamped>) -> Vec<u8> {
        let serialized = bincode::serialize(&comment_vec).unwrap();
        zstd::encode_all(serialized.as_slice(), 3).unwrap()
    }

    fn decompress_and_deserialize(compressed: &Vec<u8>) -> Vec<CommentStamped> {
        let decompressed = zstd::decode_all(compressed.as_slice()).unwrap();
        let comment_vec = bincode::deserialize(&decompressed).unwrap();
        comment_vec
    }

    fn hash(data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn update_index(&self, file_path: &path::PathBuf, hash: &str) {
        let mut index_file = fs::File::options()
            .create(true)
            .append(true)
            .open(self.proj_dir.join(dir::INDEX_PATH))
            .unwrap();

        let file_path_rel_proj = file_path.strip_prefix(&self.proj_dir).unwrap();

        let index_entry = format!("{} {}\n", file_path_rel_proj.display(), hash);

        index_file.write_all(index_entry.as_bytes()).unwrap();
    }
}
