use clap::Subcommand;

use crate::executors::{create::CreateArgs, commit::CommitArgs, close::CloseArgs, delete::DeleteArgs};

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Creates a new item
    Create(CreateArgs),
    /// Commits item to git
    Commit(CommitArgs),
    /// Closes, adds and commits an item
    Close(CloseArgs),
    /// Deletes an item
    Delete(DeleteArgs),
    // /// Lists all issues
    // List(ListIssues),
}
