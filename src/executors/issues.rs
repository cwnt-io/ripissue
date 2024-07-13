// use anyhow::Result;
// use clap::{Args, Subcommand};
//
// use crate::{
//     elements::{elem::Elem, elem_type::ElemType},
//     helpers::is_not_empty,
//     properties::statuses::Status,
// };
//
// use super::general::{AssignToEnum, Creator, GeneralExecutors, GitArgs, PropertiesArgs};
//
// #[derive(Debug, Subcommand)]
// pub enum IssueExecutors {
//     /// Creates a new Issue.
//     Create(CreateIssueArgs),
//     #[command(flatten)]
//     General(GeneralExecutors),
// }
//
// impl IssueExecutors {
//     pub fn run_cmd(&self, etype: &ElemType) -> Result<()> {
//         use IssueExecutors::*;
//         match self {
//             Create(args) => Elem::create(args, etype)?,
//             General(executor) => executor.run_cmd(etype)?,
//         }
//         Ok(())
//     }
// }
//
// #[derive(Debug, Args)]
// pub struct CreateIssueArgs {
//     /// Gives a name to the issue. An ID will be generated from this name.
//     #[arg(value_parser = is_not_empty)]
//     pub name: String,
//     #[command(flatten)]
//     pub props: PropertiesArgs,
//     #[command(flatten)]
//     pub git: GitArgs,
// }
//
// impl Creator for CreateIssueArgs {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//     fn tags(&self) -> &Option<Vec<String>> {
//         &self.props.tags
//     }
//     fn status(&self) -> &Option<Status> {
//         &self.props.status
//     }
//     fn assign_to(&self) -> &Option<AssignToEnum> {
//         &self.props.assign_to
//     }
//     fn dry(&self) -> bool {
//         self.git.dry
//     }
//     fn branch(&self) -> bool {
//         self.git.branch
//     }
//     fn add(&self) -> bool {
//         self.git.add
//     }
// }
