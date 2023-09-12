mod args;
mod elements;
mod helpers;
mod executors;

extern crate slugify;

use crate::args::{
    Cli,
    EntityType,
    issues::IssueCommand,
};

use crate::elements::issues::Issue;
use crate::elements::Element;

// use crate::executors::issues::{
//     create_issue,
//     list_all_issues,
//     close_issue,
//     up_issue,
//     delete_issue,
// };

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        EntityType::Issue(IssueCommand::Create(cmd)) => {
            let issue = Issue::new(&cmd.name);
            issue.already_exists()?;
            issue.write()?;
            if !cmd.soft {
                let msg = format!("(created) {} #{}.",
                &Issue::elem().to_uppercase(), &issue.id());
                issue.commit(&msg)?;
            }
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
