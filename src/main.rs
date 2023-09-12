mod args;
mod elements;
mod helpers;
mod executors;

extern crate slugify;

use crate::args::{
    Cli,
    EntityType::Issue,
};

use crate::args::issues::IssueCommand;

// use crate::executors::issues::{
//     create_issue,
//     list_all_issues,
//     close_issue,
//     up_issue,
//     delete_issue,
// };

use clap::Parser;
use anyhow::Result;
use elements::Element;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        Issue(IssueCommand::Create(cmd)) => {
            let issue = Element::Issue { id: (), status: (), tags: () }
        },
        // Issue(IssueCommand::List(_)) => {
        //     list_all_issues(&issues)?;
        // },
        // Issue(IssueCommand::Up(issue_cmd)) => {
        //     up_issue(&issues, issue_cmd, None)?;
        // },
        // Issue(IssueCommand::Close(issue_cmd)) => {
        //     close_issue(&issues, issue_cmd)?;
        // },
        // Issue(IssueCommand::Delete(issue_cmd)) => {
        //     delete_issue(&issues, issue_cmd)?;
        // },
        // Sprint(SprintCommand::Create(sprint_cmd)) => {
        //     // println!("sprint create!!!");
        //     Sprints::get();
        // },
    }

    Ok(())
}
