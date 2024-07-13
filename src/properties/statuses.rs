// use std::path::PathBuf;
// use std::str::FromStr;
//
// use anyhow::{bail, Result};
// use clap::ValueEnum;
// use strum_macros::{AsRefStr, EnumString};
//
// use crate::helpers::{get_file_name, traverse_files};
//
// #[derive(AsRefStr, EnumString, Debug, Copy, Clone, PartialEq, ValueEnum)]
// pub enum Status {
//     /// Item must be done and is waiting to begin
//     #[strum(serialize = "todo")]
//     Todo,
//     /// Item is in execution
//     #[strum(serialize = "doing")]
//     Doing,
//     /// Item is waiting to be reviewed
//     #[strum(serialize = "review-pending")]
//     ReviewPending,
//     /// Item is being reviewed
//     #[strum(serialize = "review-ongoing")]
//     ReviewOngoing,
//     /// Item is being reviewed
//     #[strum(serialize = "review-approved")]
//     ReviewApproved,
// }
//
// impl Status {
//     pub fn status_from_files(path: &PathBuf) -> Result<Option<Self>> {
//         let statuses: Vec<PathBuf> = traverse_files(path);
//         let status = match statuses.len() {
//             0 => None,
//             1 => {
//                 let status_full_path = statuses.first().unwrap();
//                 let status_str = get_file_name(status_full_path);
//                 let status = FromStr::from_str(&status_str)?;
//                 Some(status)
//             }
//             _ => {
//                 let msg: Vec<String> = statuses
//                     .into_iter()
//                     .map(|e| e.to_str().unwrap().to_owned())
//                     .collect();
//                 bail!("Status can't be more than one. Found {}", &msg.join(", "));
//             }
//         };
//         Ok(status)
//     }
// }
