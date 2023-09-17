use crate::{properties::{statuses::{Status, StatusTrait}, tags::{Tag, TagTrait}}, helpers::slug, executors::{create::Createable, commit::Commitable, close::Closeable, delete::Deleteable, run_subc::Runnable}};

use super::elem::{ElemBase, WriteAll};

#[derive(Debug, Clone)]
pub struct Issue {
    id: String,
    stype: &'static str,
    status: Option<Status>,
    tags: Option<Vec<Tag>>,
}

impl ElemBase for Issue {
    fn new() -> Self {
        Self {
            id: String::default(),
            stype: "Issue",
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


impl WriteAll for Issue {}

impl StatusTrait for Issue {
    fn status(&self) -> &Option<Status> {
        &self.status
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}

impl TagTrait for Issue {
    fn tags(&self) -> &Option<Vec<Tag>> {
        &self.tags
    }
    fn set_tags(&mut self, tags: Option<Vec<Tag>>) {
        self.tags = tags;
    }
}

// EXECUTORS
impl Createable for Issue {}
impl Commitable for Issue {}
impl Closeable for Issue {}
impl Deleteable for Issue {}
impl Runnable for Issue {}

    // fn add<Issue: ElemBase + Clone>(&mut self, elem: Elem<Issue>) -> Result<()> {
    //     let elem2: Elem<Issue> = elem.clone();
    //     let issue: Issue = elem2.e().clone();
    //     let id = elem.e().id().to_owned();
    //     let stype = elem.e().stype().to_owned();
    //     if self.0.insert(id.clone(),issue).is_some() {
    //         bail!("{} #{} already exists.", stype, id);
    //     }
    //     Ok(())
    // }
