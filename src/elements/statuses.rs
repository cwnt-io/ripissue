use std::{
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Result, bail};

use crate::helpers::write_file;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Todo,
    Doing,
}

impl Status {

    pub fn base_path(elem_path: &PathBuf) -> PathBuf {
        let mut base_path = elem_path.clone();
        base_path.push("status");
        base_path
    }

    pub fn as_str(&self) -> String {
        use Status::*;
        match *self {
            Todo => "todo".to_owned(),
            Doing => "doing".to_owned(),
        }
    }

    pub fn from(s: &str) -> Result<Self> {
        use Status::*;
        match s {
            "todo" => Ok(Todo),
            "doing" => Ok(Doing),
            &_ => bail!("Input \"{}\" is not a valid status", s),
        }
    }

    pub fn write(&self, elem_path: &PathBuf) -> Result<()> {
        let path = Status::base_path(elem_path);
        write_file(&path, &self.as_str(), None)?;
        Ok(())
    }

    // pub fn get(elem_id: &str) -> Result<Option<Self>> {
    //
    //     let mut path = Element::new(issue_id);
    //     path.push(&issue_id);
    //     path.push(&Status::base_path());
    //     let statuses: Vec<String> = WalkDir::new(path)
    //         .min_depth(1)
    //         .into_iter()
    //         .flatten()
    //         .map(|e| e.path().to_str().unwrap().to_owned())
    //         .collect();
    //     match statuses.len() {
    //         0 => Ok(None),
    //         1 => {
    //             let status_str = statuses.get(0).unwrap();
    //             Ok(Some(Status::from(status_str)?))
    //         },
    //         _ => bail!("Input \"{}\" doesn't match with any issue", &issue_id),
    //     }
    // }

}
