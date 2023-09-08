use std::fs::{create_dir, File};
use std::path::PathBuf;
use std::str::FromStr;
use std::collections::HashMap;

use anyhow::{Context, Result, bail};

pub enum Kanban {
    Backlog,
    Todo,
    Doing,
    Staging,
    Closed,
}

struct KanbanIter<'a>(&'a Kanban);

impl<'a> Iterator for KanbanIter<'a> {
    type Item = Kanban;

    fn next(&mut self) -> Option<Self::Item> {
        use Kanban::*;
        match *self.0 {
            Backlog => Some(Todo),
            Todo => Some(Doing),
            Doing => Some(Staging),
            Staging => Some(Closed),
            Closed => None,
        }
    }
}

impl Kanban {

    fn as_path(&self) -> &'static str {
        use Kanban::*;
        match *self {
            Backlog => "_0_backlog",
            Todo => "_1_todo",
            Doing => "_2_doing",
            Staging => "_3_staging",
            Closed => "_4_closed",
        }
    }

    fn iter(&self) -> KanbanIter<'_> {
        KanbanIter(self)
    }

}

impl From<&str> for Kanban {
    fn from(s: &str) -> Self {
        use Kanban::*;
        match s {
            "_0_backlog" => Backlog,
            "_1_todo" => Todo,
            "_2_doing" => Doing,
            "_3_staging" => Staging,
            "_4_closed" => Closed,
        }
    }
}

pub struct KanbanDirs(pub HashMap<String, PathBuf>);

impl KanbanDirs {

    pub fn new() -> Self {
        Self(HashMap::from([
            ("_0_backlog".to_string(), PathBuf::from_str("_0_backlog").unwrap()),
            ("_1_todo".to_string(), PathBuf::from_str("_1_todo").unwrap()),
            ("_2_doing".to_string(), PathBuf::from_str("_2_doing").unwrap()),
            ("_3_staging".to_string(), PathBuf::from_str("_3_staging").unwrap()),
            ("_4_closed".to_string(), PathBuf::from_str("_4_closed").unwrap()),
        ]))
    }

    pub fn path_from_str(&self, s: &str) -> Result<PathBuf> {
        match self.0.get(s) {
            Some(&p) => Ok(p),
            None => bail!(format!("Path \"{}\" is not a kanban dir", s)),
        }
    }

    pub fn write_all(&self) -> Result<()> {
        for (_, path) in self.0 {
            if !path.is_dir() {
                create_dir(&path)
                    .with_context(|| format!("could not create dir {}", path.display()) )?;
                let mut empty_file = path;
                empty_file.push(".kanban");
                File::create(&empty_file)
                    .with_context(|| "could not create empty file")?;
            }
        }
        Ok(())
    }

}
