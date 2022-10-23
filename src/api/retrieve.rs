use std::{
    path::{Path, PathBuf}, 
    fs::{OpenOptions, self}, 
    io::{prelude::*, BufReader},
};

use crate::{
    parser,
    core::{
        components::{Component, CommentStamped},
        utils::*
    },
    api::db::CommentDB,
    dir
};

pub fn retrieve_all(comment_db: CommentDB) {
    for entry in comment_db.iter() {
        let (file_path_rel_proj, _) = entry.unwrap();
        
        let file_path_rel_proj = std::str::from_utf8(&file_path_rel_proj).unwrap();
        let file_path = comment_db.proj_dir().join(file_path_rel_proj);

        let comment_vec = comment_db.get(&file_path).unwrap();

        insert(&file_path, comment_vec);
    }
}

fn insert(file_path: &PathBuf, comment_vec: Vec<CommentStamped>) {
    let tmp_file_path = String::from(dir::TMP_FILE_PATH);

    filter_contents_into_tmp(&file_path, &tmp_file_path, comment_vec);
    fs::copy(&tmp_file_path, &file_path).unwrap();
}

fn filter_contents_into_tmp<P, Q>(
    file_path: P, 
    tmp_file_path: Q, 
    comment_vec: Vec<CommentStamped>,
) 
where
    P: AsRef<Path>,
    Q: AsRef<Path> 
{
    let comment_map = comment_vec_to_hashmap(comment_vec);
    let components = match parser::parse(&file_path) {
        Ok(components) => components,
        Err(_) => return,
    };

    let file = OpenOptions::new()
        .read(true)
        .open(&file_path).unwrap();

    let mut tmp_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&tmp_file_path).unwrap();

    let mut reader = BufReader::new(&file);
    let mut buf = String::new();

    let mut line_number = 0;

    for component in components.iter() {
        if component.has_id() && !component.has_text() {
            while line_number < component.text_start().unwrap() {
                reader.read_line(&mut buf).unwrap();
                tmp_file.write_all(buf.as_bytes()).unwrap();
                buf.clear();
                line_number += 1;
            }
            
            let comment = comment_map.get(&component.id().unwrap().clone()).unwrap();
            tmp_file.write_all(comment.text().unwrap().as_bytes()).unwrap();
            
            reader.read_line(&mut buf).unwrap();
            buf.clear();
            line_number += 1;
        }
    }

    while reader.read_line(&mut buf).unwrap() > 0 {
        tmp_file.write_all(buf.as_bytes()).unwrap();
        buf.clear();
    }
}
