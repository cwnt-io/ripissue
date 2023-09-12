use clap::ValueEnum;
use anyhow::{Result, bail};

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Status {
    /// Issue must be done and is waiting to begin
    Todo,
    /// Issue is in execution
    Doing,
}

impl Status {

    pub fn as_str(&self) -> String {
        use Status::*;
        match *self {
            Todo => "todo".to_owned(),
            Doing => "doing".to_owned(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        use Status::*;
        match s {
            "todo" => Ok(Todo),
            "doing" => Ok(Doing),
            &_ => bail!("Input \"{}\" is not a valid status", s),
        }
    }

}
