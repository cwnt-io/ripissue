use std::{
    collections::BTreeMap,
    io::{BufWriter, Stdout, Write},
    path::PathBuf,
};

use anyhow::{bail, Result};

use crate::{
    args::subcmd_args::ListArgs,
    helpers::{base_path, base_path_all, traverse_dirs, wstdout},
    properties::{
        statuses::Status,
        tags::{Tag, Tags},
    },
};

use super::elem::Elem;

#[derive(Clone, Debug)]
struct FilterBy {
    all: bool,
    status: Option<Status>,
    tags: Option<Tags>,
}

impl FilterBy {
    pub fn stdout_filters(&self, bw: &mut BufWriter<Stdout>) -> Result<()> {
        writeln!(bw, "Filtered by:")?;
        self.stdout_all(bw)?;
        self.stdout_status(bw)?;
        self.stdout_tags(bw)?;
        writeln!(bw, "")?;
        Ok(())
    }
    pub fn stdout_all(&self, bw: &mut BufWriter<Stdout>) -> Result<()> {
        if self.all {
            writeln!(bw, "Opened and closed")?;
        }
        Ok(())
    }
    pub fn stdout_status(&self, bw: &mut BufWriter<Stdout>) -> Result<()> {
        if let Some(status) = self.status {
            writeln!(bw, "Status: {}", status.as_ref())?;
        }
        Ok(())
    }
    pub fn stdout_tags(&self, bw: &mut BufWriter<Stdout>) -> Result<()> {
        if let Some(tags) = &self.tags {
            let vec_str: Vec<String> = tags.iter().map(|t| t.to_str()).collect();
            let str = vec_str.join(", ");
            writeln!(bw, "Tags: {}", str)?;
        }
        Ok(())
    }
}

impl Default for FilterBy {
    fn default() -> Self {
        Self {
            all: false,
            status: None,
            tags: None,
        }
    }
}

#[derive(Debug)]
pub struct Elems {
    data: BTreeMap<String, Elem>,
    stype: String,
    filter_by: Option<FilterBy>,
    epaths: Vec<PathBuf>,
}

impl Elems {
    fn stype(&self) -> &str {
        &self.stype
    }
    pub fn raw(stype: &str) -> Self {
        Self {
            data: BTreeMap::new(),
            stype: stype.to_owned(),
            epaths: vec![],
            filter_by: None,
        }
    }
    fn add(&mut self, elem: Elem) -> Result<()> {
        let id = elem.id().to_owned();
        let will_be_added = if let Some(FilterBy { status, tags, .. }) = self.filter_by() {
            elem.is_status(&status) && elem.compare_tags(&tags)
        } else {
            true
        };
        if !will_be_added {
            return Ok(());
        }
        if self.data.insert(id.clone(), elem).is_some() {
            bail!("{} with id #{} already exists.", self.stype(), id);
        }
        Ok(())
    }
    fn filter_by(&self) -> Option<FilterBy> {
        self.filter_by.clone()
    }
    fn epaths(&self) -> &Vec<PathBuf> {
        &self.epaths
    }
    fn set_epaths(&mut self, epaths: Vec<PathBuf>) {
        self.epaths = epaths;
    }
    fn update_epaths(&mut self) {
        let base_paths = if self.filter_by().is_some() && self.filter_by().as_ref().unwrap().all {
            base_path_all(&self.stype)
        } else {
            vec![base_path(&self.stype)]
        };
        self.set_epaths(traverse_dirs(&base_paths));
    }
    fn set_filter_by(&mut self, cmd: &ListArgs) {
        if !cmd.all && cmd.tags.is_none() && cmd.status.is_none() {
            self.filter_by = None;
            return;
        }
        let mut tags = None;
        if let Some(ts) = &cmd.tags {
            tags = Tags::vec_tags_from_vec_str(ts);
        }
        self.filter_by = Some(FilterBy {
            all: cmd.all,
            status: cmd.status,
            tags,
        })
    }
    fn get(&mut self, cmd: &ListArgs) -> Result<()> {
        self.set_filter_by(cmd);
        self.update_epaths();
        let epaths = self.epaths().clone();
        for epath in epaths {
            let mut elem = Elem::raw(self.stype());
            elem.set_all_from_files(epath.to_str().unwrap())?;
            self.add(elem)?;
        }
        Ok(())
    }
    // TODO: improve output format
    pub fn list(&mut self, cmd: &ListArgs) -> Result<()> {
        self.get(cmd)?;
        let mut bw = wstdout();
        writeln!(bw, "# {}S", self.stype().to_uppercase())?;
        if let Some(filter) = self.filter_by() {
            filter.stdout_filters(&mut bw)?;
        }
        if self.data.is_empty() {
            writeln!(bw, "No {}s found.", self.stype())?;
            return Ok(());
        }
        for (_, e) in self.data.iter() {
            writeln!(bw, "#{} ({})", e.id(), e.opened_closed_status()?)?;
        }
        Ok(())
    }
}
