use crate::args::sprints::CreateSprint;

use anyhow::Result;

pub fn create_sprint(issue_cmd: &CreateSprint) -> Result<()> {
    // write dir /sprints (like kanban::write_all)
    // retrieve all sprints
    // check if new sprint already_exists
    // sprint.write()?;
    // sprints.add(sprint)

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
