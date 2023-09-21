use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{
    elements::{elem::Elem, elem_type::ElemType},
    helpers::is_valid_iso_date,
    properties::statuses::Status,
};

use super::general::{Creator, GeneralExecutors, GitArgs, PropertiesArgs};

#[derive(Debug, Subcommand)]
pub enum SprintExecutors {
    /// Creates a new Sprint.
    Create(CreateSprintArgs),
    #[command(flatten)]
    General(GeneralExecutors),
}

impl SprintExecutors {
    pub fn run_cmd(&self, etype: &ElemType) -> Result<()> {
        use SprintExecutors::*;
        match self {
            Create(args) => Elem::create(args, etype)?,
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
