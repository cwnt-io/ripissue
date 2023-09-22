// TODO: implement a new type that is a wrapper around Elem
// For Elem: create a new field that saves repo_from
// -- saves it when elem is read/created

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env::current_dir,
    path::PathBuf,
};

use anyhow::Result;

use crate::executors::sprints::SprintToIssueSubCmd;
use crate::{
    executors::sprints::ManageSprintArgs,
    helpers::{get_valid_issue, get_valid_repo, walkdir_into_iter},
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
    // EXECUTORS
    pub fn manage(args: &ManageSprintArgs, etype: &ElemType) -> Result<()> {
        let mut sprint = Self::raw(etype);
        use SprintToIssueSubCmd::*;
        match &args.subcmd_issue {
            AddIssue(issue_args) => {
                let repo = get_valid_repo(&issue_args.repository)?;
                let issue = get_valid_issue(&repo, &issue_args.pid.path_or_id)?;
                // TODO: create yaml file if dont exists
                // append line - repo: issue to it
            }
            RemoveIssue(issue_args) => {}
        }
        Ok(())
    }
}
