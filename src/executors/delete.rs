use crate::{helpers::{is_not_empty, id_from_input}, elements::elem::ElemBase};

use clap::Args;
use anyhow::Result;

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Path or Id of the issue
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
    /// Just deletes the issue. Do not commit it to git.
    #[arg(long, short)]
    pub dry: bool,
}

pub trait Deleteable<T: ElemBase> {
    fn delete(cmd: &DeleteArgs) -> Result<()> {
        let name = id_from_input(&cmd.path_or_id);
        let mut elem = T::new(name);
        elem.update_path()?;
        elem.delete_self()?;
        if !cmd.dry {
            let msg = format!("(deleted) {} #{}.",
                elem.stype(), &elem.id());
            elem.commit_self(&msg)?;
        }
        Ok(())
    }
}
