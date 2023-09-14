mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use helpers::get_file_name;
use properties::statuses::Status;
use properties::tags::Tag;
use walkdir::WalkDir;

use std::io::{stdout, BufWriter, Write};

use crate::args::{
    Cli,
    EntityType,
    issues::IssueCommand,
};

use crate::elements::issues::Issue;
use crate::elements::Element;

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        EntityType::Issue(IssueCommand::Create(cmd)) => {
            let mut issue = Issue::new(&cmd.name);
            issue.already_exists()?;
            if let Some(ts) = &cmd.tag {
                let vec_tags = Tag::vec_tags_from_vec_str(ts);
                issue.set_tags(vec_tags);
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
            let mut issue = Issue::raw(&cmd.path_or_id)?;
            issue.update_path()?;
            let vec_tags = Tag::vec_tags_from_files(&issue.tags_path());
            issue.set_tags(vec_tags);
            let status = Status::status_from_files(&issue.status_path())?;
            issue.set_status(status);
            if let Some(ts) = &cmd.tag {
                let new_vec_tags = Tag::vec_tags_from_vec_str(ts);
                if let Some(vt) = new_vec_tags.as_ref() {
                    issue.append_tags(vt);
                }
                issue.write_tags()?;
            }
            if let Some(s) = &cmd.status {
                issue.set_status(Some(s.clone()));
                issue.write_status()?;
            }
            let msg = format!("(up) {} #{}.",
                &Issue::elem().to_uppercase(), &issue.id());
            issue.commit(&msg)?;
        }
        EntityType::Issue(IssueCommand::List(cmd)) => {
            let mut base_dirs = vec![Issue::base_path()];
            if cmd.all {
                base_dirs.push(Issue::base_path_closed());
            }
            let stdout = stdout();
            let mut writer = BufWriter::new(stdout);
            for base_dir in base_dirs.into_iter() {
                let issues = WalkDir::new(base_dir)
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .flatten()
                    .filter(|e| e.path().is_dir())
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
        EntityType::Issue(IssueCommand::Close(cmd)) => {
            let mut issue = Issue::raw(&cmd.path_or_id)?;
            issue.update_path()?;
            issue.close()?;
            let msg = format!("(closed) {} #{}.",
                &Issue::elem().to_uppercase(), &issue.id());
            issue.commit(&msg)?;
        },
        EntityType::Issue(IssueCommand::Delete(cmd)) => {
            let mut issue = Issue::raw(&cmd.path_or_id)?;
            issue.update_path()?;
            issue.delete()?;
            if !cmd.dry {
                let msg = format!("(deleted) {} #{}.",
                    &Issue::elem().to_uppercase(), &issue.id());
                issue.commit(&msg)?;
            }
        },
    }

    Ok(())
}
