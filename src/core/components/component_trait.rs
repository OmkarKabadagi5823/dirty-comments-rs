use core::fmt::Debug;

pub trait Component {
    fn has_id(&self) -> bool;
    fn id(&self) -> Option<&String>;
    fn set_id(&mut self, id: String) -> bool;
    fn has_text(&self) -> bool;
    fn text(&self) -> Option<&String>;
    fn text_start(&self) -> Option<usize>;
    fn text_end(&self) -> Option<usize>;
    fn fmt(&self) -> String;
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Component {{ {} }}", self.fmt())
    }
}