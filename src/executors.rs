use std::io::{stdout, BufWriter, Write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::{CreateIssue, CloseIssue};
use crate::issues::{Issue, Issues};
use crate::helpers::{slug, git_commit};
use crate::kanban::Kanban;

use anyhow::{Result, Ok};

pub fn close_issue(issues: &Issues, issue_cmd: &CloseIssue) -> Result<()> {
    let issue = Issue::from_str(&issues, &issue_cmd.path)?;
    let msg = "test commit";
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
        writeln!(writer,"Issue: {} ({})", name, issue.path.display())?;
    }
    Ok(())
}

pub fn create_issue(issues: &Issues, issue_cmd: &CreateIssue) -> Result<()> {
    Kanban::write_all()?;
    let name = slug(&issue_cmd.name);
    let mut issue_dir = PathBuf::from_str(&Kanban::Backlog.as_str()).unwrap();
    issue_dir.push(&name);
    let issue = Issue::new(name, issue_dir);
    issue.write(&issues)?;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    writeln!(writer,"Issue: \"{}\" ({}) created.",
             &issue.name,
             &issue.path.display())?;
    Ok(())
}
