mod cli;
mod helpers;

// mod elements;
// mod executors;
// mod properties;

use anyhow::Result;
use clap::Parser;

use cli::Cli;
// use elements::elem_type::ElemType;
// use helpers::check_if_dir_is_repo;

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("{:#?}", cli);
    // let curr = current_dir()?;
    // check_if_dir_is_repo(&curr)?;
    // cli.element_type.run_cmd()?;
    Ok(())
}
