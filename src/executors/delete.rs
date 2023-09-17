use crate::{helpers::is_not_empty, elements::elem::ElemBase};

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

pub trait Deleteable: ElemBase {
    fn delete(&mut self, cmd: &DeleteArgs) -> Result<()> {
        self.set_id(&cmd.path_or_id);
        self.update_path()?;
        self.delete_self()?;
        if !cmd.dry {
            let msg = format!("(deleted) {} #{}.",
                self.stype(), &self.id());
            self.commit_self(&msg)?;
        }
        Ok(())
    }
}
