use crate::helpers::is_not_empty;

use clap::{Subcommand, Args};

#[derive(Debug, Subcommand)]
pub enum IssueCommand {
    /// Creates a new issue
    Create(CreateIssue),
    // /// Lists all issues
    // List(ListIssues),
    // /// Update an issue to the repository (adds and commits with git)
    // Up(UpIssue),
    // /// Closes, adds and commits an issue
    // Close(CloseIssue),
    // /// Deletes an issue
    // Delete(DeleteIssue),
}

#[derive(Debug, Args)]
pub struct ListIssues {}

#[derive(Debug, Args)]
pub struct CreateIssue {
    /// Name of the issue
    #[arg(value_parser = is_not_empty)]
    pub name: String,
    /// Just creates the issue. Do not commit it to git.
    #[arg(long, short)]
    pub soft: bool,
}

#[derive(Debug, Args)]
pub struct UpIssue {
    /// Name of the issue
    #[arg(value_parser = is_not_empty)]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct CloseIssue {
    /// Name of the issues
    #[arg(value_parser = is_not_empty)]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct DeleteIssue {
    /// Name of the issue
    #[arg(value_parser = is_not_empty)]
    pub path: String,
    /// If flag is set, the issue will be updated/registered to the repository (git add and commit)
    #[arg(long, short)]
    pub update: bool,
}
