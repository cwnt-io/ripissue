use crate::executors::{create::CreateArgs, commit::CommitArgs, close::CloseArgs, delete::DeleteArgs};

use clap::{Subcommand, Args};

#[derive(Debug, Subcommand)]
pub enum IssueCommand {
    /// Creates a new issue
    Create(CreateArgs),
    /// Commits issue to git
    Commit(CommitArgs),
    /// Closes, adds and commits an issue
    Close(CloseArgs),
    /// Deletes an issue
    Delete(DeleteArgs),
    /// Lists all issues
    List(ListIssues),
}

#[derive(Debug, Args)]
pub struct ListIssues {
    /// List all issues, including the closed ones
    #[arg(long, short)]
    pub all: bool,
}

