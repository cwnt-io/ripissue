use clap::{Subcommand, Args};

use crate::helpers::is_not_empty;
use crate::properties::statuses::Status;

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
    /// Lists all items
    List(ListArgs),
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Name of the item
    #[arg(value_parser = is_not_empty)]
    pub name: String,
    /// Associate tags with this item
    #[arg(long, short)]
    pub tags: Option<Vec<String>>,
    /// Set a status to this item
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
    /// Just creates the items files. Do not add/commit it to git.
    #[arg(long, short)]
    pub dry: bool,
}

#[derive(Debug, Args)]
pub struct CommitArgs {
    /// Path or Id of the existing item
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
    /// Associate tags with this item
    #[arg(long, short)]
    pub tags: Option<Vec<String>>,
    /// Set a status to this item
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
}

#[derive(Debug, Args)]
pub struct CloseArgs {
    /// Path or Id of the existing item
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Path or Id of the existing item
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
    /// Just deletes the item. Do not commit it to git.
    #[arg(long, short)]
    pub dry: bool,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// All items (opened and closed).
    #[arg(long, short)]
    pub all: bool,
    /// Filter by status
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
    /// Filter by tag
    #[arg(long, short)]
    pub tags: Option<Vec<String>>,
}
