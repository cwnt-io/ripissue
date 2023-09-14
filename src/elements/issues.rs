use std::{
    io::{Write, stdout, BufWriter},
    path::PathBuf,
};

use anyhow::Result;

use crate::{
    helpers::{slug, write_file},
    properties::statuses::Status,
    properties::tags::Tag,
};

use super::Element;

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
