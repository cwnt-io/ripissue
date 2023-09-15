use std::{
    io::{Write, stdout, BufWriter},
    path::PathBuf, collections::BTreeMap,
};

use anyhow::{Result, bail};

use crate::{
    helpers::{slug, write_file, traverse_dirs, get_elem_from_path},
    properties::statuses::Status,
    properties::tags::Tag,
};

use super::{Element, Elements};

#[derive(Debug)]
pub struct Issue {
    id: String,
    status: Option<Status>,
    tags: Option<Vec<Tag>>,
    path: PathBuf,
}

impl Element for Issue {
    type Item = Issue;

    fn new(name: &str) -> Self {
        let id = slug(&name);
        let mut path = Self::base_path();
        path.push(&id);
        Self {
            id,
            path,
            status: None,
            tags: None,
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn elem() -> &'static str {
        "issues"
    }

    fn write(&self) -> Result<()> {
        let (id, path, elem) = (self.id(), self.path(), Self::elem().to_uppercase());
        let content = format!("# {} ({})", &id, &elem);
        write_file(&path, "description.md", Some(&content))?;
        self.write_status()?;
        self.write_tags()?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} created.", Self::elem().to_uppercase(), self.id())?;
        Ok(())
    }

    // PATHS
    fn path(&self) -> &PathBuf {
        &self.path
    }
    fn set_path(&mut self, path: &PathBuf) {
        self.path = path.clone();
    }

    // STATUSES

    fn status(&self) -> &Option<Status> {
        &self.status
    }

    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }

    // TAGS
    fn tags(&self) -> &Option<Vec<Tag>> {
        &self.tags
    }

    fn set_tags(&mut self, tags: Option<Vec<Tag>>) {
        self.tags = tags;
    }

}

#[derive(Debug)]
pub struct Issues(BTreeMap<String, Issue>);

impl Elements for Issues {
    type Item = Issue;

    fn new() -> Self {
        Issues(BTreeMap::new())
    }

    fn add(&mut self, item: Self::Item) -> Result<()> {
        let id = item.id();
        if self.0.insert(id.clone(),item).is_some() {
            let elem = Self::Item::elem();
            bail!("{} #{} already exists.", elem.to_uppercase(), id);
        }
        Ok(())
    }

    fn update_all(&mut self) -> Result<()> {
        let mut all_elems = vec![];
        for p in Self::Item::base_path_all().iter() {
            let vec_elems = traverse_dirs(p);
            all_elems.extend(vec_elems);
        }
        for e in all_elems.iter() {
            let e_str = e.to_str().unwrap();
            let mut issue = Self::Item::raw(e_str)?;
            let vec_tags = Tag::vec_tags_from_files(&issue.tags_path());
            issue.set_tags(vec_tags);
            let status = Status::status_from_files(&issue.status_path())?;
            issue.set_status(status);
            // let elem = get_elem_from_path(elem_raw)?;
            self.add(issue)?;
        }
        Ok(())
    }

}


