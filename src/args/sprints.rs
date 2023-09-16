use crate::{helpers::is_not_empty, properties::statuses::Status, executors::create::CreateArgs};

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SprintCommand {
    /// Creates a new sprint
    Create(CreateArgs),
    // /// Lists all issues
    // List(ListIssues),
    // /// Update an issue to the repository (adds and commits with git)
    // Up(UpIssue),
    // /// Closes, adds and commits an issue
    // Close(CloseIssue),
    // /// Deletes an issue
    // Delete(DeleteIssue),
}
