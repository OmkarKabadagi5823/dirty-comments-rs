use std::collections::HashMap;

use crate::core::{
    components::{
        Component,
        CommentStamped
    },
    types::Comment 
};

pub fn make_all_components_ided(components: &mut Vec<Box<dyn Component>>) {
    for component in components.iter_mut() {
        if component.has_text() && !component.has_id() {
            component.set_id(uuid::Uuid::new_v4().simple().to_string());
        }
    }
}
pub fn component_vec_to_comment(component_vec: &Vec<Box<dyn Component>>) -> Vec<CommentStamped> {

    let mut comment_vec: Vec<CommentStamped> = Vec::new();

    for component in component_vec.iter().filter(|comp| 
        (comp.has_text() && comp.has_id())
    ) {
        if component.has_text() && component.has_id() {
            comment_vec.push(CommentStamped::new(
                Some(component.id().unwrap().clone()),
                Comment::new(
                    component.text_start().unwrap(),
                    component.text_end().unwrap(),
                    component.text().unwrap().clone()
                )
            ));
        }
    }

    comment_vec
}

pub fn comment_vec_to_hashmap(comment_vec: Vec<CommentStamped>) -> HashMap<String, CommentStamped> {
    let mut comment_map: HashMap<String, CommentStamped> = HashMap::new();

    for comment in comment_vec.iter() {
        comment_map.insert(comment.id().unwrap().clone(), CommentStamped::new(
            Some(comment.id().unwrap().clone()),
            Comment::new(
                comment.text_start().unwrap(),
                comment.text_end().unwrap(),
                comment.text().unwrap().clone()
            )
        ));
    }

    comment_map
}