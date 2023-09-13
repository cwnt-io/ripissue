use crate::{helpers::is_not_empty, properties::statuses::Status};

use clap::{Subcommand, Args};

#[derive(Debug, Subcommand)]
pub enum IssueCommand {
    /// Creates a new issue
    Create(CreateIssue),
    /// Commits issue to git
    Commit(CommitIssue),
    /// Lists all issues
    List(ListIssues),
    /// Closes, adds and commits an issue
    Close(CloseIssue),
    /// Deletes an issue
    Delete(DeleteIssue),
}

#[derive(Debug, Args)]
pub struct CommitIssue {
    /// Path or Id of the issue
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
    /// Associate tags with this issue
    #[arg(long, short)]
    pub tag: Option<Vec<String>>,
    /// Set this issue status
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
}

#[derive(Debug, Args)]
pub struct CloseIssue {
    /// Path or Id of the issue
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

#[derive(Debug, Args)]
pub struct ListIssues {
    /// List all issues, including the closed ones
    #[arg(long, short)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct CreateIssue {
    /// Name of the issue
    #[arg(value_parser = is_not_empty)]
    pub name: String,
    /// Associate tags with this issue
    #[arg(long, short)]
    pub tag: Option<Vec<String>>,
    /// Set this issue status
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
    /// Just creates the issue. Do not commit it to git.
    #[arg(long, short)]
    pub dry: bool,
}

#[derive(Debug, Args)]
pub struct DeleteIssue {
    /// Path or Id of the issue
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
    /// Just deletes the issue. Do not commit it to git.
    #[arg(long, short)]
    pub dry: bool,
}
