use std::{
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Result, bail};
use walkdir::WalkDir;

use super::Element;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Todo,
    Doing,
}

impl Status {

    pub fn base_path() -> PathBuf {
        PathBuf::from_str("status").unwrap()
    }

    pub fn as_str(&self) -> &'static str {
        use Status::*;
        match *self {
            Todo => "todo",
            Doing => "doing",
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

    pub fn get(issue_id: &str) -> Result<Option<Self>> {
        let issue =
        let mut path = Element::new(issue_id);
        path.push(&issue_id);
        path.push(&Status::base_path());
        let statuses: Vec<String> = WalkDir::new(path)
            .min_depth(1)
            .into_iter()
            .flatten()
            .map(|e| e.path().to_str().unwrap().to_owned())
            .collect();
        match statuses.len() {
            0 => Ok(None),
            1 => {
                let status_str = statuses.get(0).unwrap();
                Ok(Some(Status::from(status_str)?))
            },
            _ => bail!("Input \"{}\" doesn't match with any issue", &issue_id),
        }
    }

}
