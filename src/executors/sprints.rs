use std::fs::create_dir_all;
use std::{path::PathBuf, str::FromStr};

use crate::args::sprints::CreateSprint;
use crate::elements::sprints::Sprint;
use crate::helpers::slug;

use anyhow::{Context, Result};

pub fn create_sprint(issue_cmd: &CreateSprint) -> Result<()> {
    let sprint = Sprint::new(&issue_cmd.name);
    // write dir /sprints (like kanban::write_all)
    // retrieve all sprints
    // check if new sprint already_exists
    // sprint.write()?;
    // sprints.add(sprint)

    Ok(())
}
