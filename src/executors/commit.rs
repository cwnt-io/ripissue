use anyhow::Result;
use clap::Args;

use crate::elements::elem::ElemBase;
use crate::properties::statuses::{Status, StatusTrait};
use crate::helpers::{is_not_empty, id_from_input};
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

pub trait Commitable<T: ElemBase + TagTrait + StatusTrait> {
    fn commit(cmd: &CommitArgs) -> Result<()> {
        let name = id_from_input(&cmd.path_or_id);
        let mut elem = T::new(name);
        elem.update_path()?;
        elem.set_tags_from_files();
        elem.set_status_from_files()?;
        elem.write_tags_from_cmd(&cmd.tag)?;
        elem.write_status_from_cmd(cmd.status)?;
        let msg = format!("(up) {} #{}.",
            elem.stype(), &elem.id());
        elem.commit_self(&msg)?;
        Ok(())
    }
}
