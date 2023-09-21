use std::{
    collections::BTreeMap,
    fs::{create_dir_all, remove_dir_all, rename},
    io::{BufWriter, Stdout, Write},
    path::PathBuf,
};

use anyhow::{bail, Result};
use clap::Args;

use crate::{
    args::subcmd_args::{CloseArgs, CommitArgs, CreateArgs, DeleteArgs, ListArgs, SubCommand},
    helpers::{
        base_path, base_path_all, base_path_closed, get_closed_dir, git_commit, slug,
        sys_base_path, traverse_dirs, walkdir_into_iter, write_file, wstdout,
    },
    properties::{
        statuses::Status,
        tags::{Tag, Tags},
    },
};

use crate::properties::repos::Repos;

#[derive(Debug, Clone)]
struct SpecialFields {
    repos: Option<Repos>,
}

impl SpecialFields {
    pub fn new() -> Self {
        Self { repos: None }
    }
    pub fn set_repos(&mut self, repos: Repos) {
        self.repos = Some(repos)
    }
    pub fn stdout_repos(&self, bw: &mut BufWriter<Stdout>) -> Result<()> {
        println!("{:?}", self);
        if let Some(repos) = &self.repos {
            writeln!(bw, "Repos in this project:")?;
            for r in repos.iter() {
                writeln!(bw, "{}", r.get().display())?;
            }
        } else {
            writeln!(bw, "There are no Repository associated with this Project")?;
        }

        Ok(())
    }
    // sfields.stdout_repos(&mut bw)?;
}

#[derive(Debug, Clone)]
pub struct Elem {
    id: String,
    stype: String,
    status: Option<Status>,
    tags: Option<Tags>,
    sfields: Option<SpecialFields>,
}

