use std::io::{stdout, BufWriter, Write};

use crate::args::{CreateIssue, CloseIssue};
use crate::issues::{Issue, Issues, KanbanDirs};
use crate::helpers::{slug, git_commit};

use anyhow::{Result, Ok};

pub fn close_issue(issue_cmd: &CloseIssue) -> Result<()> {
    let issue = Issue::get_from_str(&issue_cmd.path)?;
    // git add _1_todo/new_issue_2/
    // git add _0_backlog/new_issue_2
    // TODO: kanbandirs to enum
    // issue.to_kanban(kanban_enum)
    //  - mv in fs and returns new issue
    let msg = "test commit";
    // println!("{:#?}", index);
    // println!("{:#?}", repo.commit());
    Ok(())
}

pub fn list_all_issues() -> Result<()> {
    let issues = Issues::get_all()?;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    if issues.0.is_empty() {
        writeln!(writer,"No issues at this repo")?;
        return Ok(());
    }
    for (name, issue) in issues.0.iter() {
        writeln!(writer,"Issue: {} ({})", name, issue.path.display())?;
    }
    Ok(())
}

pub fn create_issue(issue_cmd: &CreateIssue) -> Result<()> {
    let issues = Issues::get_all()?;
    let kanban_dirs = KanbanDirs::new();
    kanban_dirs.write()?;
    let name = slug(&issue_cmd.name);
    let mut issue_dir = kanban_dirs.backlog;
    issue_dir.push(&name);
    let issue = Issue::new(name, issue_dir);
    issues.already_exists(&issue)?;
    issue.write()?;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    writeln!(writer,"Issue: {} ({}) created",
             &issue.name,
             &issue.path.display())?;
    Ok(())
}
