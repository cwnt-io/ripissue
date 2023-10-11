use std::{
    fs::{create_dir_all, remove_dir_all, rename},
    io::Write,
    path::PathBuf,
};

use anyhow::{bail, Result};
use cmd_lib::*;

use crate::{
    executors::general::{AssignToEnum, CommitArgs, Creator, DeleteArgs},
    helpers::{base_path, base_path_closed, git_commit, slug, write_file, wstdout},
    properties::{
        assignees::{Assignee, Assignees},
        statuses::Status,
        tags::Tags,
    },
};

use super::elem_type::ElemType;

#[derive(Debug, Clone)]
pub struct Elem {
    id: String,
    stype: String,
    stype_id: char,
    status: Option<Status>,
    tags: Option<Tags>,
    assignees: Option<Assignees>,
}

impl Elem {
    pub fn set_all_from_files(&mut self, input_id: &str) -> Result<()> {
        self.set_id(input_id);
        self.update_path()?;
        self.set_tags_from_files();
        self.set_status_from_files()?;
        let assignees = Assignees::from_files(&self.assignees_path())?;
        self.set_assignees(assignees);
        Ok(())
    }
    pub fn raw(etype: &ElemType) -> Self {
        Self {
            id: String::default(),
            stype: etype.to_string(),
            stype_id: etype.to_string().chars().nth(0).unwrap(),
            status: None,
            tags: None,
            assignees: None,
        }
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn stype_id(&self) -> &char {
        &self.stype_id
    }
    pub fn opened_closed_status(&self) -> Result<&str> {
        match (self.epath().is_dir(), self.epath_closed().is_dir()) {
            (true, _) => Ok("Opened"),
            (_, true) => Ok("Closed"),
            _ => bail!("{} #{} doen't exists.", self.stype(), self.id()),
        }
    }
    fn set_id(&mut self, input: &str) {
        self.id = if input.contains('/') {
            input.split('/').last().unwrap().to_owned()
        } else {
            slug(input)
        };
    }
    fn stype(&self) -> String {
        self.stype.clone()
    }
    pub fn status(&self) -> &Option<Status> {
        &self.status
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
    pub fn is_status(&self, status: &Option<Status>) -> bool {
        if status.is_none() {
            return true;
        }
        &self.status == status
    }
    fn status_path(&self) -> PathBuf {
        let mut status_path = self.epath();
        status_path.push("status");
        status_path
    }
    fn write_status(&self) -> Result<()> {
        let status_path = self.status_path();
        if status_path.is_dir() {
            remove_dir_all(&status_path)?;
        }
        if let Some(status) = self.status() {
            let file = &status.as_ref();
            write_file(&status_path, file, None)?;
        }
        Ok(())
    }
    fn set_status_from_files(&mut self) -> Result<()> {
        let status = Status::status_from_files(&self.status_path())?;
        self.set_status(status);
        Ok(())
    }
    fn write_status_from_cmd(&mut self, status_cmd: Option<Status>) -> Result<()> {
        if status_cmd.is_some() {
            self.set_status(status_cmd);
            self.write_status()?;
        }
        Ok(())
    }
    pub fn tags(&self) -> &Option<Tags> {
        &self.tags
    }
    fn set_tags(&mut self, tags: Option<Tags>) {
        self.tags = tags;
    }
    pub fn compare_tags(&self, tags: &Option<Tags>) -> bool {
        match (self.tags(), tags) {
            (Some(this_tags), Some(tags_filter)) => {
                for tf in tags_filter.iter() {
                    if this_tags.has_any_tag_str(tf) {
                        return true;
                    }
                }
                false
            }
            (_, None) => true,
            _ => false,
        }
    }
    fn tags_path(&self) -> PathBuf {
        let mut tags_path = self.epath().clone();
        tags_path.push("tags");
        tags_path
    }
    fn append_tags(&mut self, tags: &Tags) {
        if tags.is_empty() {
            return;
        }
        let mut new_tags = self.tags().clone().unwrap_or(Tags::new());
        for tag in tags.iter() {
            new_tags.push(tag.clone());
        }
        self.set_tags(Some(new_tags));
    }
    fn write_tags(&self) -> Result<()> {
        if let Some(tags) = self.tags() {
            let dir = &self.tags_path();
            for tag in tags.iter() {
                let file = &tag.to_str();
                write_file(dir, file, None)?;
            }
        }
        Ok(())
    }
    fn write_tags_from_cmd(&mut self, tags_cmd: &Option<Vec<String>>) -> Result<()> {
        if let Some(ts) = tags_cmd {
            let new_vec_tags = Tags::vec_tags_from_vec_str(ts);
            if let Some(vt) = new_vec_tags.as_ref() {
                self.append_tags(vt);
            }
            self.write_tags()?;
        }
        Ok(())
    }

    fn set_tags_from_vec_str(&mut self, vec: &Option<Vec<String>>) {
        if let Some(ts) = vec {
            let vec_tags = Tags::vec_tags_from_vec_str(ts);
            self.set_tags(vec_tags);
        }
    }
    fn set_tags_from_files(&mut self) {
        let vec_tags = Tags::vec_tags_from_files(&self.tags_path());
        self.set_tags(vec_tags);
    }
    pub fn epath(&self) -> PathBuf {
        let mut epath = base_path(&self.stype);
        epath.push(self.id());
        epath
    }
    fn epaths_all(&self) -> Vec<PathBuf> {
        vec![self.epath(), self.epath_closed()]
    }
    fn epath_closed(&self) -> PathBuf {
        let mut closed = base_path_closed(&self.stype);
        closed.push(self.id());
        closed
    }
    fn commit_self(&self, msg: &str) -> Result<()> {
        let files_to_add = self
            .epaths_all()
            .into_iter()
            .map(|p| p.to_str().unwrap().to_owned())
            .collect::<Vec<String>>();
        git_commit(Some(&files_to_add), msg)?;
        writeln!(
            wstdout(),
            "{} #{} commited to git.",
            self.stype(),
            self.id()
        )?;
        Ok(())
    }
    fn reopen_self(&self) -> Result<()> {
        let stype = self.stype();
        let id = self.id();
        if self.epath().is_dir() {
            bail!("{} #{} is already opened.", stype, &id);
        } else {
            create_dir_all(self.epath())?;
        }
        rename(self.epath_closed(), self.epath())?;
        writeln!(wstdout(), "{} #{} reopened.", stype, &id)?;
        Ok(())
    }
    fn delete_self(&self) -> Result<()> {
        let stype = self.stype();
        let id = self.id();
        for p in self.epaths_all().iter() {
            if p.is_dir() {
                remove_dir_all(p)?;
            }
        }
        writeln!(wstdout(), "{} #{} deleted.", stype, &id)?;
        Ok(())
    }
    fn already_exists(&self) -> Result<()> {
        if self.epath().is_dir() || self.epath_closed().is_dir() {
            bail!("{} with Id #{} already exists.", self.stype(), &self.id());
        }
        Ok(())
    }
    fn update_path(&mut self) -> Result<()> {
        let path = self.epath().clone();
        let epath_closed = self.epath_closed();

        match (path.is_dir(), epath_closed.is_dir()) {
            (false, false) => {
                bail!(
                    "Id \"{}\" doesn't match with any {}.",
                    self.id(),
                    self.stype()
                );
            }
            _ => Ok(()),
        }
    }
    fn write(&self) -> Result<()> {
        let (id, epath, stype) = (self.id(), self.epath(), self.stype());
        let content = format!("# {} ({})", id, stype);
        write_file(&epath, "description.md", Some(&content))?;
        self.write_status()?;
        self.write_tags()?;
        self.write_assignees()?;
        writeln!(wstdout(), "{} #{} created.", stype, id)?;
        Ok(())
    }

    fn set_assignees(&mut self, assignees: Option<Assignees>) {
        self.assignees = assignees
    }
    fn assignees(&self) -> &Option<Assignees> {
        &self.assignees
    }
    fn assignees_path(&self) -> PathBuf {
        let mut assignees = self.epath().clone();
        assignees.push("assignees");
        assignees
    }
    fn append_assignees(&mut self, a_to: &Option<AssignToEnum>) -> Result<()> {
        if let Some(AssignToEnum::AssignTo { member, role }) = a_to {
            let new_assignee = Assignee::new(member, *role);
            let mut assignees = self.assignees().clone().unwrap_or(Assignees::new());
            assignees.add(new_assignee)?;
            self.set_assignees(Some(assignees));
        }
        Ok(())
    }
    fn write_assignees(&self) -> Result<()> {
        if let Some(assignees) = self.assignees() {
            let dir = &self.assignees_path();
            for assignee in assignees.iter() {
                let file = assignee.filename_string();
                write_file(dir, &file, None)?;
            }
        }
        Ok(())
    }
    fn branch_name(&self) -> String {
        format!("{}-{}", self.stype_id(), self.id())
    }
    pub fn delete_branch(&self) -> Result<()> {
        let branch_name = self.branch_name();
        run_cmd!(git branch -d $branch_name)?;
        Ok(())
    }
    pub fn go_to_or_create_elem_branch(&mut self) -> Result<()> {
        let mut bw = wstdout();
        let branch = self.branch_name();
        let check_branch = run_fun!(git rev-parse --verify $branch);
        if check_branch.is_ok() {
            run_cmd!(git switch $branch)?;
        } else {
            run_cmd!(git switch -c $branch)?;
        };
        writeln!(bw, "Ripissue: now in branch {}.", branch)?;
        Ok(())
    }
    pub fn from_files(args: &CommitArgs, etype: &ElemType) -> Result<Elem> {
        let mut elem = Self::raw(etype);
        elem.set_all_from_files(&args.pid.path_or_id)?;
        elem.write_tags_from_cmd(&args.props.tags)?;
        elem.write_status_from_cmd(args.props.status)?;
        elem.append_assignees(&args.props.assign_to)?;
        elem.write_assignees()?;
        Ok(elem)
    }
    fn git_elem(&mut self, dry: bool, branch: bool, add: bool, operation: &str) -> Result<()> {
        if dry {
            return Ok(());
        }
        let msg = format!("({}) {} #{}.", operation, self.stype(), self.id());
        let gen = ["created", "reopened"];
        let end = ["closed", "deleted"];
        if branch && operation == "up" {
            self.go_to_or_create_elem_branch()?;
        } else if branch && end.contains(&operation) {
            self.delete_branch()?;
        }
        if add {
            run_cmd!(git add -A)?;
        }
        self.commit_self(&msg)?;
        if branch && gen.contains(&operation) {
            self.go_to_or_create_elem_branch()?;
        }
        Ok(())
    }
    // fn git_gen_elem(&mut self, dry: bool, branch: bool, operation: &str) -> Result<()> {
    //     // created / reopened
    //     if !dry {
    //         let msg = format!("({}) {} #{}.", operation, self.stype(), self.id());
    //         self.commit_self(&msg)?;
    //         if branch {
    //             self.go_to_or_create_elem_branch()?;
    //         }
    //     }
    //     Ok(())
    // }
    // fn git_end_elem(&mut self, dry: bool, branch: bool, operation: &str) -> Result<()> {
    //     // closed / deleted
    //     if !dry {
    //         if branch {
    //             self.delete_branch()?;
    //         }
    //         let msg = format!("({}) {} #{}.", operation, self.stype(), &self.id());
    //         self.commit_self(&msg)?;
    //     }
    //     Ok(())
    // }
    // fn git_up_elem(&mut self, dry: bool, branch: bool) -> Result<()> {
    //     // commit / inside close
    //     if !dry {
    //         if branch {
    //             self.go_to_or_create_elem_branch()?;
    //         }
    //         let msg = format!("(up) {} #{}.", self.stype(), &self.id());
    //         self.commit_self(&msg)?;
    //     }
    //     Ok(())
    // }
    // EXECUTOR: ex-create
    pub fn create(args: &impl Creator, etype: &ElemType) -> Result<()> {
        init_builtin_logger();
        let mut elem = Self::raw(etype);
        elem.set_id(&args.name());
        elem.already_exists()?;
        elem.set_tags_from_vec_str(args.tags());
        elem.set_status(*args.status());
        let assignees = Assignees::from_assign_to(args.assign_to())?;
        elem.set_assignees(assignees);
        elem.write()?;
        elem.git_elem(args.dry(), args.branch(), args.add(), "created")?;
        Ok(())
    }
    // EXECUTOR: ex-commit
    pub fn commit(args: &CommitArgs, etype: &ElemType) -> Result<()> {
        init_builtin_logger();
        let mut elem = Self::from_files(args, etype)?;
        elem.git_elem(args.git.dry, args.git.branch, args.git.add, "up")?;
        Ok(())
    }
    // EXECUTOR: ex-close
    // close workflow:
    //  - switch from elem branch to develop branch
    //  - git merge and solve conflicts
    //  - run: ripi <elem> close <elem_id> [-b]
    //      - get elem from files
    //      - if is closed, finish
    //      - will do a final commit to run pre-commit hooks
    //      - will move to .closed dir
    //      - will commit to close <elem>
    //      - if [-b] will delete already merged branch
    pub fn close(args: &CommitArgs, etype: &ElemType) -> Result<()> {
        init_builtin_logger();
        let mut elem = Elem::from_files(args, etype)?;
        let stype = elem.stype();
        let id = elem.id();
        if elem.epath_closed().is_dir() {
            bail!("{} #{} was already closed.", stype, id);
        }
        // final commit update to check pre-commits
        elem.git_elem(args.git.dry, false, args.git.add, "up")?;
        create_dir_all(elem.epath_closed())?;
        rename(elem.epath(), elem.epath_closed())?;
        elem.git_elem(args.git.dry, args.git.branch, args.git.add, "closed")?;
        writeln!(wstdout(), "{} #{} closed.", stype, &id)?;
        Ok(())
    }
    // EXECUTOR: ex-reopen
    pub fn reopen(args: &CommitArgs, etype: &ElemType) -> Result<()> {
        let mut elem = Self::raw(etype);
        elem.set_id(&args.pid.path_or_id);
        elem.update_path()?;
        elem.reopen_self()?;
        elem.git_elem(args.git.dry, args.git.branch, args.git.add, "reopened")?;
        Ok(())
    }
    // EXECUTOR: ex-delete
    pub fn delete(args: &DeleteArgs, etype: &ElemType) -> Result<()> {
        init_builtin_logger();
        let mut elem = Self::raw(etype);
        elem.set_id(&args.pid.path_or_id);
        elem.update_path()?;
        elem.delete_self()?;
        elem.git_elem(args.git.dry, args.git.branch, args.git.add, "deleted")?;
        Ok(())
    }
}
