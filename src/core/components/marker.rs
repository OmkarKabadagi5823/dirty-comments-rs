use serde::{Serialize, Deserialize};

use crate::core::components::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    id: String,
    line: usize
}

impl Marker {
    pub fn new(id: String, line: usize) -> Self {
        Marker {
            id,
            line
        }
    }
}

impl Component for Marker {
    fn is_ided(&self) -> bool {
        true
    }

    fn get_id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn fmt(&self) -> String {
        format!("{:?}", self)
    }
}
