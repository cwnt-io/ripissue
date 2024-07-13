// use anyhow::Result;
// use clap::Subcommand;
// use strum_macros::Display;

// use crate::executors::{issues::IssueExecutors, sprints::SprintExecutors};

// /// Ripissue: Manage your project and issues with `ripi` CLI app!
// #[derive(Debug, Parser)]
// #[command(
//     name = env!("CARGO_PKG_NAME"),
//     about = env!("CARGO_PKG_DESCRIPTION"),
//     version = env!("CARGO_PKG_VERSION"),
//     author = env!("CARGO_PKG_AUTHORS")
// )]
// pub struct Cli {
//     // Choose which element type to operate over.
//     #[command(subcommand)]
//     pub element_type: ElemType,
// }

// #[derive(Debug, Subcommand, Display)]
// pub enum ElemType {
//     ///Tasks, bugs, features, stories, Pull Requests (PR's), etc. A unit of work.
//     #[command(subcommand)]
//     Issue(IssueExecutors),
//     ///Set of issues to be executed in a certain period of time.
//     #[command(subcommand)]
//     Sprint(SprintExecutors),
//     ///Major feature. Can be a set of sprints and/or issues.
//     #[command(subcommand)]
//     Epic(IssueExecutors),
//     ///Major abstract long term goal. E.g.: solve all pending bugs until the end of the year.
//     #[command(subcommand)]
//     Initiative(IssueExecutors),
// }
//
// impl ElemType {
//     pub fn run_cmd(&self) -> Result<()> {
//         use ElemType::*;
//         match self {
//             Issue(executor) => executor.run_cmd(self)?,
//             Sprint(executor) => executor.run_cmd(self)?,
//             Epic(executor) => executor.run_cmd(self)?,
//             Initiative(executor) => executor.run_cmd(self)?,
//         };
//         Ok(())
//     }
// }
