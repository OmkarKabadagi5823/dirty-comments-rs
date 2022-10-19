use serde::{Serialize, Deserialize};

use crate::core::types::Comment;
use crate::core::components::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnidedComment {
    pub comment: Comment,
}

impl UnidedComment {
    pub fn new(comment: Comment) -> Self {
        UnidedComment {
            comment,
        }
    }
}
    

impl Component for UnidedComment {
    fn is_ided(&self) -> bool {
        false
    }

    fn get_id(&self) -> Option<String> {
        None
    }

    fn fmt(&self) -> String {
        format!("{:?}", self)
    }
}