use crate::{helpers::is_not_empty, properties::statuses::Status};

use clap::{Subcommand, Args};

#[derive(Debug, Subcommand)]
pub enum SprintCommand {
    /// Creates a new sprint
    Create(CreateSprint),
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
pub struct CreateSprint {
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
