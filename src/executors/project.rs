use clap::{Args, Subcommand};

use super::general::GeneralExecutors;

#[derive(Debug, Subcommand)]
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
