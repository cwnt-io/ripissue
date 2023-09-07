mod args;
mod issues;
mod helpers;
mod executors;

extern crate slugify;

use crate::args::{
    Cli,
    EntityType::Issue,
    IssueCommand,
};
use crate::executors::{
    create_issue,
    list_all_issues,
    close_issue,
};

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        Issue(IssueCommand::Create(issue_cmd)) => {
            create_issue(issue_cmd)?;
        },
        Issue(IssueCommand::List(_)) => {
            list_all_issues()?;
        },
        Issue(IssueCommand::Close(issue_cmd)) => {
            close_issue(issue_cmd)?;
        },
        _ => {}
    }

    Ok(())
}
