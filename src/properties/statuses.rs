use std::path::PathBuf;
use std::str::FromStr;

use clap::ValueEnum;
use anyhow::{Result, bail};
use strum_macros::{AsRefStr, EnumString};

use crate::helpers::{traverse_files, get_file_name};

#[derive(AsRefStr, EnumString, Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Status {
    /// Issue must be done and is waiting to begin
    #[strum(serialize = "todo")]
    Todo,
    /// Issue is in execution
    #[strum(serialize = "doing")]
    Doing,
}

impl Status {

    pub fn status_from_files(path: &PathBuf) -> Result<Option<Self>> {
        let statuses: Vec<PathBuf> = traverse_files(path);
        let status = match statuses.len() {
            0 => None,
            1 => {
                let status_full_path = statuses.get(0).unwrap();
                let status_str = get_file_name(&status_full_path);
                let status = FromStr::from_str(&status_str)?;
                Some(status)
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
