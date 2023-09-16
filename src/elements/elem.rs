use std::{path::PathBuf, io::{stdout, BufWriter, Write}, fs::{create_dir_all, rename, remove_dir_all}};

use anyhow::{Result, bail};

use crate::{helpers::{sys_base_path, get_closed_dir, git_commit, write_file}, properties::{statuses::StatusTrait, tags::TagTrait}};

pub trait ElemBase {
    fn new(name: &str) -> Self;
    fn id(&self) -> &str;
    fn stype(&self) -> &str;
    fn epath(&self) -> PathBuf {
        let mut epath = self.base_path();
        epath.push(self.id());
        epath
    }
    fn epaths_all(&self) -> Vec<PathBuf> {
        vec![
            self.epath(),
            self.epath_closed(),
        ]
    }
    fn epath_closed(&self) -> PathBuf {
        let mut closed = self.base_path_closed();
        closed.push(self.id());
        closed
    }
    fn base_path(&self) -> PathBuf {
        let mut base_path = sys_base_path();
        base_path.push(self.stype());
        base_path
    }
    fn base_path_closed(&self) -> PathBuf {
        let mut closed = get_closed_dir();
        closed.push(self.stype());
        closed
    }
    fn base_path_all(&self) -> Vec<PathBuf> {
        vec![
            self.base_path(),
            self.base_path_closed(),
        ]
    }

    fn commit_self(&self, msg: &str) -> Result<()> {
        let files_to_add = self.epaths_all().into_iter().map(|p| {
            p.to_str().unwrap().to_owned()
        }).collect::<Vec<String>>();
        git_commit(Some(&files_to_add), msg)?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} commited to git.", self.stype(), self.id())?;
        Ok(())
    }
    fn close_self(&self) -> Result<()> {
        let stype = self.stype();
        let id = self.id();
        if self.epath_closed().is_dir() {
            bail!("{} #{} is already closed.", stype, &id);
        } else {
            create_dir_all(self.epath_closed())?;
        }
        rename(self.epath(), self.epath_closed())?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} closed.", stype, &id)?;
        Ok(())
    }
    fn delete_self(&self) -> Result<()> {
        let stype = self.stype();
        let id = self.id();
        for p in self.epaths_all().iter() {
            if p.is_dir() {
                remove_dir_all(p)?;
            }
        }
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} deleted.", stype, &id)?;
        Ok(())
    }
    fn already_exists(&self) -> Result<()> {
        if self.epath().is_dir() || self.epath_closed().is_dir() {
            bail!("{} with Id #{} already exists.",
                  self.stype(), &self.id());
        }
        Ok(())
    }
    fn update_path(&mut self) -> Result<()> {
        let path = self.epath().clone();
        let epath_closed = self.epath_closed();

        match (path.is_dir(), epath_closed.is_dir()) {
            (false, false) => {
                bail!("Id \"{}\" doesn't match with any {}.",
                      self.id(),
                      self.stype());
            },
            _ => Ok(())
        }
    }
}

pub trait WriteAll: StatusTrait + TagTrait {
    fn write(&self) -> Result<()> {
        let (id, epath, stype) =
            (self.id(), self.epath(), self.stype());
        let content = format!("# {} ({})", id, stype);
        write_file(&epath, "description.md", Some(&content))?;
        self.write_status()?;
        self.write_tags()?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} created.", stype, id)?;
        Ok(())
    }
}
