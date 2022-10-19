use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub start: usize,
    pub end: usize,
    pub text: String
}

impl Comment {
    pub fn new(start: usize, end: usize, text: String) -> Comment {
        Comment {
            start,
            end,
            text
        }
    }
}