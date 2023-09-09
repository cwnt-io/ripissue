use std::io::{stdout, BufWriter, Write};

use crate::args::{CreateIssue, CloseIssue, UpIssue};
use crate::issues::{Issue, Issues};
use crate::helpers::{slug, git_commit};
use crate::kanban::Kanban;

use anyhow::{Result, Ok};

pub fn up_issue(issues: &Issues, issue_cmd: &UpIssue) -> Result<()> {
    let issue = Issue::from_str(&issues, &issue_cmd.path)?;
    if issue.kanban == Kanban::Closed {
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer,
                 "issue #{} (\"{}\") is closed, nothing to do.",
                 &issue.name,
                 issue.path.display())?;
        return Ok(());
    }
    let msg = format!("(up) issue #{}.", &issue.name);
    git_commit(Some(&[issue.path.to_str().unwrap().to_owned()]), &msg)?;
    let stdout = stdout();
    // let mut writer = BufWriter::new(stdout);
    let mut writer = stdout.lock();
    writeln!(writer,"Issue: #{} ({}) updated.",
             &issue.name,
             &issue.path.display())?;
    Ok(())
}

pub fn close_issue(issues: &Issues, issue_cmd: &CloseIssue) -> Result<()> {
    let mut issue = Issue::from_str(&issues, &issue_cmd.path)?;
    if issue.kanban == Kanban::Closed {
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer,
                 "issue #{} ({}) already closed.",
                 &issue.name,
                 issue.path.display())?;
        return Ok(());
    }
    let mut issues_to_add = vec![];
    issues_to_add.push(issue.path.to_str().unwrap().to_owned());
    issue.move_to_kanban(Kanban::Closed)?;
    issues_to_add.push(issue.path.to_str().unwrap().to_owned());
    let msg = format!("(closes) issue #{}.", &issue.name);
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
            _ => writeln!(writer,"Issue: #{} ({})", name, issue.path.display())?,
        }
    }
    Ok(())
}

pub fn create_issue(issues: &mut Issues, issue_cmd: &CreateIssue) -> Result<()> {
    Kanban::write_all()?;
    let name = slug(&issue_cmd.name);
    let issue = Issue::new(name.clone(), Kanban::Backlog);
    issues.already_exists(&issue)?;
    issue.write()?;
    issues.add(issue.clone())?;
    let stdout = stdout();
    // let mut writer = BufWriter::new(stdout);
    let mut writer = stdout.lock();
    writeln!(writer,"Issue: #{} ({}) created.",
             &issue.name,
             &issue.path.display())?;
    if issue_cmd.update {
        let up = UpIssue {
            path: name,
        };
        up_issue(&issues, &up)?;
    }
    Ok(())
}
