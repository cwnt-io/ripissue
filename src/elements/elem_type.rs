use anyhow::Result;
use clap::Subcommand;
use strum_macros::Display;

use crate::executors::{issues::IssueExecutors, sprints::SprintExecutors};

#[derive(Debug, Subcommand, Display)]
pub enum ElemType {
    ///Tasks, bugs, features, stories, Pull Requests (PR's), etc. A unit of work.
    #[command(subcommand)]
    Issue(IssueExecutors),
    ///Set of issues to be executed in a certain period of time.
    #[command(subcommand)]
    Sprint(SprintExecutors),
    ///Major feature. Can be a set of sprints and/or issues.
    #[command(subcommand)]
    Epic(IssueExecutors),
    ///Major abstract long term goal. E.g.: solve all pending bugs until the end of the year.
    #[command(subcommand)]
    Initiative(IssueExecutors),
}

impl ElemType {
    pub fn run_cmd(&self) -> Result<()> {
        use ElemType::*;
        match self {
            Issue(executor) => executor.run_cmd(self)?,
            Sprint(executor) => executor.run_cmd(self)?,
            Epic(executor) => executor.run_cmd(self)?,
            Initiative(executor) => executor.run_cmd(self)?,
        };
        Ok(())
    }
}
