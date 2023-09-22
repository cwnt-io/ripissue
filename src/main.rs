mod elements;
mod executors;
mod helpers;
mod properties;

use std::env::current_dir;

use anyhow::Result;
use clap::Parser;

use elements::elem_type::ElemType;
use helpers::check_if_dir_is_repo;

/// Ripissue: Manage your project and issues with `ripi` CLI app!
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    // Choose which element type to operate over.
    #[command(subcommand)]
    pub element_type: ElemType,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let curr = current_dir()?;
    check_if_dir_is_repo(&curr)?;
    cli.element_type.run_cmd()?;
    Ok(())
}
