use std::fs::{create_dir, File};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Result, bail};
use enum_iterator::Sequence;

#[derive(Debug, Sequence)]
pub enum Kanban {
    Backlog,
    Todo,
    Doing,
    Staging,
    Closed,
}

impl Kanban {

    pub fn as_str(&self) -> Result<&'static str> {
        use Kanban::*;
        match *self {
            Backlog => Ok("_0_backlog"),
            Todo => Ok("_1_todo"),
            Doing => Ok("_2_doing"),
            Staging => Ok("_3_staging"),
            Closed => Ok("_4_closed"),
        }
    }

    pub fn write(&self) -> Result<()> {
        let dir_str = self.as_str().unwrap();
        let path = PathBuf::from_str(dir_str).unwrap();
        if !path.is_dir() {
            create_dir(&path)
                .with_context(|| format!("could not create dir {}", path.display()) )?;
            let mut empty_file = path;
            empty_file.push(".kanban");
            File::create(&empty_file)
                .with_context(|| "could not create empty file")?;
        } else {
            bail!("Dir \"{}\" already exists", dir_str);
        }
        Ok(())
    }

    pub fn from(s: &str) -> Result<Self> {
        use Kanban::*;
        match s {
            "_0_backlog" => Ok(Backlog),
            "_1_todo" => Ok(Todo),
            "_2_doing" => Ok(Doing),
            "_3_staging" => Ok(Staging),
            "_4_closed" => Ok(Closed),
            &_ => bail!("Input \"{}\" is not a valid kanban", s),
        }
    }

}

