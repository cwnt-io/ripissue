pub mod issues;
pub mod sprints;
pub mod subcommand;

use crate::{args::issues::IssueCommand, elements::{elem::ElemBase, issue::Issue}};

use anyhow::Result;
use clap::{Parser, Subcommand};

use self::{sprints::SprintCommand, subcommand::SubCommand};

/// Manage your project and issues
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Operations for item Issue: create, list, update, close, deletes, etc.
    #[command(subcommand)]
    Issue(SubCommand),
    // /// Operations for item Sprint: create, list, update, close, deletes, etc.
    // #[command(subcommand)]
    // Sprint(SubCommand),
    // /// Create, edit, list, delete, close epic
    // Epic(EpicCommand),
    // /// Create, edit, list, delete, close initiative
    // Initiative(InitiativeCommand),
}

// TODO:
// impl EntityType {
//     fn get_elem(&self) -> impl ElemBase {
//         match *self {
//             EntityType::Issue(_) => Issue,
//         }
//     }
//
// }
