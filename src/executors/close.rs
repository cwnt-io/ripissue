use anyhow::Result;
use clap::Args;

use crate::{helpers::{is_not_empty, id_from_input}, elements::elem::ElemBase};

#[derive(Debug, Args)]
pub struct CloseArgs {
    /// Path or Id of the existing item
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

pub trait Closeable<T: ElemBase> {
    fn close(cmd: &CloseArgs) -> Result<()> {
        let name = id_from_input(&cmd.path_or_id);
        let mut elem = T::new(name);
        elem.update_path()?;
        elem.close_self()?;
        let msg = format!("(closed) {} #{}.",
            elem.stype(), &elem.id());
        elem.commit_self(&msg)?;
        Ok(())
    }
}
