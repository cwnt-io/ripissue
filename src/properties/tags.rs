use std::{path::PathBuf, slice::Iter};

use crate::helpers::{slug_tag, walkdir_into_iter};

#[derive(Debug, Clone)]
pub struct Tag(Vec<String>);

impl Tag {

    pub fn new(s: &str) -> Self {
        let s = slug_tag(s);
        Self(s.split('-').map(|p|p.to_owned()).collect())
    }

    pub fn to_str(&self) -> String {
        self.0.join("-")
    }

    pub fn vec_tags_from_files(path: &PathBuf) -> Option<Vec<Self>> {
        let walk_iter = walkdir_into_iter(path);
        let vec_tags: Vec<Tag> = walk_iter
            .map(|e|  Tag::new(e.file_name().to_str().unwrap()) )
            .collect();
        match vec_tags.is_empty() {
            true => None,
            false => Some(vec_tags),
        }
    }

    pub fn vec_tags_from_vec_str(vec: &[String]) -> Option<Vec<Self>> {
        let vec_tags: Vec<Tag> = vec.iter().map(|s| Tag::new(s)).collect();
        match vec_tags.is_empty() {
            true => None,
            false => Some(vec_tags),
        }
    }

    pub fn iter(&self) -> Iter<'_, String> {
        self.0.iter()
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(&s.to_owned())
    }

}
