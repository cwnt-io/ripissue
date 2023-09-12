pub mod statuses;
pub mod sprints;
pub mod tags;
pub mod issues;

use std::{
    path::PathBuf,
    str::FromStr, io::{stdout, BufWriter, Write},
};

use anyhow::{Result, bail};

use crate::helpers::{
    get_closed_dir,
    write_file,
    get_file_name,
    git_commit,
};

pub trait Element {
    type Item;

    fn new(name: &str) -> Self::Item;

    fn base_path() -> PathBuf;

    fn path(&self) -> PathBuf {
        let mut path = Self::base_path();
        path.push(&self.id());
        path
    }

    fn closed_path(&self) -> PathBuf {
        let mut closed = get_closed_dir();
        closed.push(Self::elem());
        closed
    }

    fn id(&self) -> String;

    fn elem() -> String {
        get_file_name(&Self::base_path())
    }

    fn from(path_or_id: &str) -> Result<Self::Item> {
        let mut id = "".to_owned();
        let vec: Vec<&str> = path_or_id.split("/").collect();
        let path = match vec.len() {
            2 => {
                // path_or_id: <element>/<element_id>
                id = vec[1].to_owned();
                Some(PathBuf::from_str(&path_or_id).unwrap())
            },
            1 => {
                // path_or_id: <element_id>
                id = vec[0].to_owned();
                // let mut int_path = Self::Item::base_path() self.base_path();
                let mut int_path = Self::base_path();
                int_path.push(&path_or_id);
                Some(int_path)
            },
            _ => None,
        };

        if path.is_none() || !path.unwrap().is_dir() {
            bail!("Input \"{}\" doesn't match with any {}.",
                  &path_or_id,
                  Self::elem().to_uppercase());
        }

        Ok(Self::new(&id))
    }

    fn already_exists(&self) -> Result<()> {
        if self.path().is_dir() || self.closed_path().is_dir() {
            bail!("{} with Id #{} already exists.",
                  Self::elem().to_uppercase(), &self.id());
        }
        Ok(())
    }

    fn write_basic_files(&self) -> Result<()> {
        let (id, path, elem) = (self.id(), self.path(), Self::elem().to_uppercase());
        let content = format!("# {} ({})", &id, &elem);
        write_file(&path, "description.md", Some(&content))?;
        Ok(())
    }

    fn write(&self) -> Result<()>;

    fn commit(&self, msg: &str) -> Result<()> {
        let path = self.path().to_str().unwrap().to_owned();
        let closed_path = self.closed_path().to_str().unwrap().to_owned();
        let files_to_add = [path, closed_path];
        git_commit(Some(&files_to_add), msg)?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} commited to git.", Self::elem().to_uppercase(), self.id())?;
        Ok(())
    }

}
