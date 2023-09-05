use std::collections::{HashSet, HashMap, BTreeMap};
use std::fs::{create_dir_all, File, read_dir};
use std::path::PathBuf;
use std::str::FromStr;
use std::io::{prelude::*, stdout, BufWriter, Write, BufReader};

use anyhow::{Context, Result, bail};
use walkdir::{WalkDir, DirEntry};

use crate::args::CreateIssue;
use crate::helpers::slug;

#[derive(Debug)]
pub struct Issues {
    dirs: Vec<String>,
    issues: BTreeMap<u32, Issue>,
}

impl Issues {

    fn new() -> Self {
        let dirs = vec![
            "_0_backlog".to_string(),
            "_1_todo".to_string(),
            "_2_doing".to_string(),
            "_3_staging".to_string(),
            "_4_closed".to_string(),
        ];
        Issues {
            dirs,
            issues: BTreeMap::new()
        }
    }

    pub fn add(&mut self, mut issue: Issue) -> Result<()> {
        if self.issues.contains_key(&issue.id) {
            let last_id_opt = self.issues.last_key_value();
            let next_id = if let Some((last_id,_)) = last_id_opt {
                last_id + 1
            } else {
                1
            };
            let id_file_path = issue.path.path().clone().join("id");
            let mut id_file = File::create(id_file_path)
                .with_context(|| "could not create id")?;
            id_file.write_all(format!("{}", next_id).as_bytes())
                .with_context(|| "could not write id")?;
            issue.id = get_id_from_file(&issue.path)?;
        }
        self.issues.insert(issue.id, issue);
        Ok(())
    }

}

#[derive(Debug)]
pub struct Issue {
    id: u32,
    name: String,
    path: DirEntry,
}

fn get_id_from_file(issue_path: &DirEntry) -> Result<u32> {
    let id_path = issue_path.path().join("id");
    let mut id_str = String::new();
    let f = File::open(id_path)
        .with_context(|| format!("could not read file `{:?}`", issue_path))?;
    let mut content = BufReader::new(f);
    content.read_line(&mut id_str)?;
    id_str.trim().to_string().parse::<u32>()
        .with_context(|| format!("invalid id ({:?}) for issue {:?}", id_str, issue_path))
}

impl Issue {
    pub fn new(path: &DirEntry) -> Result<Self> {
        Ok(Issue {
            id: get_id_from_file(path)?,
            name: path.file_name().to_str().unwrap().to_owned(),
            path: path.clone(),
        })
    }
}

fn get_all_issues() -> Result<Issues> {
    let mut issues = Issues::new();
    let dirs: Vec<String> = issues.dirs.clone();
    for dir in dirs {
        let issues_paths = WalkDir::new(dir).min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir());
        for issue_path in issues_paths {
            let issue = Issue::new(&issue_path)?;
            println!("{:?}", issue);
            issues.add(issue)?;
        }
    }
    Ok(issues)
}

pub fn list_all_issues() -> Result<()> {
    let issues = get_all_issues()?;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    if issues.issues.is_empty() {
        writeln!(writer,"No issues in this repo")?;
        return Ok(());
    }
    for (id, issue) in issues.issues.iter() {
        writeln!(writer,"Issue #{}: {}", id, issue.name)?;
    }
    // println!("{:#?}", issues.issues);
    Ok(())
}


pub fn create_issue(issue_cmd: &CreateIssue) -> Result<()> {

    let issues = get_all_issues()?;

    let name = slug(&issue_cmd.name);
    let issues = Issues::new();
    let base_dir = PathBuf::from_str(&issues.dirs[0]).unwrap();
    create_dir_all(&base_dir)
        .with_context(|| format!("could not create base_dir {}", base_dir.display()) )?;
    let mut issue_dir = base_dir.clone();
    issue_dir.push(name);
    create_dir_all(&issue_dir)
        .with_context(|| format!("could not create issue_dir {}", issue_dir.display()) )?;
    let mut desc_file_path = issue_dir.clone();
    desc_file_path.push("description.md");
    File::create(desc_file_path)
        .with_context(|| "could not create issue description.md")?;

    // for dir_str in issues.dirs.iter() {
    //     let base_dir = PathBuf::from_str(&dir_str).unwrap();
    //     if !base_dir.is_dir() {
    //         create_dir_all(&base_dir)
    //             .with_context(|| format!("could not create base_dir {}", base_dir.display()) )?;
    //     }
    //     let issue_dir = base_dir.clone().push(&name);
    // }
    // if dir.is_dir() {
    //     bail!(format!("issue create: {} already exists, nothing was done", dir.into_os_string().into_string().unwrap()));
    // }
    // let mut desc_file = dir.clone();
    // desc_file.push("description.md");
    // create_dir_all(dir)
    //     .with_context(|| "could not create issue directory")?;
    // File::create(desc_file)
    //     .with_context(|| "could not create issue description.md")?;
    Ok(())
}
