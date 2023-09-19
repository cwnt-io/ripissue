pub mod subcmd_args;

use anyhow::Result;
use clap::{Parser, Subcommand};
use strum_macros::Display;

use crate::elements::{elem::Elem, elems::Elems};

use self::subcmd_args::SubCommand;

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
    /// Operations for item Epic: create, list, update, close, deletes, etc.
    #[command(subcommand)]
    Epic(SubCommand),
    /// Operations for item Initiative: create, list, update, close, deletes, etc.
    #[command(subcommand)]
    Initiative(SubCommand),
}

impl EntityType {
    pub fn run_cmd(&self) -> Result<()> {
        use EntityType::*;
        match self {
            Issue(subcmd) => run_cmd(&self.to_string(), subcmd)?,
            Sprint(subcmd) => run_cmd(&self.to_string(), subcmd)?,
            Epic(subcmd) => run_cmd(&self.to_string(), subcmd)?,
            Initiative(subcmd) => run_cmd(&self.to_string(), subcmd)?,
        }
        Ok(())
    }
}

fn run_cmd(stype: &str, subcmd: &SubCommand) -> Result<()> {
    use SubCommand::*;
    match subcmd {
        Create(cmd) => {
            let mut elem = Elem::raw(stype);
            elem.create(cmd)?
        },
        Commit(cmd) => {
            let mut elem = Elem::raw(stype);
            elem.commit(cmd)?
        },
        Close(cmd) => {
            let mut elem = Elem::raw(stype);
            elem.close(cmd)?
        },
        Delete(cmd) => {
            let mut elem = Elem::raw(stype);
            elem.delete(cmd)?
        },
        List(cmd) => {
            let mut elems = Elems::raw(stype);
            elems.list(cmd)?;
        },
    }
    Ok(())
}
