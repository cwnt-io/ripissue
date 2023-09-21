mod elements;
mod executors;
mod helpers;
mod properties;

use anyhow::Result;
use clap::Parser;

use elements::elem_type::ElemType;

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
    cli.element_type.run_cmd()?;
    Ok(())
}
