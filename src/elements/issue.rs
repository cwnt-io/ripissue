use std::io::{stdout, BufWriter, Write};

use anyhow::Result;

use crate::{properties::{statuses::{Status, StatusTrait}, tags::{Tag, TagTrait}}, helpers::{slug, write_file}};

use super::elem::ElemBase;

#[derive(Debug)]
pub struct Issue {
    id: String,
    stype: &'static str,
    status: Option<Status>,
    tags: Option<Vec<Tag>>,
}

impl ElemBase for Issue {
    fn new(name: &str) -> Self {
        Self {
            id: slug(name),
            stype: "Issue",
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

pub trait WriteAll: StatusTrait + TagTrait {
    fn write(&self) -> Result<()> {
        let (id, epath, stype) =
            (self.id(), self.epath(), self.stype());
        let content = format!("# {} ({})", id, stype);
        write_file(&epath, "description.md", Some(&content))?;
        self.write_status()?;
        self.write_tags()?;
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        writeln!(writer, "{} #{} created.", stype, id)?;
        Ok(())
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

// impl<T: ElemTrait> Elem<T> {
//     fn etype(&self) -> &'static str {
//
//     }
// }