impl Elem {
    pub fn set_all_from_files(&mut self, input_id: &str) -> Result<()> {
        self.set_id(input_id);
        self.update_path()?;
        self.set_tags_from_files();
        self.set_status_from_files()?;
        self.set_repos();
        Ok(())
    }
    fn set_sfields(&mut self, sfields: Option<SpecialFields>) {
        self.sfields = sfields;
    }
    fn sfields(&self) -> &Option<SpecialFields> {
        &self.sfields
    }
    fn repos_from_tags(&self) -> Option<Repos> {
        if let Some(tags) = self.tags() {
            let tags = tags.filter_by_str("repo");
            let repos = tags.iter().filter_map(|t| t.nth(2)).collect();
            return Some(Repos::from_vec_str(repos));
        }
        None
    }
    fn set_repos(&mut self) {
        if let Some(repos) = self.repos_from_tags() {
            let mut sfields = self.sfields.to_owned().unwrap_or(SpecialFields::new());
            sfields.set_repos(repos);
            self.set_sfields(Some(sfields));
        }
    }
    pub fn raw(stype: &str) -> Self {
        Self {
            id: String::default(),
            stype: stype.to_owned(),
            status: None,
            tags: None,
            sfields: None,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn opened_closed_status(&self) -> Result<&str> {
        match (self.epath().is_dir(), self.epath_closed().is_dir()) {
            (true, _) => Ok("Opened"),
            (_, true) => Ok("Closed"),
            _ => bail!("{} #{} doen't exists.", self.stype(), self.id()),
        }
    }
    fn set_id(&mut self, input: &str) {
        self.id = if input.contains('/') {
            input.split('/').last().unwrap().to_owned()
        } else {
            slug(input)
        };
    }
    fn stype(&self) -> &str {
        &self.stype
    }
    pub fn status(&self) -> &Option<Status> {
        &self.status
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
    pub fn is_status(&self, status: &Option<Status>) -> bool {
        if status.is_none() {
            return true;
        }
        &self.status == status
    }
    fn status_path(&self) -> PathBuf {
        let mut status_path = self.epath();
        status_path.push("status");
        status_path
    }
    fn write_status(&self) -> Result<()> {
        let status_path = self.status_path();
        if status_path.is_dir() {
            remove_dir_all(&status_path)?;
        }
        if let Some(status) = self.status() {
            let file = &status.as_ref();
            write_file(&status_path, file, None)?;
        }
        Ok(())
    }
    fn set_status_from_files(&mut self) -> Result<()> {
        let status = Status::status_from_files(&self.status_path())?;
        self.set_status(status);
        Ok(())
    }
    fn write_status_from_cmd(&mut self, status_cmd: Option<Status>) -> Result<()> {
        if status_cmd.is_some() {
            self.set_status(status_cmd);
            self.write_status()?;
        }
        Ok(())
    }
    pub fn tags(&self) -> &Option<Tags> {
        &self.tags
    }
    fn set_tags(&mut self, tags: Option<Tags>) {
        self.tags = tags;
    }
    pub fn compare_tags(&self, tags: &Option<Tags>) -> bool {
        match (self.tags(), tags) {
            (Some(this_tags), Some(tags_filter)) => {
                for tf in tags_filter.iter() {
                    if this_tags.has_any_tag_str(tf) {
                        return true;
                    }
                }
                false
            }
            (_, None) => true,
            _ => false,
        }
    }
    fn tags_path(&self) -> PathBuf {
        let mut tags_path = self.epath().clone();
        tags_path.push("tags");
        tags_path
    }
    fn append_tags(&mut self, tags: &Tags) {
        if tags.is_empty() {
            return;
        }
        let mut new_tags = self.tags().clone().unwrap_or(Tags::new());
        for tag in tags.iter() {
            new_tags.push(tag.clone());
        }
        self.set_tags(Some(new_tags));
    }
    fn write_tags(&self) -> Result<()> {
        if let Some(tags) = self.tags() {
            let dir = &self.tags_path();
            for tag in tags.iter() {
                let file = &tag.to_str();
                write_file(dir, file, None)?;
            }
        }
        Ok(())
    }
    fn write_tags_from_cmd(&mut self, tags_cmd: &Option<Vec<String>>) -> Result<()> {
        if let Some(ts) = tags_cmd {
            let new_vec_tags = Tags::vec_tags_from_vec_str(ts);
            if let Some(vt) = new_vec_tags.as_ref() {
                self.append_tags(vt);
            }
            self.write_tags()?;
        }
        Ok(())
    }

    fn set_tags_from_vec_str(&mut self, vec: &Option<Vec<String>>) {
        if let Some(ts) = vec {
            let vec_tags = Tags::vec_tags_from_vec_str(ts);
            self.set_tags(vec_tags);
        }
    }
    fn set_tags_from_files(&mut self) {
        let vec_tags = Tags::vec_tags_from_files(&self.tags_path());
        self.set_tags(vec_tags);
    }
    fn epath(&self) -> PathBuf {
        let mut epath = base_path(&self.stype);
        epath.push(self.id());
        epath
    }
    fn epaths_all(&self) -> Vec<PathBuf> {
        vec![self.epath(), self.epath_closed()]
    }
    fn epath_closed(&self) -> PathBuf {
        let mut closed = base_path_closed(&self.stype);
        closed.push(self.id());
        closed
    }

    fn commit_self(&self, msg: &str) -> Result<()> {
        let files_to_add = self
            .epaths_all()
            .into_iter()
            .map(|p| p.to_str().unwrap().to_owned())
            .collect::<Vec<String>>();
        git_commit(Some(&files_to_add), msg)?;
        writeln!(
            wstdout(),
            "{} #{} commited to git.",
            self.stype(),
            self.id()
        )?;
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
        writeln!(wstdout(), "{} #{} closed.", stype, &id)?;
        Ok(())
    }
    fn reopen_self(&self) -> Result<()> {
        let stype = self.stype();
        let id = self.id();
        if self.epath().is_dir() {
            bail!("{} #{} is already opened.", stype, &id);
        } else {
            create_dir_all(self.epath())?;
        }
        rename(self.epath_closed(), self.epath())?;
        writeln!(wstdout(), "{} #{} reopened.", stype, &id)?;
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
        writeln!(wstdout(), "{} #{} deleted.", stype, &id)?;
        Ok(())
    }
    fn already_exists(&self) -> Result<()> {
        if self.epath().is_dir() || self.epath_closed().is_dir() {
            bail!("{} with Id #{} already exists.", self.stype(), &self.id());
        }
        Ok(())
    }
    fn update_path(&mut self) -> Result<()> {
        let path = self.epath().clone();
        let epath_closed = self.epath_closed();

        match (path.is_dir(), epath_closed.is_dir()) {
            (false, false) => {
                bail!(
                    "Id \"{}\" doesn't match with any {}.",
                    self.id(),
                    self.stype()
                );
            }
            _ => Ok(()),
        }
    }
    fn write(&self) -> Result<()> {
        let (id, epath, stype) = (self.id(), self.epath(), self.stype());
        let content = format!("# {} ({})", id, stype);
        write_file(&epath, "description.md", Some(&content))?;
        self.write_status()?;
        self.write_tags()?;
        writeln!(wstdout(), "{} #{} created.", stype, id)?;
        Ok(())
    }

    // EXECUTORS
    pub fn create(&mut self, cmd: &CreateArgs) -> Result<()> {
        self.set_id(&cmd.name);
        self.already_exists()?;
        self.set_tags_from_vec_str(&cmd.tags);
        self.set_status(cmd.status);
        self.set_repos();
        self.write()?;
        if !cmd.dry {
            let msg = format!("(created) {} #{}.", self.stype(), self.id());
            self.commit_self(&msg)?;
        }
        Ok(())
    }
    pub fn commit(&mut self, cmd: &CommitArgs) -> Result<()> {
        self.set_all_from_files(&cmd.path_or_id)?;
        self.write_tags_from_cmd(&cmd.tags)?;
        self.write_status_from_cmd(cmd.status)?;
        if !cmd.dry {
            let msg = format!("(up) {} #{}.", self.stype(), &self.id());
            self.commit_self(&msg)?;
        }
        Ok(())
    }
    pub fn close(&mut self, cmd: &CloseArgs) -> Result<()> {
        self.set_id(&cmd.path_or_id);
        self.update_path()?;
        self.close_self()?;
        let msg = format!("(closed) {} #{}.", self.stype(), &self.id());
        self.commit_self(&msg)?;
        Ok(())
    }
    pub fn reopen(&mut self, cmd: &CloseArgs) -> Result<()> {
        self.set_id(&cmd.path_or_id);
        self.update_path()?;
        self.reopen_self()?;
        let msg = format!("(reopened) {} #{}.", self.stype(), &self.id());
        self.commit_self(&msg)?;
        Ok(())
    }
    pub fn delete(&mut self, cmd: &DeleteArgs) -> Result<()> {
        self.set_id(&cmd.path_or_id);
        self.update_path()?;
        self.delete_self()?;
        if !cmd.dry {
            let msg = format!("(deleted) {} #{}.", self.stype(), &self.id());
            self.commit_self(&msg)?;
        }
        Ok(())
    }
    // TODO
    pub fn list_proj(&mut self, cmd: &ListArgs) -> Result<()> {
        self.set_all_from_files(&cmd.path_or_id.clone().unwrap())?;
        let mut bw = wstdout();
        writeln!(bw, "# {} #{}\n", self.stype(), self.id())?;
        if let Some(sfields) = self.sfields() {
            sfields.stdout_repos(&mut bw)?;
        }
        Ok(())
    }
}
