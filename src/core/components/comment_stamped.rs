use serde::{Serialize, Deserialize};

use crate::core::types::Comment;
use crate::core::components::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentStamped {
    id: Option<String>,
    pub comment: Comment,
}

impl CommentStamped {
    pub fn new(id: Option<String>, comment: Comment) -> Self {
        CommentStamped {
            id,
            comment,
        }
    }
    
}

impl Component for CommentStamped {
    fn has_id(&self) -> bool {
        true
    }

    fn id(&self) -> Option<&String> {
        match &self.id {
            Some(id) => Some(id),
            None => None,
        }
    }

    fn set_id(&mut self, id: String) -> bool {
        self.id = Some(id);
        true
    }

    fn has_text(&self) -> bool {
        true
    }

    fn text(&self) -> Option<&String> {
        Some(&self.comment.text)
    }

    fn text_start(&self) -> Option<usize> {
        Some(self.comment.start)
    }

    fn text_end(&self) -> Option<usize> {
        Some(self.comment.end)
    }

    fn fmt(&self) -> String {
        format!("{:?}", self)
    }
}