use anyhow::Result;
use clap::{Subcommand, Args};

use super::GeneralExecutors;

#[derive(Debug, Clone, Subcommand)]
pub enum ProjectExecutors {
    #[command(flatten)]
    General(GeneralExecutors),
    Repo(RepoArgs),
}

#[derive(Debug, Args, Clone)]
pub struct RepoArgs {
    /// Path or Id of the existing item
    // #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

impl ProjectExecutors {
    pub fn run_cmd(&self) -> Result<()> {
        use ProjectExecutors::*;
        match self {
            General(ex) => {
                ex.run_cmd()?;
            },
            Repo(args) => {
                println!("repo {:?}", args);
            },
        }
        Ok(())
    }
}
