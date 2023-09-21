pub mod project;

use anyhow::Result;
use clap::{Subcommand, Args};

#[derive(Debug, Args, Clone)]
pub struct CloseArgs {
    /// Path or Id of the existing item
    // #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum GeneralExecutors {
    /// Creates a new item
    Create(CloseArgs),
    /// Commits item to git
    Commit(CloseArgs),
    /// Closes, adds and commits an item
    Close(CloseArgs),
    /// Reopens an item and adds and commits to git
    Reopen(CloseArgs),
    /// Deletes an item
    Delete(CloseArgs),
    /// Lists all items
    List(CloseArgs),
}

impl GeneralExecutors {
    pub fn run_cmd(&self) -> Result<()> {
        use GeneralExecutors::*;
        match self {
            Create(args) => {
                println!("general {:?}", args);
            },
            _ => {},
        }
        Ok(())
    }
}

// pub fn run_cmd(stype: &str, subcmd: &SubCommand) -> Result<()> {
//     use SubCommand::*;
//     match subcmd {
//         Create(cmd) => {
//             let mut elem = Elem::raw(stype);
//             elem.create(cmd)?
//         },
//         Commit(cmd) => {
//             let mut elem = Elem::raw(stype);
//             elem.commit(cmd)?
//         },
//         Close(cmd) => {
//             let mut elem = Elem::raw(stype);
//             elem.close(cmd)?
//         },
//         Reopen(cmd) => {
//             let mut elem = Elem::raw(stype);
//             elem.reopen(cmd)?
//         },
//         Delete(cmd) => {
//             let mut elem = Elem::raw(stype);
//             elem.delete(cmd)?
//         },
//         List(cmd) => {
//             match stype {
//                 "Project" if cmd.path_or_id.is_some() => {
//                     let mut proj = Elem::raw(stype);
//                     proj.list_proj(cmd)?;
//                 },
//                 _ => {
//                     let mut elems = Elems::raw(stype);
//                     elems.list(cmd)?;
//                 }
//             }
//         },
//     }
//     Ok(())
// }
