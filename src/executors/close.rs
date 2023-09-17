use anyhow::Result;
use clap::Args;

use crate::{helpers::is_not_empty, elements::elem::ElemBase};

#[derive(Debug, Args)]
pub struct CloseArgs {
    /// Path or Id of the existing item
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
}

pub trait Closeable: ElemBase {
    fn close(&mut self, cmd: &CloseArgs) -> Result<()> {
        self.set_id(&cmd.path_or_id);
        self.update_path()?;
        self.close_self()?;
        let msg = format!("(closed) {} #{}.",
            self.stype(), &self.id());
        self.commit_self(&msg)?;
        Ok(())
    }
}
