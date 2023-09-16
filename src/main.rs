mod args;
mod elements;
mod helpers;
mod properties;

extern crate slugify;
use args::sprints::SprintCommand;
use elements::{issue::Issue, elem::WriteAll};
use elements::sprint::Sprint;
use elements::elem::ElemBase;
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
        EntityType::Sprint(SprintCommand::Create(cmd)) => {
            let mut sprint = Sprint::new(&cmd.name);
            sprint.already_exists()?;
            sprint.set_tags_from_vec_str(&cmd.tag);
            sprint.set_status(cmd.status);
            sprint.write()?;
            if !cmd.dry {
                let msg = format!(
                    "(created) {} #{}.",
                    sprint.stype(), sprint.id());
                sprint.commit(&msg)?;
            }
        }
        EntityType::Issue(IssueCommand::Create(cmd)) => {
            let mut issue = Issue::new(&cmd.name);
            issue.already_exists()?;
            issue.set_tags_from_vec_str(&cmd.tag);
            issue.set_status(cmd.status);
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
            let mut issue = Issue::new(name);
            issue.update_path()?;
            issue.set_tags_from_files();
            issue.set_status_from_files()?;
            issue.write_tags_from_cmd(&cmd.tag)?;
            issue.write_status_from_cmd(cmd.status)?;
            let msg = format!("(up) {} #{}.",
                issue.stype(), &issue.id());
            issue.commit(&msg)?;
        }
        EntityType::Issue(IssueCommand::Close(cmd)) => {
            let name = id_from_input(&cmd.path_or_id);
            let mut issue = Issue::new(name);
            issue.update_path()?;
            issue.close()?;
            let msg = format!("(closed) {} #{}.",
                issue.stype(), &issue.id());
            issue.commit(&msg)?;
        },
        EntityType::Issue(IssueCommand::Delete(cmd)) => {
            let name = id_from_input(&cmd.path_or_id);
            let mut issue = Issue::new(name);
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
