use std::{
    path:: Path, 
    fs::{OpenOptions, self}, 
    io::{
        prelude::*, 
        BufReader, Error
    }
};

use crate::{
    parser, 
    core::{
        components::Component,
        utils::*
    }
};

const TMP_FILE_PATH: &str = "/tmp/dirty_comments_tmp";


pub fn remove<P: AsRef<Path>>(file_path: P) -> Result<(), Error> {
    let tmp_file_path = String::from(TMP_FILE_PATH);

    let mut components = parser::parse(&file_path).unwrap();
    make_all_components_ided(&mut components);

    filter_contents_into_tmp(&file_path, &tmp_file_path, &components);
    fs::copy(&tmp_file_path, &file_path)?;

    Ok(())
}

fn filter_contents_into_tmp<P, Q>(file_path: P, tmp_file_path: Q, components: &Vec<Box<dyn Component>>) 
where
    P: AsRef<Path>,
    Q: AsRef<Path> 
{
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
        if component.has_text() {
            while line_number < component.text_start().unwrap() && reader.read_line(&mut buf).unwrap() > 0 {
                tmp_file.write_all(buf.as_bytes()).unwrap();
                buf.clear();
                line_number += 1;
            }
    
            while line_number <= component.text_end().unwrap() && reader.read_line(&mut buf).unwrap() > 0 {
                buf.clear();
                line_number += 1;
            }
    

            let marker = format!("# !dcm-{}\n", component.id().expect("Component should have an id"));
            tmp_file.write_all(marker.as_bytes()).unwrap();
        }
    }

    while reader.read_line(&mut buf).unwrap() > 0 {
        tmp_file.write_all(buf.as_bytes()).unwrap();
        buf.clear();
    }
}