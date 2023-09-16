use std::{io::{stdout, BufWriter, Write}, collections::BTreeMap};

use anyhow::{Result, bail};

use crate::{properties::{statuses::{Status, StatusTrait}, tags::{Tag, TagTrait}}, helpers::{slug, write_file}};

use super::{elem::{ElemBase, Elem, WriteAll}, elems::ElemsBase};

#[derive(Debug, Clone)]
pub struct Sprint {
    id: String,
    stype: &'static str,
    status: Option<Status>,
    tags: Option<Vec<Tag>>,
}

impl ElemBase for Sprint {
    fn new(name: &str) -> Self {
        Self {
            id: slug(name),
            stype: "Sprint",
            status: None,
            tags: None
        }
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn stype(&self) -> &str {
        self.stype
    }
}

impl WriteAll for Sprint {}

impl StatusTrait for Sprint {
    fn status(&self) -> &Option<Status> {
        &self.status
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}

impl TagTrait for Sprint {
    fn tags(&self) -> &Option<Vec<Tag>> {
        &self.tags
    }
    fn set_tags(&mut self, tags: Option<Vec<Tag>>) {
        self.tags = tags;
    }
}

