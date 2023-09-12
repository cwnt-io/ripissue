use std::fs::remove_dir_all;
use std::io::{stdout, BufWriter, Write};

use crate::args::issues::{CreateIssue, CloseIssue, DeleteIssue};
use crate::elements::{
    issues::Issue,
};
use crate::helpers::git_commit;

use anyhow::{Result, Ok};

// pub fn delete_issue(issues: &Issues, issue_cmd: &DeleteIssue) -> Result<()> {
//     let issue = Issue::from_str(&issues, &issue_cmd.path)?;
//     remove_dir_all(&issue.path)?;
//     let stdout = stdout();
//     let mut writer = stdout.lock();
//     writeln!(writer,"issue #{} ({}) deleted.",
//              &issue.name,
//              &issue.path.display())?;
//     if issue_cmd.update {
//         let up = UpIssue {
//             path: issue.name.clone(),
//         };
//         let msg = format!("(deleted) issue #{} ({}).",
//                  &issue.name,
//                  &issue.path.display());
//         up_issue(&issues, &up, Some(&msg))?;
//     }
//     Ok(())
// }

// pub fn up_issue(issues: &Issues, issue_cmd: &UpIssue, commit_msg: Option<&str>) -> Result<()> {
//     let issue = Issue::from_str(&issues, &issue_cmd.path)?;
//     if issue.kanban == Kanban::Closed {
//         let stdout = stdout();
//         let mut writer = BufWriter::new(stdout);
//         writeln!(writer,
//                  "issue #{} (\"{}\") is closed, nothing to do.",
//                  &issue.name,
//                  issue.path.display())?;
//         return Ok(());
//     }
//
//     let default_msg = format!("(up) issue #{}.", &issue.name);
//     let msg = commit_msg.unwrap_or(default_msg.as_str());
//     git_commit(Some(&[issue.path.to_str().unwrap().to_owned()]), &msg)?;
//     let stdout = stdout();
//     // let mut writer = BufWriter::new(stdout);
//     let mut writer = stdout.lock();
//     writeln!(writer,"Issue: #{} ({}) updated.",
//              &issue.name,
//              &issue.path.display())?;
//     Ok(())
// }
//
// pub fn close_issue(issues: &Issues, issue_cmd: &CloseIssue) -> Result<()> {
//     let mut issue = Issue::from_str(&issues, &issue_cmd.path)?;
//     if issue.kanban == Kanban::Closed {
//         let stdout = stdout();
//         let mut writer = BufWriter::new(stdout);
//         writeln!(writer,
//                  "issue #{} ({}) already closed.",
//                  &issue.name,
//                  issue.path.display())?;
//         return Ok(());
//     }
//     let mut issues_to_add = vec![];
//     issues_to_add.push(issue.path.to_str().unwrap().to_owned());
//     issue.move_to_kanban(Kanban::Closed)?;
//     issues_to_add.push(issue.path.to_str().unwrap().to_owned());
//     let msg = format!("(closes) issue #{}.", &issue.name);
//     git_commit(Some(&issues_to_add), &msg)?;
//     let stdout = stdout();
//     let mut writer = stdout.lock();
//     writeln!(writer,"closed issue #{} ({}).",
//              &issue.name,
//              &issue.path.display())?;
//     Ok(())
// }

// pub fn list_all_issues(issues: &Issues) -> Result<()> {
//     let stdout = stdout();
//     let mut writer = BufWriter::new(stdout);
//     if issues.0.is_empty() {
//         writeln!(writer,"No issues at this repo")?;
//         return Ok(());
//     }
//     for (name, issue) in issues.0.iter() {
//         match issue.kanban {
//             Kanban::Closed => {},
//             _ => writeln!(writer,"Issue: #{} ({})", name, issue.path.display())?,
//         }
//     }
//     Ok(())
// }

pub fn create_issue(issue_cmd: &CreateIssue) -> Result<()> {

    // Kanban::write_all()?;
    // let name = slug(&issue_cmd.name);
    // let issue = Issue::new(name.clone(), Kanban::Backlog);
    // issues.already_exists(&issue)?;
    // issue.write()?;
    // issues.add(issue.clone())?;
    // let stdout = stdout();
    // // let mut writer = BufWriter::new(stdout);
    // let mut writer = stdout.lock();
    // writeln!(writer,"Issue: #{} ({}) created.",
    //          &issue.name,
    //          &issue.path.display())?;
    // if issue_cmd.update {
    //     let up = UpIssue {
    //         path: name,
    //     };
    //     let msg = format!("(created) issue #{} ({}).",
    //              &issue.name,
    //              &issue.path.display());
    //     up_issue(&issues, &up, Some(&msg))?;
    // }
    Ok(())
}
