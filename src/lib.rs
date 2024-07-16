use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use error_msgs::ERROR_MSG_NOT_PROJECT_ROOT;

pub mod ai_module;
pub mod cli;
pub mod commands;
pub mod config;
pub mod error_msgs;
pub mod helpers;

#[derive(Debug)]
pub struct Context {
    wd: PathBuf,
}

impl Context {
    pub fn new(wd: PathBuf) -> Result<Self> {
        let is_git_dir = wd.join(".git").is_dir();
        if !is_git_dir {
            bail!(ERROR_MSG_NOT_PROJECT_ROOT);
        }
        Ok(Self { wd })
    }
    pub fn wd(&self) -> &Path {
        &self.wd
    }
}

// pub use ai_module::ai_model::{AiModel, Message};
// pub use ai_module::openai::{InnerMessage, OpenAIClient};
