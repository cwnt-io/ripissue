pub mod subcommand;

use anyhow::Result;
use clap::{Parser, Subcommand};
use strum_macros::Display;

use crate::elements::elem::Elem;

use self::subcommand::SubCommand;

/// Manage your project and issues
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand, Display)]
pub enum EntityType {
    /// Operations for item Issue: create, list, update, close, deletes, etc.
    #[command(subcommand)]
    Issue(SubCommand),
    /// Operations for item Sprint: create, list, update, close, deletes, etc.
    #[command(subcommand)]
    Sprint(SubCommand),
    // /// Create, edit, list, delete, close epic
    // Epic(EpicCommand),
    // /// Create, edit, list, delete, close initiative
    // Initiative(InitiativeCommand),
}

impl EntityType {
    pub fn run_cmd(&self) -> Result<()> {
        use EntityType::*;
        match self {
            Issue(subcmd) => Elem::run_cmd(&self.to_string(), &subcmd)?,
            Sprint(subcmd) => Elem::run_cmd(&self.to_string(), &subcmd)?,
        }

        Ok(())
    }
}

