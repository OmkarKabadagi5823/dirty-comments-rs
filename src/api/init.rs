use std::{
    fs,
    env
};

use crate::dir;

pub fn init() {
    let cwd = env::current_dir().expect("Could not find current directory");

    if let Err(e) = fs::create_dir(cwd.join(dir::DC_DIR_PATH)) {
        panic!("Could not create directory: {}", e)
    }

    fs::File::options().create_new(true).write(true).open(&cwd.join(dir::INDEX_PATH)).expect("Could not create index");
    
    fs::create_dir(&cwd.join(dir::OBJECTS_PATH)).expect("Could not create objects directory");
}