pub mod issues;
pub mod sprints;

use crate::args::issues::IssueCommand;
use crate::args::sprints::SprintCommand;

use clap::{Parser, Subcommand};

/// Manage your project and issues
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, list, update, close or delete issues.
    #[command(subcommand)]
    Issue(IssueCommand),
    /// Create, list, update, close or delete issues.
    #[command(subcommand)]
    Sprint(SprintCommand),

    // /// Create, edit, list, delete, close epic
    // Epic(EpicCommand),
    // /// Create, edit, list, delete, close initiative
    // Initiative(InitiativeCommand),
}

