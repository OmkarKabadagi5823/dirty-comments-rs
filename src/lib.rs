pub mod core;
pub mod api;

pub use crate::core::parser;

pub mod dir {
    pub const DC_DIR_PATH: &'static str  = ".dirty_comments";
    pub const INDEX_PATH: &'static str = ".dirty_comments/index";
    pub const OBJECTS_PATH: &'static str = ".dirty_comments/objects";
    pub const TMP_FILE_PATH: &'static str = "/tmp/dirty_comments_tmp";
}

pub mod tags {
    pub const OPENING: &'static str = "dco";
    pub const CLOSING: &'static str = "dcc";
    pub const MARKER: &'static str = "dcm";
}