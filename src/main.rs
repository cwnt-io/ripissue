mod args;
mod elements;
mod helpers;
mod executors;
mod properties;

extern crate slugify;
use helpers::get_file_name;
use walkdir::WalkDir;

use std::io::{stdout, BufWriter, Write};

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
use properties::tags::Tags;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        EntityType::Issue(IssueCommand::Create(cmd)) => {
            let mut issue = Issue::new(&cmd.name);
            issue.already_exists()?;
            if let Some(ts) = &cmd.tag {
                issue.set_tags(Some(Tags::from_vec_str(&ts)));
            }
            if let Some(s) = &cmd.status {
                issue.set_status(Some(s.clone()));
            }
            issue.write()?;
            if !cmd.dry {
                let msg = format!("(created) {} #{}.",
                &Issue::elem().to_uppercase(), &issue.id());
                issue.commit(&msg)?;
            }
        },
        EntityType::Issue(IssueCommand::Commit(cmd)) => {
            let issue_path = Issue::get_path(&cmd.path_or_id)?;
            let mut issue = Issue::from_path(&issue_path)?;
            if let Some(ts) = &cmd.tag {
                issue.set_tags(Some(Tags::from_vec_str(&ts)));
                issue.write_tags()?;
            }
            if let Some(s) = &cmd.status {
                issue.set_status(Some(s.clone()));
                issue.write_status()?;
            }
            let msg = format!("(up) {} #{}.",
            &Issue::elem().to_uppercase(), &issue.id());
            issue.commit(&msg)?;
            // let path = PathBuf::from_str(&cmd.path_or_id)?;
            // println!("{:?}", path.display());
            // println!("{:?}", path.parent());
        }
        EntityType::Issue(IssueCommand::List(cmd)) => {
            let mut base_dirs = vec![Issue::base_path()];
            if cmd.all {
                base_dirs.push(Issue::base_closed());
            }
            let stdout = stdout();
            let mut writer = BufWriter::new(stdout);
            for base_dir in base_dirs.into_iter() {
                let issues = WalkDir::new(base_dir)
                    .min_depth(1)
                    .into_iter()
                    .flatten()
                    .map(|e| {
                        e.path().to_path_buf()
                    });
                for issue in issues {
                    let id = get_file_name(&issue);
                    writeln!(writer,"{} #{} ({})",
                             Issue::elem().to_uppercase(),
                             &id,
                             issue.display()
                             )?;
                }
            }
        }
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
