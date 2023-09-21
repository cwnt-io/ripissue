use anyhow::Result;
use clap::{Args, Subcommand};

use crate::elements::elems::Elems;
use crate::{elements::elem::Elem, helpers::is_not_empty, properties::statuses::Status};

use crate::elements::elem_type::ElemType;

#[derive(Debug, Subcommand)]
pub enum GeneralExecutors {
    /// Creates a new item
    Create(CreateArgs),
    /// Commits item to git
    Commit(CommitArgs),
    /// Closes, adds and commits an item
    Close(PIdArgs),
    /// Reopens an item and adds and commits to git
    Reopen(PIdArgs),
    /// Deletes an item
    Delete(PIdArgs),
    /// Lists all items
    List(ListArgs),
}

impl GeneralExecutors {
    pub fn run_cmd(&self, etype: &ElemType) -> Result<()> {
        use GeneralExecutors::*;
        match self {
            Create(args) => Elem::create(args, etype)?,
            Commit(args) => Elem::commit(args, etype)?,
            Close(args) => Elem::close(args, etype)?,
            Reopen(args) => Elem::reopen(args, etype)?,
            Delete(args) => Elem::delete(args, etype)?,
            List(args) => Elems::list(args, etype)?,
        }
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct PropertiesArgs {
    /// Tags with this item
    #[arg(long, short)]
    pub tags: Option<Vec<String>>,
    /// Status to this item
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
}

#[derive(Debug, Args)]
pub struct GitArgs {
    /// Do not add/commit it to git.
    #[arg(long, short)]
    pub dry: bool,
}

#[derive(Debug, Args)]
pub struct PIdArgs {
    /// Path or Id of the existing item
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Gives a name to the item. An ID will be generated from this name.
    #[arg(value_parser = is_not_empty)]
    pub name: String,
    #[command(flatten)]
    pub props: PropertiesArgs,
    #[command(flatten)]
    pub git: GitArgs,
}

#[derive(Debug, Args)]
pub struct CommitArgs {
    #[command(flatten)]
    pub pid: PIdArgs,
    #[command(flatten)]
    pub props: PropertiesArgs,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(flatten)]
    pub pid: PIdArgs,
    /// All items (opened and closed).
    #[arg(long, short)]
    pub all: bool,
    /// Filter by:
    #[command(flatten)]
    pub props: PropertiesArgs,
}
