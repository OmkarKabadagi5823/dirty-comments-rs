use std::{
    path,
    io::*
};
use rocksdb;
use sha1::{Sha1, Digest};

use crate::{
    dir, 
    core::components::{Component, CommentStamped},
    core::types::Comment,
    core::{utils::*}
};

pub struct CommentDB {
    db: rocksdb::DB,
    _db_path: path::PathBuf,
    proj_dir: path::PathBuf,
    blob_dir: path::PathBuf
}

impl CommentDB {
    pub fn new(proj_dir: &path::PathBuf) -> CommentDB {
        let db_path = proj_dir.join(dir::DC_DIR_PATH).join("db");
        let blob_dir = proj_dir.join(dir::OBJECTS_PATH);
        let db = rocksdb::DB::open_default(&db_path).unwrap();

        CommentDB {
            db,
            _db_path: db_path,
            proj_dir: proj_dir.clone(),
            blob_dir
        }
    }

    pub fn proj_dir(&self) -> &path::PathBuf {
        &self.proj_dir
    }

    pub fn put(&self, file_path: &path::PathBuf, component_vec: &Vec<Box<dyn Component>>) {
        let blob = self.db.get(file_path.to_str().unwrap()).unwrap();
        let mut compressed = Vec::new();
        let comment_vec = component_vec_to_comment(&component_vec);
        if comment_vec.is_empty() {
            return;
        }

        if blob.is_none() {
                compressed = CommentDB::serialize_and_compress(comment_vec);
        } else if let Some(old_blob) = blob {
            let hash = std::str::from_utf8(&old_blob).unwrap();
            let mut bin_file = std::fs::File::options()
            .read(true)
            .open(self.blob_dir.join(hash))
            .unwrap();

            let mut old_compressed = Vec::new();
            bin_file.read_to_end(&mut old_compressed).unwrap();
            let old_comment_vec = CommentDB::decompress_and_deserialize(&old_compressed);
            let merged_comment_vec = CommentDB::merge(old_comment_vec, component_vec);

            compressed = CommentDB::serialize_and_compress(merged_comment_vec);
        }

        if compressed.len() > 0 {
            let hash = CommentDB::hash(&compressed);

            let mut bin_file = std::fs::File::options()
                .create_new(true)
                .write(true)
                .open(self.blob_dir.join(&hash))
                .unwrap();

            bin_file.write_all(&compressed).unwrap();
            self.db.put(file_path.display().to_string(), hash).unwrap();
        }
    }

    pub fn get(&self, file_path: &path::PathBuf) -> Option<Vec<CommentStamped>> {
        let blob = self.db.get(file_path.to_str().unwrap()).unwrap();

        if blob.is_none() {
            return None;
        }

        let hash = blob.unwrap();
        let hash = std::str::from_utf8(&hash).unwrap();
            
        let mut bin_file = std::fs::File::options()
            .read(true)
            .open(self.blob_dir.join(hash))
            .unwrap();

        let mut compressed = Vec::new();
        bin_file.read_to_end(&mut compressed).unwrap();

        Some(CommentDB::decompress_and_deserialize(&compressed))
    }

    pub fn iter(&self) -> rocksdb::DBIterator {
        self.db.iterator(rocksdb::IteratorMode::Start, )
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

    fn merge(old_comment_vec: Vec<CommentStamped>, new_component_vec: &Vec<Box<dyn Component>>) -> Vec<CommentStamped> {
        let mut merged_comment_vec = Vec::new();
        let old_comment_map = comment_vec_to_hashmap(old_comment_vec);

        for component in new_component_vec.iter() {
            if component.has_id() && !component.has_text() {
                let id = component.id().unwrap();
                let comment = old_comment_map.get(id).unwrap();
                merged_comment_vec.push(CommentStamped::new(
                    Some(id.clone()),
                    Comment::new(
                        comment.text_start().unwrap(),
                        comment.text_end().unwrap(),
                        comment.text().unwrap().clone()
                    )
                ));
            } else if component.has_id() && component.has_text() {
                merged_comment_vec.push(CommentStamped::new(
                    Some(component.id().unwrap().clone()),
                    Comment::new(
                        component.text_start().unwrap(),
                        component.text_end().unwrap(),
                        component.text().unwrap().clone()
                    )
                ));
            }
        }

        merged_comment_vec
    }
}
    