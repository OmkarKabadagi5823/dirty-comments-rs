use serde::{Serialize, Deserialize};

use crate::core::components::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    id: String
}

impl Marker {
    pub fn new(id: String) -> Self {
        Marker {
            id
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
        format!("Marker{{id: {}}}", self.id)
    }
}
