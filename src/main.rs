use std::env::current_dir;

use anyhow::Result;
use clap::Parser;

use ripissue::elements::elem_type::ElemType;
use ripissue::helpers::check_if_dir_is_repo;

/// Ripissue: Manage your project and issues with `ripi` CLI app!
#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
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
