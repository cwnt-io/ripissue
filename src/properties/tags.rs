use std::path::PathBuf;

use anyhow::Result;

use crate::{helpers::{slug_tag, walkdir_into_iter, write_file}, elements::issue::ElemBase};

#[derive(Debug, Clone)]
pub struct Tag(Vec<String>);

impl Tag {

    pub fn new(s: &str) -> Self {
        let s = slug_tag(s);
        Self(s.split("-").map(|p|p.to_owned()).collect())
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

    pub fn vec_tags_from_vec_str(vec: &Vec<String>) -> Option<Vec<Self>> {
        let vec_tags: Vec<Tag> = vec.iter().map(|s| Tag::new(s)).collect();
        match vec_tags.is_empty() {
            true => None,
            false => Some(vec_tags),
        }
    }

}

pub trait TagTrait: ElemBase {
    fn tags(&self) -> &Option<Vec<Tag>>;
    fn set_tags(&mut self, tags: Option<Vec<Tag>>);
    fn tags_path(&self) -> PathBuf {
        let mut tags_path = self.epath().clone();
        tags_path.push("tags");
        tags_path
    }
    fn append_tags(&mut self, tags: &Vec<Tag>) {
        if tags.is_empty() {
            return;
        }
        let mut new_tags = self.tags().clone().unwrap_or(vec![]);
        for tag in tags {
            new_tags.push(tag.clone());
        }
        self.set_tags(Some(new_tags));
    }
    fn write_tags(&self) -> Result<()> {
        if let Some(tags) = self.tags() {
            let dir = &self.tags_path();
            for tag in tags.iter() {
                let file = &tag.to_str();
                write_file(dir,file,None)?;
            }
        }
        Ok(())
    }
}
