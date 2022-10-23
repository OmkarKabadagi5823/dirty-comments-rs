use std::collections::HashMap;
use regex::Regex;
use crate::tags;

pub struct TagRegex {
    regexes: HashMap<String, Regex>,
}

impl TagRegex {
    pub fn new() -> Self {
        let opening = Regex::new(&format!("!(?P<tag_ided>{})-(?P<uuid>[a-z, 0-9]+)|!(?P<tag>{})", tags::OPENING, tags::OPENING)).unwrap();
        let closing = Regex::new(&format!("!(?P<tag>{})", tags::CLOSING)).unwrap();
        let marker = Regex::new(&format!("!(?P<tag_ided>{})-(?P<uuid>[a-z, 0-9]+)|!(?P<tag>{})", tags::MARKER, tags::MARKER)).unwrap();
       
        let mut regexes = HashMap::new();
        regexes.insert(String::from("OPENING"), opening);
        regexes.insert(String::from("CLOSING"), closing);
        regexes.insert(String::from("MARKER"), marker);

        TagRegex {
            regexes: regexes
        }
    }

    pub fn contains_tag(&self, tag_type: &str, line: &str) -> bool {
        if let Some(_) = self.regexes.get(tag_type).unwrap().find(line) {
            true
        } else {
            false
        }
    }

    pub fn contains_id(&self, tag_type: &str, line: &str) -> Option<String> {
        if let Some(caps) = self.regexes.get(tag_type).unwrap().captures(line) {
            if let Some(uuid) = caps.name("uuid") {
                Some(String::from(uuid.as_str()))
            } else {
                None
            }
        } else {
            None
        }
    }
}