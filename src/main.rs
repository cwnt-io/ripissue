mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use elements::issue::{Issue, WriteAll};
use elements::elem::{Elem, ElemBase};
use helpers::id_from_input;
use properties::{tags::{Tag, TagTrait}, statuses::{Status, StatusTrait}};

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
            let mut eissue = Elem::new(Issue::new(&cmd.name));
            let issue = eissue.e();
            issue.already_exists()?;
            if let Some(ts) = &cmd.tag {
                let vec_tags = Tag::vec_tags_from_vec_str(ts);
                issue.set_tags(vec_tags);
            }
            if let Some(s) = &cmd.status {
                issue.set_status(Some(s.clone()))
            }
            issue.write()?;
            if !cmd.dry {
                let msg = format!(
                    "(created) {} #{}.",
                    issue.stype(), issue.id());
                issue.commit(&msg)?;
            }
        },
        EntityType::Issue(IssueCommand::Commit(cmd)) => {
            let name = id_from_input(&cmd.path_or_id);
            let mut eissue = Elem::new(Issue::new(name));
            let issue = eissue.e();
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
                issue.stype(), &issue.id());
            issue.commit(&msg)?;
        }
        EntityType::Issue(IssueCommand::Close(cmd)) => {
            let name = id_from_input(&cmd.path_or_id);
            let mut eissue = Elem::new(Issue::new(name));
            let issue = eissue.e();
            issue.update_path()?;
            issue.close()?;
            let msg = format!("(closed) {} #{}.",
                issue.stype(), &issue.id());
            issue.commit(&msg)?;
        },
        EntityType::Issue(IssueCommand::Delete(cmd)) => {
            let name = id_from_input(&cmd.path_or_id);
            let mut eissue = Elem::new(Issue::new(name));
            let issue = eissue.e();
            issue.update_path()?;
            issue.delete()?;
            if !cmd.dry {
                let msg = format!("(deleted) {} #{}.",
                    issue.stype(), &issue.id());
                issue.commit(&msg)?;
            }
        },
        EntityType::Issue(IssueCommand::List(cmd)) => {
            // let mut issues = Elems::new(ElemType::Issue);
            // issues.update()?;
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
