use std::collections::BTreeMap;
use std::fs::{create_dir_all, create_dir, File, rename};
use std::path::PathBuf;
use std::str::FromStr;
use std::io::{prelude::*, stdout, BufWriter, Write};

use anyhow::{Context, Result, bail, Ok};
use walkdir::WalkDir;
use enum_iterator::all;

use crate::helpers::{get_file_name, get_parent_dir};
use crate::elements::kanban::Kanban;

#[derive(Debug, Clone)]
pub struct Issue {
    pub name: String,
    pub kanban: Kanban,
    pub path: PathBuf,
}

impl Default for Issue {
    fn default() -> Self {
        Self {
            name: String::default(),
            kanban: Kanban::Backlog,
            path: PathBuf::default(),
        }
    }
}

impl Issue {

    pub fn new(name: String, kanban: Kanban) -> Self {
        let mut path = PathBuf::from_str(kanban.as_str()).unwrap();
        path.push(&name);
        Self {
            name,
            kanban,
            path,
        }
    }

    pub fn from_str(issues: &Issues, s: &str) -> Result<Self> {
        let path = PathBuf::from_str(&s).unwrap();
        if let Some(i) = issues.0.get(s) {
        // s: issue_name
            return Ok(i.clone());
        } else if let Some(i) = issues.0.get(&get_file_name(&path)) {
        // s: kanban/issue_name
            Kanban::from(get_parent_dir(&path).as_str())?;
            if path == i.path {
                return Ok(i.clone());
            }
        }
        bail!(format!("Input \"{}\" doesn't match with any issue", s));
    }

    pub fn write(&self) -> Result<()> {
        create_dir_all(&self.path)
            .with_context(|| format!("could not create issue_dir {}", &self.path.display()) )?;

        let mut desc_file_path = self.path.clone();
        desc_file_path.push("description.md");
        let mut desc_file = File::create(&desc_file_path)
            .with_context(|| "could not create issue description.md")?;
        desc_file.write_all(format!("# {}", self.name).as_bytes())
            .with_context(|| format!("could not write description title at file: {}", desc_file_path.display()))?;
        Ok(())
    }

    pub fn move_to_kanban(&mut self, kanban: Kanban) -> Result<()> {
        if self.kanban != kanban {
            let new_issue = Issue::new(self.name.clone(), kanban);
            rename(self.path.clone(), new_issue.path.clone())?;
            *self = new_issue;
        }
        Ok(())
    }

}

#[derive(Debug)]
pub struct Issues(pub BTreeMap<String,Issue>);

impl Issues {

    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn already_exists(&self, issue: &Issue) -> Result<()> {
        if self.0.contains_key(&issue.name) {
            bail!(format!(
                    "Issue {} ({}) already exists, rename it before continue",
                    &issue.name,
                    &issue.path.display()));

        }
        Ok(())
    }

    pub fn add(&mut self, issue: Issue) -> Result<()> {
        self.already_exists(&issue)?;
        self.0.insert(issue.name.clone(), issue);
        Ok(())
    }

    pub fn get_all() -> Result<Issues> {
        let mut issues = Issues::new();

        for kanban in all::<Kanban>() {
            let issues_in_kanban_dir = WalkDir::new(kanban.as_str())
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir());
            for issue_path in issues_in_kanban_dir {
                let issue_path_buf = issue_path.path().to_path_buf();
                let issue = Issue::new(get_file_name(&issue_path_buf), kanban);
                issues.add(issue)?;
            }
        }

        Ok(issues)
    }

}