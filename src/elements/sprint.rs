use crate::{properties::{statuses::{Status, StatusTrait}, tags::{Tag, TagTrait}}, helpers::slug, executors::{create::Createable, commit::Commitable, close::Closeable, delete::Deleteable, run_subc::Runnable}};

use super::elem::{ElemBase, WriteAll};

#[derive(Debug, Clone)]
pub struct Sprint {
    id: String,
    stype: &'static str,
    status: Option<Status>,
    tags: Option<Vec<Tag>>,
}

impl ElemBase for Sprint {
    fn new() -> Self {
        Self {
            id: String::default(),
            stype: "Sprint",
            status: None,
            tags: None
        }
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn set_id(&mut self, input: &str) {
        self.id = if input.contains("/") {
            input.split("/").last().unwrap().to_owned()
        } else {
            slug(input)
        };
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

// EXECUTORS
impl Createable for Sprint {}
impl Commitable for Sprint {}
impl Closeable for Sprint {}
impl Deleteable for Sprint {}
impl Runnable for Sprint {}
