use std::{
    path,
    env
};
use clap::{Parser};

use dirty_comments::api as dc_api;

#[derive(Parser, Debug)]
enum Cli {
    #[clap(version = "0.1.0")]
    Init,
    Remove ,
    Retrieve,
}

fn main() {

    let args = Cli::parse();

    match args {
        Cli::Init {} => dc_api::init::init(),
        Cli::Remove {} => {
            let comment_db = dc_api::db::CommentDB::new(
                &path::PathBuf::from(env::current_dir().unwrap())
            );
            
            dc_api::remove::remove_all(&comment_db)
        },
        Cli::Retrieve {} => {
            let comment_db = dc_api::db::CommentDB::new(
                &path::PathBuf::from(env::current_dir().unwrap())
            );

            dc_api::retrieve::retrieve_all(comment_db)
        },
    }
}