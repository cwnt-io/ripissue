use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::{
    elements::{elem::Elem, elem_type::ElemType},
    helpers::is_valid_iso_date,
    properties::statuses::Status,
};

use super::general::{Creator, GeneralExecutors, GitArgs, PIdArgs, PropertiesArgs};

#[derive(Debug, Subcommand)]
pub enum SprintExecutors {
    /// Creates a new Sprint.
    Create(CreateSprintArgs),
    /// Adds or Removes Issues inside sprints
    Edit(EditSprintArgs),
    #[command(flatten)]
    General(GeneralExecutors),
}

#[derive(Debug, Args)]
pub struct EditSprintArgs {
    #[command(flatten)]
    pub pid: PIdArgs,
    #[command(subcommand)]
    pub subcmd_issue: SprintToIssueSubCmd,
}

#[derive(Debug, Subcommand)]
pub enum SprintToIssueSubCmd {
    AddIssue(SprintToIssueArgs),
    RemoveIssue(SprintToIssueArgs),
}

#[derive(Debug, Args)]
pub struct SprintToIssueArgs {
    /// Repository directory name that has the issue
    #[arg(long, short)]
    pub repository: bool,
    /// Issue path or id
    #[command(flatten)]
    pub pid: PIdArgs,
}

impl SprintExecutors {
    pub fn run_cmd(&self, etype: &ElemType) -> Result<()> {
        use SprintExecutors::*;
        match self {
            Create(args) => Elem::create(args, etype)?,
            Edit(args) => {
                let mut elem = Elem::raw(etype);
                elem.set_all_from_files(&args.pid.path_or_id)?;
            }
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
