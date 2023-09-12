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
