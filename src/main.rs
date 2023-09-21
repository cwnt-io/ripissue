mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use anyhow::Result;
use clap::Parser;

use crate::args::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.entity_type.run_cmd()?;
    Ok(())
}
