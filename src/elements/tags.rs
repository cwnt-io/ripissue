use std::{
    path::PathBuf,
    str::FromStr,
};

use walkdir::WalkDir;
use anyhow::Result;

use crate::helpers::get_file_name;

use super::issues::Issue;

#[derive(Debug, Clone)]
pub struct Tag(Vec<String>);

impl Tag {

    pub fn new(s: &str) -> Self {
        Self(s.split("-").map(|p|p.to_owned()).collect())
    }

    pub fn to_str(&self) -> String {
        self.0.join("-")
    }

}

#[derive(Debug, Clone)]
pub struct Tags(Vec<Tag>);

impl Tags {

    pub fn base_path() -> PathBuf {
        PathBuf::from_str("tags").unwrap()
    }

    pub fn get(issue_id: &str) -> Result<Option<Self>> {
        let mut path = Issue::base_path();
        path.push(&issue_id);
        path.push(&Tags::base_path());
        let tags_vec = WalkDir::new(path)
                    .min_depth(1)
                    .into_iter()
                    .flatten()
                    .map(|e| {
                        let p = e.path().to_path_buf();
                        Tag::new(&get_file_name(&p))
                    }).collect::<Vec<Tag>>();
        if tags_vec.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self(tags_vec)))
        }
    }

}
