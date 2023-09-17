use anyhow::Result;

use crate::args::subcommand::SubCommand;

use super::AllExecutables;


pub trait Runnable: AllExecutables {
    fn run_cmd(&mut self, subc: &SubCommand) -> Result<()> {
        use SubCommand::*;
        match subc {
            Create(cmd) => self.create(&cmd)?,
            Commit(cmd) => self.commit(&cmd)?,
            Close(cmd) => self.close(&cmd)?,
            Delete(cmd) => self.delete(&cmd)?,
        }
        Ok(())
    }
}
