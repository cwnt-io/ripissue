use std::fs::{create_dir_all, rename};
use std::io::{stdout, BufWriter, Write};
use std::{path::PathBuf, fs::remove_dir_all};
use std::convert::AsRef;

use anyhow::{Result, bail};
use strum_macros::{AsRefStr, EnumString};

use crate::helpers::{write_file, git_commit};
use crate::{properties::{statuses::Status, tags::Tag}, helpers::{sys_base_path, slug, get_closed_dir}};

#[derive(AsRefStr, EnumString, Debug)]
pub enum ElemType {
    Issue,
    Sprint,
    Epic,
    Initiative,
}

impl ElemType {

    // PATHS
    fn base_path(&self) -> PathBuf {
        let mut base_path = sys_base_path();
        base_path.push(self.as_ref());
        base_path
    }
    fn base_path_closed(&self) -> PathBuf {
        let mut closed = get_closed_dir();
        closed.push(self.as_ref());
        closed
    }
    fn base_path_all(&self) -> Vec<PathBuf> {
        vec![
            self.base_path(),
            self.base_path_closed(),
        ]
    }
}

#[derive(Debug)]
pub struct Elem {
    id: String,
    status: Option<Status>,
    tags: Option<Vec<Tag>>,
    epath: PathBuf,
    etype: ElemType,
}

impl Elem {

    pub fn new(etype: ElemType, name: &str, status: Option<Status>, tags: Option<Vec<Tag>>) -> Self {
        let id = slug(&name);
        let mut epath = etype.base_path();
        epath.push(&id);
        Self {
            id,
            epath,
            status,
            tags,
            etype
        }
    }

    // GETTERS/SETTERS
    fn elem(&self) -> &ElemType {
        &self.etype
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn etype(&self) -> &ElemType {
        &self.etype
    }
    fn status(&self) -> &Option<Status> {
        &self.status
    }
    pub fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
    fn tags(&self) -> &Option<Vec<Tag>> {
        &self.tags
    }
    pub fn set_tags(&mut self, tags: Option<Vec<Tag>>) {
        self.tags = tags;
    }

    // PATHS
    fn epath(&self) -> PathBuf {
        let mut epath = self.elem().base_path();
        epath.push(self.id());
        epath
    }
    fn set_epath(&mut self, epath: PathBuf) {
        self.epath = epath;
    }
    fn epaths_all(&self) -> Vec<PathBuf> {
        vec![
            self.epath(),
            self.epath_closed(),
        ]
    }
    fn epath_closed(&self) -> PathBuf {
        let mut closed = self.elem().base_path_closed();
        closed.push(self.id());
        closed
    }
    pub fn status_path(&self) -> PathBuf {
        let mut status_path = self.epath();
        status_path.push("status");
        status_path
    }
    pub fn tags_path(&self) -> PathBuf {
        let mut tags_path = self.epath().clone();
        tags_path.push("tags");
        tags_path
    }

    // OPERATIONS
    pub fn write(&self) -> Result<()> {
        let (id, epath, etype) =
            (self.id(), self.epath(), self.etype());
        let content = format!("# {} ({})", id, etype.as_ref());
        write_file(&epath, "description.md", Some(&content))?;
        self.write_status()?;
        self.write_tags()?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} created.", etype.as_ref(), id)?;
        Ok(())
    }
    pub fn commit(&self, msg: &str) -> Result<()> {
        let files_to_add = self.epaths_all().into_iter().map(|p| {
            p.to_str().unwrap().to_owned()
        }).collect::<Vec<String>>();
        git_commit(Some(&files_to_add), msg)?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} commited to git.", self.etype().as_ref(), self.id())?;
        Ok(())
    }
    pub fn close(&self) -> Result<()> {
        let etype_str = self.etype().as_ref();
        let id = self.id();
        if self.epath_closed().is_dir() {
            bail!("{} #{} is already closed.", etype_str, &id);
        } else {
            create_dir_all(self.epath_closed())?;
        }
        rename(self.epath(), self.epath_closed())?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} closed.", etype_str, &id)?;
        Ok(())
    }
    pub fn delete(&self) -> Result<()> {
        let etype_str = self.etype().as_ref();
        let id = self.id();
        for p in self.epaths_all().iter() {
            if p.is_dir() {
                remove_dir_all(p)?;
            }
        }
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} deleted.", etype_str, &id)?;
        Ok(())
    }
    pub fn already_exists(&self) -> Result<()> {
        if self.epath().is_dir() || self.epath_closed().is_dir() {
            bail!("{} with Id #{} already exists.",
                  self.etype().as_ref(), &self.id());
        }
        Ok(())
    }
    pub fn update_path(&mut self) -> Result<()> {
        let path = self.epath().clone();
        let epath_closed = self.epath_closed();

        let elem_path = match (path.is_dir(), epath_closed.is_dir()) {
            (true,_) => path,
            (_,true) => epath_closed,
            _ => {
                bail!("Id \"{}\" doesn't match with any {}.",
                      self.id(),
                      self.etype().as_ref());
            },
        };
        self.set_epath(elem_path);
        Ok(())
    }

    // STATUSES
    pub fn write_status(&self) -> Result<()> {
        let status_path = self.status_path();
        if status_path.is_dir() {
            remove_dir_all(&status_path)?;
        }
        if let Some(status) = self.status() {
            let file = &status.as_ref();
            write_file(&status_path,file,None)?;
        }
        Ok(())
    }

    // TAGS
    pub fn append_tags(&mut self, tags: &Vec<Tag>) {
        if tags.is_empty() {
            return;
        }
        let mut new_tags = self.tags().clone().unwrap_or(vec![]);
        for tag in tags {
            new_tags.push(tag.clone());
        }
        self.set_tags(Some(new_tags));
    }
    pub fn write_tags(&self) -> Result<()> {
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
