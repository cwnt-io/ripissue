mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use crate::args::Cli;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.entity_type.run_cmd()?;
    Ok(())
}
