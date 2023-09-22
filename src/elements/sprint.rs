use std::{collections::BTreeMap, fs::OpenOptions, io::Write};

use anyhow::Result;

use crate::executors::sprints::SprintToIssueSubCmd;
use crate::{
    executors::sprints::ManageSprintArgs,
    helpers::{get_valid_issue, get_valid_repo},
};

use super::{elem::Elem, elem_type::ElemType};

#[derive(Debug, Clone)]
pub struct ESprint {
    sprint: Elem,
}

impl ESprint {
    fn raw(etype: &ElemType) -> Self {
        Self {
            sprint: Elem::raw(etype),
        }
    }
    fn e_mut(&mut self) -> &mut Elem {
        &mut self.sprint
    }
    fn e(&self) -> &Elem {
        &self.sprint
    }
    // EXECUTORS
    pub fn manage(args: &ManageSprintArgs, etype: &ElemType) -> Result<()> {
        let mut sprint = Self::raw(etype);
        sprint.e_mut().set_all_from_files(&args.path_or_id)?;
        use SprintToIssueSubCmd::*;
        match &args.subcmd_issue {
            AddIssue(issue_args) => {
                let repo_name = &issue_args.repository;
                let issue_name = &issue_args.path_or_id;
                let repo = get_valid_repo(repo_name)?;
                let _ = get_valid_issue(&repo, issue_name)?;
                let v = vec![BTreeMap::from([(repo_name, issue_name)])];
                let yaml = serde_yaml::to_string(&v)?;

                let mut issue_yaml = sprint.e().epath();
                issue_yaml.push("issue.yaml");
                let mut issue_yaml_file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(issue_yaml)?;
                issue_yaml_file.write_all(yaml.as_bytes())?;
            }
        }
        Ok(())
    }
}
