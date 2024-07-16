use std::env::current_dir;

use anyhow::Result;
use clap::Parser;
use ripissue::{
    cli::{Cli, Commands::*},
    commands::{create_issue, init},
    config::Config,
    Context,
};

// use elements::elem_type::ElemType;
// use helpers::check_if_dir_is_repo;

fn main() -> Result<()> {
    let ctx = Context::new(current_dir()?)?;
    let cfg = Config::default();
    let cli = Cli::parse();
    println!("{:#?}", cli);
    match cli.cmd {
        Init => init(&ctx, cfg)?,
        Create(args) => create_issue(args),
    };
    // let curr = current_dir()?;
    // check_if_dir_is_repo(&curr)?;
    // cli.element_type.run_cmd()?;
    Ok(())
}
