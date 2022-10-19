use serde::{Serialize, Deserialize};

use crate::core::types::Comment;
use crate::core::components::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdedComment {
    id: String,
    pub comment: Comment,
}

impl IdedComment {
    pub fn new(id: String, comment: Comment) -> Self {
        IdedComment {
            id,
            comment,
        }
    }
    
}

impl Component for IdedComment {
    fn is_ided(&self) -> bool {
        true
    }

    fn get_id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn fmt(&self) -> String {
        format!("IdedComment{{id: {}, comment: {:?}}}", self.id, self.comment)
    }
}