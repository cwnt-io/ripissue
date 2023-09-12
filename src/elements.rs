pub mod sprints;
pub mod issues;

use std::{
    path::PathBuf,
    io::{stdout, BufWriter, Write}, fs::rename,
};

use anyhow::{Result, bail};
use walkdir::WalkDir;

use crate::{helpers::{
    get_closed_dir,
    write_file,
    get_file_name,
    git_commit,
}, properties::{statuses::Status, tags::{Tags, Tag}}};

pub struct ElementPath {
    id: String,
    elem: String,
    full_path: PathBuf,
}

impl ElementPath {

    pub fn new(id: String, elem: String, full_path: PathBuf) -> Self {
        Self {id, elem, full_path}
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn elem(&self) -> String {
        self.elem.clone()
    }

    pub fn full_path(&self) -> PathBuf {
        self.full_path.clone()
    }

}

pub trait Element {
    type Item;

    fn new(name: &str) -> Self::Item;

    fn base_path() -> PathBuf;

    fn status_path(&self) -> PathBuf {
        let mut status_path = self.path();
        status_path.push("status");
        status_path
    }

    fn status(&self) -> Option<Status>;

    fn set_status(&mut self, status: Option<Status>);

    fn set_status_from_files(&mut self) -> Result<()> {
        let statuses: Vec<PathBuf> = WalkDir::new(self.status_path())
            .min_depth(1)
            .into_iter()
            .flatten()
            .map(|e| e.path().to_path_buf())
            .collect();
        let status = match statuses.len() {
            0 => None,
            1 => {
                let status_full_path = statuses.get(0).unwrap();
                let status_str = get_file_name(&status_full_path);
                Some(Status::from_str(&status_str)?)
            },
            _ => {
                let msg: Vec<String> = statuses.into_iter()
                    .map(|e| e.to_str().unwrap().to_owned())
                    .collect();
                bail!("Status can't be more than one. Found {}",
                      &msg.join(", "));
            },
        };
        self.set_status(status);
        Ok(())
    }

    fn write_status(&self) -> Result<()> {
        if let Some(status) = self.status() {
            let dir = &self.status_path();
            let file = &status.as_str();
            let content = None;
            write_file(dir,file,content)?;
        }
        Ok(())
    }

    fn tags_path(&self) -> PathBuf {
        let mut tags_path = self.path();
        tags_path.push("tags");
        tags_path
    }

    fn tags(&self) -> Option<Tags>;

    fn set_tags(&mut self, tags: Option<Tags>);

    fn set_tags_from_files(&mut self) -> Result<()> {
        let tags_vec: Vec<Tag> = WalkDir::new(self.tags_path())
            .min_depth(1)
            .into_iter()
            .flatten()
            .map(|e| {
                let tag_path = e.path().to_path_buf();
                Tag::new(&get_file_name(&tag_path))
            }).collect();
        let tags = match tags_vec.is_empty() {
            true => None,
            false => Some(Tags::new(tags_vec)),
        };
        self.set_tags(tags);
        Ok(())
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

    fn base_closed() -> PathBuf {
        let mut closed = get_closed_dir();
        closed.push(Self::elem());
        closed
    }

    fn path(&self) -> PathBuf {
        let mut path = Self::base_path();
        path.push(&self.id());
        path
    }

    fn closed_path(&self) -> PathBuf {
        let mut closed = Self::base_closed();
        closed.push(self.id());
        closed
    }

    fn id(&self) -> String;

    fn elem() -> String {
        get_file_name(&Self::base_path())
    }

    fn from_path(path: &ElementPath) -> Result<Self::Item>;

    fn get_path(path_or_id: &str) -> Result<ElementPath> {
        let vec: Vec<&str> = path_or_id.split("/").collect();
        let id = vec.last().unwrap();
        let mut path = Self::base_path();
        path.push(&id);
        let mut closed = Self::base_closed();
        closed.push(&id);

        let elem_path = match (path.is_dir(), closed.is_dir()) {
            (true,_) => path,
            (_,true) => closed,
            _ => {
                bail!("Input \"{}\" doesn't match with any {}.",
                      &path_or_id,
                      Self::elem().to_uppercase());
            },
        };

        Ok(ElementPath::new(id.to_string(), Self::elem(), elem_path))
    }

    fn already_exists(&self) -> Result<()> {
        if self.path().is_dir() || self.closed_path().is_dir() {
            bail!("{} with Id #{} already exists.",
                  Self::elem().to_uppercase(), &self.id());
        }
        Ok(())
    }

    fn write_basic_files(&self) -> Result<()> {
        let (id, path, elem) = (self.id(), self.path(), Self::elem().to_uppercase());
        let content = format!("# {} ({})", &id, &elem);
        write_file(&path, "description.md", Some(&content))?;
        Ok(())
    }

    fn write(&self) -> Result<()>;

    fn commit(&self, msg: &str) -> Result<()> {
        let path = self.path().to_str().unwrap().to_owned();
        let closed_path = self.closed_path().to_str().unwrap().to_owned();
        let files_to_add = [path, closed_path];
        git_commit(Some(&files_to_add), msg)?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} commited to git.", Self::elem().to_uppercase(), self.id())?;
        Ok(())
    }

    fn close(&self) -> Result<()> {
        let elem = Self::elem().to_uppercase();
        let id = self.id();
        if self.closed_path().is_dir() {
            bail!("{} #{} is already closed.", &elem, &id);
        }
        rename(self.path(), self.closed_path())?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} closed.", &elem, &id)?;
        Ok(())
    }

}
