use std::{collections::BTreeMap, path::PathBuf};

use anyhow::{Result, bail};

use crate::{properties::{statuses::Status, tags::Tag}, args::subcmd_args::ListArgs, helpers::{base_path_all, base_path, traverse_dirs}};

use super::elem::Elem;

#[derive(Clone, Debug)]
struct FilterBy {
    epaths: Vec<PathBuf>,
    status: Option<Status>,
    tags: Option<Vec<Tag>>,
}

impl Default for FilterBy {
    fn default() -> Self {
        Self {
            epaths: Vec::default(),
            status: None,
            tags: None,
        }
    }
}

#[derive(Debug)]
pub struct Elems {
    data: BTreeMap<String, Elem>,
    stype: String,
    filter_by: FilterBy
}

impl Elems {
    fn stype(&self) -> &str {
        &self.stype
    }
    pub fn raw(stype: &str) -> Self {
        Self {
            data: BTreeMap::new(),
            stype: stype.to_owned(),
            filter_by: FilterBy::default(),
        }
    }
    fn add(&mut self, elem: Elem) -> Result<()> {
        let id = elem.id().to_owned();
        if self.data.insert(id.clone(), elem).is_some() {
            bail!("{} with id #{} already exists.", self.stype(), id);
        }
        Ok(())
    }
    fn filter_by(&self) -> FilterBy {
        self.filter_by.clone()
    }
    fn set_filter_by(&mut self, cmd: &ListArgs) {
        let base_paths = if cmd.all {
            base_path_all(&self.stype)
        } else {
            vec![base_path(&self.stype)]
        };
        let epaths = traverse_dirs(&base_paths);
        let mut tags = None;
        if let Some(ts) = &cmd.tags {
            tags = Tag::vec_tags_from_vec_str(ts);
        }
        self.filter_by = FilterBy {
            epaths,
            status: cmd.status,
            tags
        }
    }
    fn get(&mut self) -> Result<()> {
        let FilterBy {
            epaths,
            status,
            tags,
        } = self.filter_by();
        for epath in epaths {
            let mut elem = Elem::raw(self.stype());
            elem.set_all_from_files(epath.to_str().unwrap())?;
            if elem.is_status(&status)
                && elem.compare_tags(&tags)
            {
                self.add(elem)?;
            }
        }
        Ok(())
    }
    pub fn list(&mut self, cmd: &ListArgs) -> Result<()> {
        self.set_filter_by(cmd);
        self.get()?;
        println!("{:#?}", self);

        // Text:
        // # ISSUES: Filtered by:
        // - Status: doing
        // - Tags: assignee, feature, screen
        // for e in elems.iter() {
            // stdout each e
        // }
        Ok(())
    }
}
