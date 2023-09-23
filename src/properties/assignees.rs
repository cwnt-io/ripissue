use std::collections::hash_set::Iter;
use std::str::FromStr;
use std::{collections::HashSet, path::PathBuf};

use anyhow::{bail, Context, Result};

use crate::executors::general::RoleEnum;
use crate::helpers::slug;
use crate::{executors::general::AssignToEnum, helpers::walkdir_into_iter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assignee(String);

impl Assignee {
    pub fn new(member: &str, role: &RoleEnum) -> Self {
        let member = slug(member);
        let role = role.as_ref();
        Self(format!("{}-{}", member, role))
    }
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Assignees(HashSet<Assignee>);

impl Assignees {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    pub fn add(&mut self, a: Assignee) -> Result<()> {
        if !self.0.insert(a.clone()) {
            bail!(
                "Assignee {} already exists. Same member at same role.",
                a.to_str()
            );
        }
        Ok(())
    }
    pub fn iter(&self) -> Iter<'_, Assignee> {
        self.0.iter()
    }
    pub fn from_assign_to(a_to: &Option<AssignToEnum>) -> Result<Option<Self>> {
        if let Some(AssignToEnum::AssignTo { member, role }) = a_to {
            let assignee = Assignee::new(member, role);
            let mut assignees = Assignees::new();
            assignees.add(assignee)?;
            return Ok(Some(assignees));
        }
        Ok(None)
    }
    pub fn from_files(path: &PathBuf) -> Result<Option<Self>> {
        let walk_iter = walkdir_into_iter(path);
        let mut assignees = Assignees::new();
        for assign_file in walk_iter {
            let fname = assign_file.file_name().to_str().unwrap();
            let vfname: Vec<&str> = fname.split('-').collect();
            if vfname.len() != 2 {
                bail!("Assignee file name {} is not in a correct format.", fname);
            }
            let (member, role_str) = (vfname[0], vfname[1]);
            let role = FromStr::from_str(role_str).with_context(|| {
                format!(
                    "Role \"{}\" is invalid (from assignee \"{}\")",
                    role_str, fname
                )
            })?;
            let assignee = Assignee::new(member, &role);
            assignees.add(assignee)?;
        }
        match assignees.len() {
            0 => Ok(None),
            _ => Ok(Some(assignees)),
        }
    }
}
