use std::fs::{create_dir, File};
use std::path::PathBuf;
use std::str::FromStr;
use std::io::{stdout, BufWriter, Write};

use anyhow::{Context, Result, bail};
use enum_iterator::{all, Sequence};

#[derive(Debug, Sequence, Copy, Clone, PartialEq)]
pub enum Kanban {
    Backlog,
    Todo,
    Doing,
    Staging,
    Closed,
}

impl Kanban {

    pub fn as_str(&self) -> &'static str {
        use Kanban::*;
        match *self {
            Backlog => "_0_backlog",
            Todo => "_1_todo",
            Doing => "_2_doing",
            Staging => "_3_staging",
            Closed => "_4_closed",
        }
    }

    pub fn write(&self) -> Result<()> {
        let dir_str = self.as_str();
        let path = PathBuf::from_str(dir_str).unwrap();
        if !path.is_dir() {
            create_dir(&path)
                .with_context(|| format!("could not create dir {}", path.display()) )?;
            let mut empty_file = path;
            empty_file.push(".kanban");
            File::create(&empty_file)
                .with_context(|| "could not create empty file")?;
        }
        Ok(())
    }

    pub fn write_all() -> Result<()> {
        for k in all::<Kanban>() {
            k.write()?;
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

