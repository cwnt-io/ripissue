pub mod statuses;
pub mod sprints;
pub mod tags;
pub mod issues;

use std::{
    path::PathBuf,
    str::FromStr,
    fs::{File, create_dir_all},
    io::{Write, stdout, BufWriter},
};

// use std::convert::AsRef;
use anyhow::{Context, Result, bail};

use crate::{
    helpers::{slug, get_closed_dir, type_to_str},
    elements::{statuses::Status, tags::Tags},
};

pub trait Element {
    type Item;

    fn new(name: &str) -> Result<Self::Item>;

    fn base_path() -> PathBuf;

    fn id(&self) -> String;

    fn get(&self, path_or_id: &str) -> Result<(String, PathBuf)> {
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
            bail!("Input \"{}\" doesn't match with any {}",
                  &path_or_id, type_to_str(&self));
        }

        Ok( (id, path.unwrap()) )
    }

    fn gen_unique_id(name: &str) -> Result<(String, PathBuf)> {
        let id = slug(&name);
        let mut path = Self::base_path();
        path.push(&id);
        let mut closed = get_closed_dir();
        closed.push(&path);
        if path.is_dir() || closed.is_dir() {
            bail!("{} with Id #{} already exists.",
                  Self::base_path().display().to_string().to_uppercase(), &id);
        }
        Ok( (id, path) )
    }

    fn write_file(&self, file_path: &PathBuf, content: &Option<String>) -> Result<()> {
        let mut file = File::create(file_path)
            .with_context(|| "Could not create file description.md")?;
        if let Some(c) = content {
            file.write_all(c.as_bytes())
                .with_context(|| format!("Could not write description title at file: {}", file_path.display()))?;
        }
        Ok(())
    }

    fn write_new(&self, name: &str) -> Result<(String, PathBuf)> {
        let (id,path) = self.gen_unique_id(name)?;
        create_dir_all(&path)
            .with_context(|| format!("Could not create issue #{}",
                                     path.display())
                          )?;
        let mut file_path = path.clone();
        file_path.push("description.md");
        let content = format!("# {}", &id);
        self.write_file(&file_path, &Some(content))?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} created.", type_to_str(&self), &id)?;
        Ok( (id,path) )
    }

}
