use core::fmt::Debug;

pub trait Component {
    fn is_ided(&self) -> bool;
    fn get_id(&self) -> Option<String>;
    fn fmt(&self) -> String;
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Component{{{}}}", self.fmt())
    }
}