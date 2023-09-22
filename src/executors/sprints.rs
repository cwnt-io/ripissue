use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{
    elements::{elem::Elem, elem_type::ElemType, sprint::ESprint},
    helpers::{is_not_empty, is_valid_iso_date},
    properties::statuses::Status,
};

use super::general::{Creator, GeneralExecutors, GitArgs, PropertiesArgs};

#[derive(Debug, Subcommand)]
pub enum SprintExecutors {
    /// Creates a new Sprint.
    Create(CreateSprintArgs),
    /// Adds or Removes Issues inside sprints
    Manage(ManageSprintArgs),
    #[command(flatten)]
    General(GeneralExecutors),
}

#[derive(Debug, Args)]
pub struct ManageSprintArgs {
    /// Path or Id of the existing SPRINT
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
    #[command(subcommand)]
    pub subcmd_issue: SprintToIssueSubCmd,
}

#[derive(Debug, Subcommand)]
pub enum SprintToIssueSubCmd {
    AddIssue(SprintToIssueArgs),
}

#[derive(Debug, Args)]
pub struct SprintToIssueArgs {
    /// Repository directory name that has the issue
    #[arg(long, short)]
    pub repository: String,
    /// Path or Id of the existing ISSUE
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

impl SprintExecutors {
    pub fn run_cmd(&self, etype: &ElemType) -> Result<()> {
        use SprintExecutors::*;
        match self {
            Create(args) => Elem::create(args, etype)?,
            Manage(args) => ESprint::manage(args, etype)?,
            General(executor) => executor.run_cmd(etype)?,
        }
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct CreateSprintArgs {
    /// Set a due date for this Sprint. This will be its ID. (Date must be in ISO format. E.g. 2023-09-21).
    #[arg(value_parser = is_valid_iso_date)]
    pub due_date: String,
    #[command(flatten)]
    pub props: PropertiesArgs,
    #[command(flatten)]
    pub git: GitArgs,
}

impl Creator for CreateSprintArgs {
    fn name(&self) -> String {
        let mut id = "due_".to_owned();
        id.push_str(&self.due_date);
        id
    }
    fn tags(&self) -> &Option<Vec<String>> {
        &self.props.tags
    }
    fn status(&self) -> &Option<Status> {
        &self.props.status
    }
    fn dry(&self) -> bool {
        self.git.dry
    }
}
