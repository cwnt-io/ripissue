use std::{
    path::PathBuf,
    io::{Write, stdout, BufWriter},
};

use anyhow::Result;

use crate::{
    helpers::{
        slug,
        sys_base_path,
        get_file_name
    },
    properties::statuses::{
        Status,
    },
    properties::tags::{
        Tags,
    }
};

use super::{Element, ElementPath};

#[derive(Debug)]
pub struct Issue {
    id: String,
    status: Option<Status>,
    tags: Option<Tags>,
}

impl Element for Issue {
    type Item = Issue;

    fn new(name: &str) -> Self {
        Self {
            id: slug(&name),
            status: None,
            tags: None,
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn base_path() -> PathBuf {
        let mut base_path = sys_base_path();
        base_path.push("issues");
        base_path
    }

    fn write(&self) -> Result<()> {
        self.write_basic_files()?;
        self.write_status()?;
        self.write_tags()?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} created.", Self::elem().to_uppercase(), self.id())?;
        Ok(())
    }

    fn from_path(elem_path: &ElementPath) -> Result<Self> {
        let mut issue = Issue::new(&elem_path.id());
        issue.set_status_from_files()?;
        issue.set_tags_from_files()?;
        Ok(issue)
    }

    fn status(&self) -> Option<Status> {
        self.status
    }

    fn set_status(&mut self, status: Option<Status>) {
        self.status = status
    }

    fn tags(&self) -> Option<Tags> {
        self.tags.clone()
    }

    fn set_tags(&mut self, tags: Option<Tags>) {
        self.tags = tags
    }

}
