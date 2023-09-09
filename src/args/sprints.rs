use crate::helpers::is_not_empty;

use clap::{Subcommand, Args};

#[derive(Debug, Subcommand)]
pub enum SprintCommand {
    /// Creates a new issue
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
    /// If flag is set, the issue will be updated/registered to the repository (git add and commit)
    #[arg(long, short)]
    pub update: bool,
}
