use std::io::{stdout, BufWriter, Write};

use crate::args::{CreateIssue, CloseIssue};
use crate::issues::{Issue, Issues};
use crate::helpers::{slug, git_commit};
use crate::kanban::Kanban;

use anyhow::{Result, Ok};

pub fn close_issue(issues: &Issues, issue_cmd: &CloseIssue) -> Result<()> {
    let mut issues_to_add = vec![];
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    let mut issue = Issue::from_str(&issues, &issue_cmd.path)?;
    issues_to_add.push(issue.path.to_str().unwrap().to_owned());
    if issue.kanban == Kanban::Closed {
        writeln!(writer,
                 "Issue: \"{}\" ({}) already closed.",
                 &issue.name,
                 issue.path.display())?;
        return Ok(());
    }
    issue.move_to_kanban(Kanban::Closed)?;
    issues_to_add.push(issue.path.to_str().unwrap().to_owned());
    let msg = format!("Closes: \"{}\" issue.", &issue.name);
    git_commit(Some(&issues_to_add), &msg)?;
    Ok(())
}

pub fn list_all_issues(issues: &Issues) -> Result<()> {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    if issues.0.is_empty() {
        writeln!(writer,"No issues at this repo")?;
        return Ok(());
    }
    for (name, issue) in issues.0.iter() {
        match issue.kanban {
            Kanban::Closed => {},
            _ => writeln!(writer,"Issue: {} ({})", name, issue.path.display())?,
        }
    }
    Ok(())
}

pub fn create_issue(issues: &Issues, issue_cmd: &CreateIssue) -> Result<()> {
    Kanban::write_all()?;
    let name = slug(&issue_cmd.name);
    let issue = Issue::new(name, Kanban::Backlog);
    issues.already_exists(&issue)?;
    issue.write()?;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    writeln!(writer,"Issue: \"{}\" ({}) created.",
             &issue.name,
             &issue.path.display())?;
    Ok(())
}
