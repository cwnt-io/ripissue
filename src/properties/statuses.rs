use std::path::PathBuf;

use clap::ValueEnum;
use anyhow::{Result, bail};

use crate::helpers::{traverse_files, get_file_name};

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Status {
    /// Issue must be done and is waiting to begin
    Todo,
    /// Issue is in execution
    Doing,
}

impl Status {

    pub fn as_str(&self) -> String {
        use Status::*;
        match *self {
            Todo => "todo".to_owned(),
            Doing => "doing".to_owned(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        use Status::*;
        match s {
            "todo" => Ok(Todo),
            "doing" => Ok(Doing),
            &_ => bail!("Input \"{}\" is not a valid status", s),
        }
    }

    pub fn status_from_files(path: &PathBuf) -> Result<Option<Self>> {
        let statuses: Vec<PathBuf> = traverse_files(path);
        let status = match statuses.len() {
            0 => None,
            1 => {
                let status_full_path = statuses.get(0).unwrap();
                let status_str = get_file_name(&status_full_path);
                Some(Status::from_str(&status_str)?)
            },
            _ => {
                let msg: Vec<String> = statuses.into_iter()
                    .map(|e| e.to_str().unwrap().to_owned())
                    .collect();
                bail!("Status can't be more than one. Found {}",
                      &msg.join(", "));
            },
        };
        Ok(status)
    }

}
