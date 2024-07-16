use clap::{Args, Parser, Subcommand, ValueEnum};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::helpers::is_not_empty;

/// Ripissue: Manage your project and issues with `ripi` CLI app!
#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub struct Cli {
    // Choose the command to run
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Creates a Ripissue config file
    Init,
    /// Creates a new issue
    Create(CreateIssueArgs),
    // /// Commits an issue
    // Commit(CommitArgs),
    // /// Closes an issue
    // Close(CommitArgs),
    // /// Reopens an issue
    // Reopen(CommitArgs),
    // /// Deletes an issue
    // Delete(DeleteArgs),
    // /// Shows a report for the project issues
    // Report(ListArgs),
}

#[derive(Debug, Args)]
pub struct CreateIssueArgs {
    /// Gives a name to the issue. An ID will be generated from this name. The issue ID is the slugifyed issue name.
    #[arg(value_parser = is_not_empty)]
    pub name: String,
    #[command(flatten)]
    pub props: PropertiesArgs,
    #[command(flatten)]
    pub git: GitArgs,
}

#[derive(Debug, Args)]
pub struct GitArgs {
    /// Do not add/commit it to git.
    #[arg(long, short)]
    pub dry: bool,
    /// Creates or Switches to a branch related with this item
    #[arg(long, short)]
    pub branch: bool,
    /// Add all changes to git (git add -A)
    #[arg(long, short)]
    pub add: bool,
}

#[derive(Debug, Args)]
pub struct PropertiesArgs {
    /// Tags with this item
    #[arg(long, short)]
    pub tags: Option<Vec<String>>,
    /// Status to this item
    #[arg(long, short)]
    pub status: Option<Status>,
    /// Assign item to a member and set a role
    #[command(subcommand)]
    pub assign_to: Option<AssignToEnum>,
}

#[derive(AsRefStr, EnumString, Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Status {
    /// Issue must be done and is waiting to begin
    #[strum(serialize = "todo")]
    Todo,
    /// Issue is in execution
    #[strum(serialize = "doing")]
    Doing,
    /// Issue is waiting to be reviewed
    #[strum(serialize = "review-pending")]
    ReviewPending,
    /// Issue is being reviewed
    #[strum(serialize = "review-ongoing")]
    ReviewOngoing,
    /// Issue is being reviewed
    #[strum(serialize = "review-approved")]
    ReviewApproved,
}

#[derive(Debug, Subcommand)]
pub enum AssignToEnum {
    AssignTo {
        /// Name/Nick/Alias of the team member that will be assigned for this item.
        #[arg(long, short)]
        member: String,
        /// Role that this member will assume
        #[arg(long, short)]
        role: RoleEnum,
    },
}

#[derive(AsRefStr, EnumString, Debug, Copy, Clone, Eq, PartialEq, ValueEnum, Display, Hash)]
pub enum RoleEnum {
    /// Executor/implementor/developer of the item
    #[strum(serialize = "executor")]
    Executor,
    /// Reviewer of the work done by an executor
    #[strum(serialize = "reviewer")]
    Reviewer,
    /// Responsible for the project integrity. Consolidates the work done by the Executor and the Reviewer.
    #[strum(serialize = "authority")]
    Authority,
}
