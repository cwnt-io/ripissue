use std::{
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;

use crate::helpers::write_file;

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

    pub fn base_path(elem_path: &PathBuf) -> PathBuf {
        let mut base_path = elem_path.clone();
        base_path.push("tags");
        base_path
    }

    pub fn write(&self, elem_path: &PathBuf) -> Result<()> {
        let path = Tags::base_path(elem_path);
        for tag in self.0.iter() {
            write_file(&path, &tag.to_str(), None)?;
        }
        Ok(())
    }

    // pub fn get(issue_id: &str) -> Result<Option<Self>> {
    //     let mut path = Issue::base_path();
    //     path.push(&issue_id);
    //     path.push(&Tags::base_path());
    //     let tags_vec = WalkDir::new(path)
    //                 .min_depth(1)
    //                 .into_iter()
    //                 .flatten()
    //                 .map(|e| {
    //                     let p = e.path().to_path_buf();
    //                     Tag::new(&get_file_name(&p))
    //                 }).collect::<Vec<Tag>>();
    //     if tags_vec.is_empty() {
    //         Ok(None)
    //     } else {
    //         Ok(Some(Self(tags_vec)))
    //     }
    // }

}
