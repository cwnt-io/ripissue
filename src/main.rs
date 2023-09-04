use std::{fs::{File, create_dir_all}, io::{Read, BufReader}};

use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
#[macro_use] extern  crate  slugify;
use slugify::slugify;

/// Manage your project and issues
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates an issue
    Create {
        issue_name: Option<String>,
    },
}

fn slug(s: &str) -> String {
    slugify!(s, separator = "_")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create { issue_name }) => {
            if let Some(n) = issue_name {
                create_dir_all(format!("./backlog/{}", slug(n)))
                    .with_context(|| "could not create dir")?;
                println!("creating dir: _backlog/{:?}", slug(n));
            }
        },
        None => {  }
    }

    Ok(())
}
