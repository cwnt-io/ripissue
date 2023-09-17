use anyhow::Result;
use clap::Args;

use crate::elements::elem::ElemBase;
use crate::properties::statuses::{Status, StatusTrait};
use crate::helpers::is_not_empty;
use crate::properties::tags::TagTrait;

#[derive(Debug, Args)]
pub struct CommitArgs {
    /// Path or Id of the existing item
    #[arg(value_parser = is_not_empty)]
    pub path_or_id: String,
    /// Associate tags with this item
    #[arg(long, short)]
    pub tag: Option<Vec<String>>,
    /// Set a status to this item
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
}

pub trait Commitable: ElemBase + TagTrait + StatusTrait {
    fn commit(&mut self, cmd: &CommitArgs) -> Result<()> {
        self.set_id(&cmd.path_or_id);
        self.update_path()?;
        self.set_tags_from_files();
        self.set_status_from_files()?;
        self.write_tags_from_cmd(&cmd.tag)?;
        self.write_status_from_cmd(cmd.status)?;
        let msg = format!("(up) {} #{}.",
            self.stype(), &self.id());
        self.commit_self(&msg)?;
        Ok(())
    }
}
