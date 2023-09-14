mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use helpers::{get_file_name, get_elem_from_path, get_all_elems};
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
            let mut issue = get_elem_from_path(Issue::raw(&cmd.path_or_id)?)?;
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
            // let stdout = stdout();
            // let mut writer = BufWriter::new(stdout);
            // writeln!(writer,"{} #{} ({})",
            // Issue::elem().to_uppercase(),
            // &id,
            // issue.display()
            // )?;
            let issues = get_all_elems::<Issue>()?;
            println!("{:#?}", issues);

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
