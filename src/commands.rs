use std::{fs::File, io::Write};

use anyhow::{bail, Result};

use crate::{
    cli::CreateIssueArgs, config::Config, error_msgs::ERROR_MSG_CONFIG_FILE_ALREADY_EXISTS, Context,
};

pub fn init(ctx: &Context, cfg: Config) -> Result<()> {
    let wd = ctx.wd();
    let config_file_path = wd.join("ripissue.toml");
    if config_file_path.is_file() {
        bail!(ERROR_MSG_CONFIG_FILE_ALREADY_EXISTS);
    }
    let cfg_string = toml::to_string(&cfg)?;
    let mut cfg_file = File::create(config_file_path)?;
    cfg_file.write_all(cfg_string.as_bytes())?;
    Ok(())
}

pub fn create_issue(_create_issue_args: CreateIssueArgs) {}
