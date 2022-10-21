use std::{
    path,
    env
};
use dirty_comments::api as dc_api;

fn main() {
    let storage = dc_api::db::Storage::new(path::PathBuf::from(env::current_dir().unwrap()));
    dc_api::remove_all(&storage);

}