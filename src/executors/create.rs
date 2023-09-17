use crate::{helpers::is_not_empty, properties::{statuses::{Status, StatusTrait}, tags::TagTrait}, elements::elem::{ElemBase, WriteAll}};

use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Name of the item
    #[arg(value_parser = is_not_empty)]
    pub name: String,
    /// Associate tags with this item
    #[arg(long, short)]
    pub tag: Option<Vec<String>>,
    /// Set a status to this item
    #[arg(long, short, value_enum)]
    pub status: Option<Status>,
    /// Just creates the items files. Do not add/commit it to git.
    #[arg(long, short)]
    pub dry: bool,
}

pub trait Createable: ElemBase + TagTrait + StatusTrait + WriteAll {
    fn create(&mut self, cmd: &CreateArgs) -> Result<()> {
        self.set_id(&cmd.name);
        self.already_exists()?;
        self.set_tags_from_vec_str(&cmd.tag);
        self.set_status(cmd.status);
        self.write()?;
        if !cmd.dry {
            let msg = format!(
                "(created) {} #{}.",
                self.stype(), self.id());
            self.commit_self(&msg)?;
        }
        Ok(())
    }
}
