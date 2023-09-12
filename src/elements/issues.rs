use std::{
    path::PathBuf,
    str::FromStr,
    fs::{File, create_dir_all},
    io::{Write, stdout, BufWriter},
};

use anyhow::{Context, Result};

use crate::{
    helpers::{slug, get_closed_dir},
    elements::{statuses::Status, tags::Tags},
};

use super::Element;

pub struct Issue {
    id: String,
    status: Option<Status>,
    tags: Option<Tags>,
}

impl Element for Issue {

    type Item = Self;

    fn new(&self, name: &str) -> Result<Self> {


    }

    fn id(&self) -> String {
        self.id
    }

    fn base_path() -> PathBuf {
        PathBuf::from_str("issues").unwrap()
    }

}
