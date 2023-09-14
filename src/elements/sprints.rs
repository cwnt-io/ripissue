// use std::{collections::HashSet, path::PathBuf, str::FromStr, fs::create_dir_all};
//
// use crate::helpers::slug;
//
// use anyhow::{Context, Result};
// use walkdir::WalkDir;
//
// #[derive(Debug)]
// pub struct Sprint {
//     id: String,
//     path: PathBuf,
// }
//
// impl Sprint {
//
//     pub fn new(name: &str) -> Self {
//         let mut path = PathBuf::from_str(sprint_dir()).unwrap();
//         path.push(name);
//         Self {
//             id: slug(name),
//             path,
//         }
//     }
//
//     pub fn id(&self) -> &str {
//         &self.id
//     }
//
//     pub fn write(&self) -> Result<()> {
//         if !self.path.is_dir() {
//             create_dir_all(&self.path)
//                 .with_context(|| format!("could not create sprint {}", &self.path.display()) )?;
//         }
//         Ok(())
//     }
//
// }
//
// #[derive(Debug)]
// pub struct Sprints(HashSet<Sprint>);
//
// impl Sprints {
//
//     pub fn list() -> Self {
//         let sprints_dir = WalkDir::new(sprint_dir())
//             .min_depth(1)
//             .into_iter()
//             .flatten()
//             .filter(|e| e.path().is_dir());
//         // println!("{:?}");
//         for s in sprints_dir {
//             println!("{:?}", s);
//         }
//         Self(HashSet::new())
//     }
//
// }
