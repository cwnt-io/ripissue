use std::{
    path::PathBuf,
    str::FromStr,
    io::{Write, stdout, BufWriter},
};

use anyhow::Result;

use crate::{elements::{statuses::Status, tags::Tags}, helpers::{slug, sys_base_path}};

use super::Element;

pub struct Issue {
    id: String,
    status: Option<Status>,
    tags: Option<Tags>,
}

impl Issue {

    fn status(&self) -> Option<Status> {
        self.status
    }

    fn tags(&self) -> Option<Tags> {
        self.tags.clone()
    }

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
        if let Some(tags) = self.tags() {
            tags.write(&self.path())?;
        }
        if let Some(status) = self.status() {
            status.write(&self.path())?;
        }
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} created.", Self::elem().to_uppercase(), self.id())?;
        Ok(())
    }

}
