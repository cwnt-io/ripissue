mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use elements::elem::{Elem, ElemType};
use helpers::id_from_input;
// use helpers::{get_all_elems};
use properties::{tags::Tag, statuses::Status};

use crate::args::{
    Cli,
    EntityType,
    issues::IssueCommand,
};

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        EntityType::Issue(IssueCommand::Create(cmd)) => {
            let etype = ElemType::Issue;
            let name = &cmd.name;
            let status = None;
            let tags = None;
            let mut issue = Elem::new(etype, name, status, tags);
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
                let msg = format!(
                    "(created) {} #{}.",
                    issue.etype().as_ref(), &issue.id());
                issue.commit(&msg)?;
            }
        },
        EntityType::Issue(IssueCommand::Commit(cmd)) => {
            let etype = ElemType::Issue;
            let name = id_from_input(&cmd.path_or_id);
            let status = None;
            let tags = None;
            let mut issue = Elem::new(etype, name, status, tags);
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
                issue.etype().as_ref(), &issue.id());
            issue.commit(&msg)?;
        }
        EntityType::Issue(IssueCommand::Close(cmd)) => {
            let etype = ElemType::Issue;
            let name = id_from_input(&cmd.path_or_id);
            let status = None;
            let tags = None;
            let mut issue = Elem::new(etype, name, status, tags);
            issue.update_path()?;
            issue.close()?;
            let msg = format!("(closed) {} #{}.",
                issue.etype().as_ref(), &issue.id());
            issue.commit(&msg)?;
        },
        EntityType::Issue(IssueCommand::Delete(cmd)) => {
            let etype = ElemType::Issue;
            let name = id_from_input(&cmd.path_or_id);
            let status = None;
            let tags = None;
            let mut issue = Elem::new(etype, name, status, tags);
            issue.update_path()?;
            issue.delete()?;
            if !cmd.dry {
                let msg = format!("(deleted) {} #{}.",
                    issue.etype().as_ref(), &issue.id());
                issue.commit(&msg)?;
            }
        },
        EntityType::Issue(IssueCommand::List(cmd)) => {
            // let stdout = stdout();
            // let mut writer = BufWriter::new(stdout);
            // writeln!(writer,"{} #{} ({})",
            // Issue::elem().to_uppercase(),
            // &id,
            // issue.display()
            // )?;
            // let issues = get_all_elems::<Issue>()?;
            // println!("{:#?}", issues);

        }
    }

    Ok(())
}
