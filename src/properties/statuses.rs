use std::{path::PathBuf, fs::remove_dir_all};
use std::str::FromStr;

use clap::ValueEnum;
use anyhow::{Result, bail};
use strum_macros::{AsRefStr, EnumString};

use crate::helpers::write_file;
use crate::{helpers::{traverse_files, get_file_name}, elements::elem::ElemBase};

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

pub trait StatusTrait: ElemBase {
    fn status(&self) -> &Option<Status>;
    fn set_status(&mut self, status: Option<Status>);
    fn status_path(&self) -> PathBuf {
        let mut status_path = self.epath();
        status_path.push("status");
        status_path
    }
    fn write_status(&self) -> Result<()> {
        let status_path = self.status_path();
        if status_path.is_dir() {
            remove_dir_all(&status_path)?;
        }
        if let Some(status) = self.status() {
            let file = &status.as_ref();
            write_file(&status_path,file,None)?;
        }
        Ok(())
    }
    fn set_status_from_files(&mut self) -> Result<()> {
        let status = Status::status_from_files(&self.status_path())?;
        self.set_status(status);
        Ok(())
    }
    fn write_status_from_cmd(&mut self, status_cmd: Option<Status>) -> Result<()> {
        if status_cmd.is_some() {
            self.set_status(status_cmd);
            self.write_status()?;
        }
        Ok(())
    }
}
