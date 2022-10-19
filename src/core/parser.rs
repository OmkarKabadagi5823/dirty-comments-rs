use std::{
    path:: Path, 
    fs::OpenOptions, 
    io::{
        prelude::*, 
        BufReader, Error
    }
};

use crate::core::{
    tags::TagRegex,
    types::Comment, 
    components::*
};

pub fn parse<P: AsRef<Path>>(file_path: P) -> Result<Vec<Box<dyn Component>>, Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(&file_path)?;


    let tag_regex = TagRegex::new();
    let mut comments: Vec<Box<dyn Component>> = Vec::new();
    
    let mut buf = String::new();
    let mut reader = BufReader::new(file);
    
    let mut line_number = 0;
    let mut in_comment = false;
    let mut comment_id: Option<String> = None;
    let mut start = 0;
   
    while reader.read_line(&mut buf)? > 0 {
        if in_comment {
            if tag_regex.contains_tag("CLOSING", &buf) {
                let end = line_number;
                
                if let Some(id) = comment_id.take() {
                    comments.push(Box::new(IdedComment::new(id, Comment::new(start, end, buf.clone()))));
                } else {
                    comments.push(Box::new(UnidedComment::new(Comment::new(start, end, buf.clone()))));
                }
        
                in_comment = false;
                buf.clear();
            }
        } else {
            if tag_regex.contains_tag("OPENING", &buf) {
                in_comment = true;
                start = line_number;
                match tag_regex.contains_id("OPENING", &buf) {
                    Some(id) => {
                        comment_id = Some(id);
                    },
                    None => {
                        comment_id = None;
                    }
                }
            } else if tag_regex.contains_tag("MARKER", &buf) {
                let id = tag_regex.contains_id("MARKER", &buf)
                    .expect("A uuid was expected but not found");
                comments.push(Box::new(Marker::new(id)));
                buf.clear();
            } else {
                buf.clear();
            }
        }

        line_number += 1;
    }


    for comment in comments.iter() {
        println!("{:#?}", comment.as_ref());
    }
    Ok(comments)
}