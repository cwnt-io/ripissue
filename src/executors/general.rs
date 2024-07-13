// use anyhow::Result;
// use clap::{Args, Subcommand, ValueEnum};
//
// use crate::elements::elem_type::ElemType;
// use crate::elements::elems::Elems;
// use crate::{elements::elem::Elem, helpers::is_not_empty, properties::statuses::Status};
//
// use strum_macros::{AsRefStr, Display, EnumString};
//
// #[derive(Debug, Subcommand)]
// pub enum GeneralExecutors {
//     /// Commits item to git
//     Commit(CommitArgs),
//     /// Closes, adds and commits an item
//     Close(CommitArgs),
//     /// Reopens an item and adds and commits to git
//     Reopen(CommitArgs),
//     /// Deletes an item
//     Delete(DeleteArgs),
//     /// Lists all items
//     List(ListArgs),
// }
//
// impl GeneralExecutors {
//     pub fn run_cmd(&self, etype: &ElemType) -> Result<()> {
//         use GeneralExecutors::*;
//         match self {
//             Commit(args) => Elem::commit(args, etype)?,
//             Close(args) => Elem::close(args, etype)?,
//             Reopen(args) => Elem::reopen(args, etype)?,
//             Delete(args) => Elem::delete(args, etype)?,
//             List(args) => Elems::list(args, etype)?,
//         }
//         Ok(())
//     }
// }
//
// #[derive(Debug, Args)]
// pub struct PropertiesArgs {
//     /// Tags with this item
//     #[arg(long, short)]
//     pub tags: Option<Vec<String>>,
//     /// Status to this item
//     #[arg(long, short)]
//     pub status: Option<Status>,
//     /// Assign item to a member and set a role
//     #[command(subcommand)]
//     pub assign_to: Option<AssignToEnum>,
// }
//
// #[derive(Debug, Subcommand)]
// pub enum AssignToEnum {
//     AssignTo {
//         /// Name/Nick/Alias of the team member that will be assigned for this item.
//         #[arg(long, short)]
//         member: String,
//         /// Role that this member will assume
//         #[arg(long, short)]
//         role: RoleEnum,
//     },
// }
//
// #[derive(AsRefStr, EnumString, Debug, Copy, Clone, Eq, PartialEq, ValueEnum, Display, Hash)]
// pub enum RoleEnum {
//     /// Executor/implementor/developer of the item
//     #[strum(serialize = "executor")]
//     Executor,
//     /// Reviewer of the work done by an executor
//     #[strum(serialize = "reviewer")]
//     Reviewer,
//     /// Responsible for the project integrity. Consolidates the work done by the Executor and the Reviewer.
//     #[strum(serialize = "authority")]
//     Authority,
// }
//
// #[derive(Debug, Args)]
// pub struct GitArgs {
//     /// Do not add/commit it to git.
//     #[arg(long, short)]
//     pub dry: bool,
//     /// Creates or Switches to a branch related with this item
//     #[arg(long, short)]
//     pub branch: bool,
//     /// Add all changes to git (git add -A)
//     #[arg(long, short)]
//     pub add: bool,
// }
//
// #[derive(Debug, Args)]
// pub struct PIdArgs {
//     /// Path or Id of the existing item
//     #[arg(value_parser = is_not_empty)]
//     pub path_or_id: String,
// }
//
// #[derive(Debug, Args)]
// pub struct CommitArgs {
//     #[command(flatten)]
//     pub pid: PIdArgs,
//     #[command(flatten)]
//     pub props: PropertiesArgs,
//     #[command(flatten)]
//     pub git: GitArgs,
// }
//
// #[derive(Debug, Args)]
// pub struct DeleteArgs {
//     #[command(flatten)]
//     pub pid: PIdArgs,
//     #[command(flatten)]
//     pub git: GitArgs,
// }
//
// #[derive(Debug, Args)]
// pub struct ListArgs {
//     /// All items (opened and closed).
//     #[arg(long, short)]
//     pub all: bool,
//     /// Filter by:
//     #[command(flatten)]
//     pub props: PropertiesArgs,
// }
//
// pub trait Creator {
//     fn name(&self) -> String;
//     fn tags(&self) -> &Option<Vec<String>>;
//     fn status(&self) -> &Option<Status>;
//     fn dry(&self) -> bool;
//     fn branch(&self) -> bool;
//     fn add(&self) -> bool;
//     fn assign_to(&self) -> &Option<AssignToEnum>;
// }
