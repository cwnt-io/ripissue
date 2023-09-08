use std::path::PathBuf;

use slugify::slugify;
use anyhow::{Context, Result, Ok};
use git2::{Repository, IndexAddOption};

pub fn slug(s: &str) -> String {
    slugify!(s, separator = "_")
}

pub fn get_file_name(path: &PathBuf) -> String {
    path.file_name().unwrap().to_os_string().into_string().unwrap()
}

pub fn get_parent_dir(path: &PathBuf) -> String {
    path.parent().unwrap().to_str().unwrap().to_owned()
}

pub fn git_commit(files_to_add: Option<&[String]>, msg: &str) -> Result<()> {
    let repo = Repository::open(".")
        .with_context(|| "failed to open repository")?;
    let signature = repo.signature()?;
    let mut index = repo.index()?;
    if let Some(files_to_add) = files_to_add {
        index.add_all(files_to_add.iter(), IndexAddOption::DEFAULT, None)?;
    }
    index.write()?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let head = repo.head()?;
    let ref_name = head.name();
    let parent_commit_res = head.peel_to_commit();
    let parent_commit = if parent_commit_res.is_ok() {
        vec![parent_commit_res.as_ref().unwrap()]
    } else {
        vec![]
    };

    repo.commit(
        ref_name,
        &signature,
        &signature,
        msg,
        &tree,
        &parent_commit,
    )?;
    Ok(())
}
