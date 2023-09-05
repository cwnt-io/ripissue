pub mod args;
pub mod issues;
pub mod helpers;

extern  crate  slugify;

use crate::args::{
    Cli,
    EntityType::Issue,
    IssueCommand,
};
use crate::issues::{
    create_issue,
    list_all_issues,
};

use clap::Parser;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        Issue(IssueCommand::Create(issue_cmd)) => {
            create_issue(issue_cmd)?;
        },
        Issue(IssueCommand::List(_)) => {
            list_all_issues()?;
        },
        _ => {}
    }

    Ok(())
}
