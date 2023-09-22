use std::{path::PathBuf, slice::Iter};

use crate::helpers::{slug_tag, walkdir_into_iter};

#[derive(Debug, Clone)]
pub struct Tags(Vec<Tag>);

impl Tags {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn push(&mut self, tag: Tag) {
        self.0.push(tag);
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn has_any_tag_str(&self, tag: &Tag) -> bool {
        self.iter().any(|t| tag.iter().any(|s| t.contains(s)))
    }
    pub fn iter(&self) -> Iter<'_, Tag> {
        self.0.iter()
    }
    pub fn vec_tags_from_vec_str(vec: &[String]) -> Option<Self> {
        let vec_tags: Vec<Tag> = vec.iter().map(|s| Tag::new(s)).collect();
        match vec_tags.is_empty() {
            true => None,
            false => Some(Self(vec_tags)),
        }
    }
    pub fn vec_tags_from_files(path: &PathBuf) -> Option<Self> {
        let walk_iter = walkdir_into_iter(path);
        let vec_tags: Vec<Tag> = walk_iter
            .map(|e| Tag::new(e.file_name().to_str().unwrap()))
            .collect();
        match vec_tags.is_empty() {
            true => None,
            false => Some(Self(vec_tags)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tag(Vec<String>);

impl Tag {
    pub fn new(s: &str) -> Self {
        let s = slug_tag(s);
        Self(s.split('-').map(|p| p.to_owned()).collect())
    }
    pub fn to_str(&self) -> String {
        self.0.join("-")
    }
    pub fn iter(&self) -> Iter<'_, String> {
        self.0.iter()
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(&s.to_owned())
    }
}
