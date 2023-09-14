pub mod sprints;
pub mod issues;

use std::{
    io::{stdout, BufWriter, Write},
    fs::{rename, remove_dir_all, create_dir_all}, path::PathBuf
};

use anyhow::{Result, bail};

use crate::{
    helpers::{
        git_commit,
        id_from_input,
        sys_base_path,
        get_closed_dir,
        write_file,
    },
    properties::{statuses::Status, tags::Tag}
};

pub trait Element {
    type Item;

    fn new(name: &str) -> Self::Item;
    fn write(&self) -> Result<()>;
    fn id(&self) -> String;
    fn elem() -> &'static str;
    // PATHS
    fn path(&self) -> &PathBuf;
    fn set_path(&mut self, path: &PathBuf);
    // STATUSES
    fn status(&self) -> &Option<Status>;
    fn set_status(&mut self, status: Option<Status>);
    // TAGS
    fn tags(&self) -> &Option<Vec<Tag>>;
    fn set_tags(&mut self, tags: Option<Vec<Tag>>);

    fn raw(input: &str) -> Result<Self::Item> {
        let id = id_from_input(input);
        Ok(Self::new(&id))
    }

    fn commit(&self, msg: &str) -> Result<()> {
        let files_to_add = self.paths_all().into_iter().map(|p| {
            p.to_str().unwrap().to_owned()
        }).collect::<Vec<String>>();
        git_commit(Some(&files_to_add), msg)?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} commited to git.", Self::elem().to_uppercase(), self.id())?;
        Ok(())
    }

    fn close(&self) -> Result<()> {
        let elem = Self::elem().to_uppercase();
        let id = self.id();
        if self.closed_path().is_dir() {
            bail!("{} #{} is already closed.", &elem, &id);
        } else {
            create_dir_all(self.closed_path())?;
        }
        rename(self.path(), self.closed_path())?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} closed.", &elem, &id)?;
        Ok(())
    }

    fn delete(&self) -> Result<()> {
        let elem = Self::elem().to_uppercase();
        let id = self.id();
        for p in self.paths_all().iter() {
            if p.is_dir() {
                remove_dir_all(p)?;
            }
        }
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} deleted.", &elem, &id)?;
        Ok(())
    }


    // PATHS
    fn already_exists(&self) -> Result<()> {
        if self.path().is_dir() || self.closed_path().is_dir() {
            bail!("{} with Id #{} already exists.",
                  Self::elem().to_uppercase(), &self.id());
        }
        Ok(())
    }

    fn update_path(&mut self) -> Result<()> {
        let path = self.path().clone();
        let closed_path = self.closed_path();

        let elem_path = match (path.is_dir(), closed_path.is_dir()) {
            (true,_) => path,
            (_,true) => closed_path,
            _ => {
                bail!("Id \"{}\" doesn't match with any {}.",
                      self.id(),
                      Self::elem().to_uppercase());
            },
        };
        self.set_path(&elem_path);
        Ok(())
    }

    fn base_path() -> PathBuf {
        let mut base_path = sys_base_path();
        base_path.push(Self::elem());
        base_path
    }

    fn base_path_closed() -> PathBuf {
        let mut closed = get_closed_dir();
        closed.push(Self::elem());
        closed
    }

    fn base_path_all() -> [PathBuf; 2] {
        [
            Self::base_path(),
            Self::base_path_closed(),
        ]
    }

    fn paths_all(&self) -> [PathBuf; 2] {
        [
            self.path().clone(),
            self.closed_path(),
        ]
    }

    fn closed_path(&self) -> PathBuf {
        let mut closed = Self::base_path_closed();
        closed.push(self.id());
        closed
    }

    // STATUSES
    fn write_status(&self) -> Result<()> {
        let status_path = self.status_path();
        if status_path.is_dir() {
            remove_dir_all(&status_path)?;
        }
        if let Some(status) = self.status() {
            let file = &status.as_str();
            write_file(&status_path,file,None)?;
        }
        Ok(())
    }

    fn status_path(&self) -> PathBuf {
        let mut status_path = self.path().clone();
        status_path.push("status");
        status_path
    }

    // TAGS
    fn append_tags(&mut self, tags: &Vec<Tag>) {
        if tags.is_empty() {
            return;
        }
        let mut new_tags = self.tags().clone().unwrap_or(vec![]);
        for tag in tags {
            new_tags.push(tag.clone());
        }
        self.set_tags(Some(new_tags));
    }


    fn write_tags(&self) -> Result<()> {
        if let Some(tags) = self.tags() {
            let dir = &self.tags_path();
            for tag in tags.iter() {
                let file = &tag.to_str();
                write_file(dir,file,None)?;
            }
        }
        Ok(())
    }

    fn tags_path(&self) -> PathBuf {
        let mut tags_path = self.path().clone();
        tags_path.push("tags");
        tags_path
    }

}


// pub trait Elements {
//     type Item;
//
//     fn get_elem_paths_from_bases(base_paths: &[PathBuf]) -> Vec<ElementPath> {
//         let mut all = vec![];
//         for path in base_paths.iter() {
//             let v: Vec<PathBuf> = WalkDir::new(path)
//                 .min_depth(1)
//                 .max_depth(1)
//                 .into_iter()
//                 .flatten()
//                 .filter(|e| e.path().is_dir())
//                 .map(|e| {
//                     let e_str = e.path().to_str().unwrap().to_string();
//                     Self::Item::get_path(e_str)
//                 }).collect();
//             all.extend(v);
//         }
//         all
//     }
//
//     fn get(elem_paths: &[ElementPath]) -> BTreeMap<String, Self::Item>;
//
// }

