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
    fn has_id(&self) -> bool {
        true
    }

    fn id(&self) -> Option<&String> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: String) -> bool {
        self.id = id;
        true
    }

    fn has_text(&self) -> bool {
        false
    }

    fn text(&self) -> Option<&String> {
        None
    }

    fn text_start(&self) -> Option<usize> {
        None
    }

    fn text_end(&self) -> Option<usize> {
        None
    }

    fn fmt(&self) -> String {
        format!("{:?}", self)
    }
}
