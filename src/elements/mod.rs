use anyhow::Result;
use clap::Subcommand;

use crate::executors::{GeneralExecutors, project::ProjectExecutors};

#[derive(Debug, Clone, Subcommand)]
pub enum ElemType {
    ///Tasks, bugs, features, stories, Pull Requests (PR's), etc. A unit of work.
    #[command(subcommand)]
    Issue(GeneralExecutors),
    ///Set of issues to be executed in a certain period of time.
    #[command(subcommand)]
    Sprint(GeneralExecutors),
    ///Major feature. Can be a set of sprints and/or issues.
    #[command(subcommand)]
    Epic(GeneralExecutors),
    ///Major abstract long term goal. E.g.: solve all pending bugs until the end of the year.
    #[command(subcommand)]
    Initiative(GeneralExecutors),
    ///Set of repositories related to the same project.
    #[command(subcommand)]
    Project(ProjectExecutors),
}

impl ElemType {
    pub fn run_cmd(&self) -> Result<()> {
        use ElemType::*;
        match self {
            Issue(executor) => executor.run_cmd()?,
            Sprint(executor) => executor.run_cmd()?,
            Epic(executor) => executor.run_cmd()?,
            Initiative(executor) => executor.run_cmd()?,
            Project(executor) => executor.run_cmd()?,
        };
        Ok(())
    }
}

