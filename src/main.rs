mod args;
mod issues;
mod helpers;
mod executors;
mod kanban;

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
    up_issue,
};

use clap::Parser;
use anyhow::Result;
use issues::Issues;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut issues = Issues::get_all()?;

    match &cli.entity_type {
        Issue(IssueCommand::Create(issue_cmd)) => {
            create_issue(&mut issues, issue_cmd)?;
        },
        Issue(IssueCommand::List(_)) => {
            list_all_issues(&issues)?;
        },
        Issue(IssueCommand::Up(issue_cmd)) => {
            up_issue(&issues, issue_cmd)?;
        },
        Issue(IssueCommand::Close(issue_cmd)) => {
            close_issue(&issues, issue_cmd)?;
        },
    }

    Ok(())
}
