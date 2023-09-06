use std::collections::{HashSet, HashMap, BTreeMap};
use std::fs::{create_dir_all, create_dir, File, read_dir};
use std::path::PathBuf;
use std::str::FromStr;
use std::io::{prelude::*, stdout, BufWriter, Write, BufReader};

use anyhow::{Context, Result, bail, Ok};
use walkdir::{WalkDir, DirEntry};

use crate::args::{CreateIssue, CloseIssue};
use crate::helpers::slug;

#[derive(Debug)]
pub struct KanbanDirs {
    backlog: PathBuf,
    todo: PathBuf,
    doing: PathBuf,
    staging: PathBuf,
    closed: PathBuf,
}

impl KanbanDirs {

    pub fn new() -> Self {
        KanbanDirs {
            backlog: PathBuf::from_str("_0_backlog").unwrap(),
            todo: PathBuf::from_str("_1_todo").unwrap(),
            doing: PathBuf::from_str("_2_doing").unwrap(),
            staging: PathBuf::from_str("_3_staging").unwrap(),
            closed: PathBuf::from_str("_4_closed").unwrap(),
        }
    }

    fn as_vec(&self) -> Vec<PathBuf> {
        vec![
            self.backlog.clone(),
            self.todo.clone(),
            self.doing.clone(),
            self.staging.clone(),
            self.closed.clone(),
        ]
    }

    fn write(&self) -> Result<()> {
        let dirs = self.as_vec();
        for dir in dirs {
            if !dir.is_dir() {
                create_dir(&dir)
                    .with_context(|| format!("could not create dir {}", dir.display()) )?;
                let mut empty_file = dir;
                empty_file.push(".kanban");
                File::create(&empty_file)
                    .with_context(|| "could not create empty file")?;
            }
        }
        Ok(())
    }

}

fn get_all_issues() -> Result<HashMap<String, PathBuf>> {
    let kanban_dirs = KanbanDirs::new();
    let mut map = HashMap::new();
    for dir in kanban_dirs.as_vec() {
        let issues_paths = WalkDir::new(dir).min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir());
        for issue_path in issues_paths {
            let issue_path_buf = issue_path.path().to_path_buf();
            let name = get_file_name(&issue_path_buf);
            if map.insert(name.clone(), issue_path_buf.clone()).is_some() {
                bail!(format!("Issue {} ({}) already exists, rename it before continue", name, issue_path_buf.display()));
            }
        }
    }
    Ok(map)
}

fn get_file_name(path: &PathBuf) -> String {
    path.file_name().unwrap().to_os_string().into_string().unwrap()
}

pub fn close_issue(issue_cmd: &CloseIssue) -> Result<()> {
    let issues = get_all_issues()?;
    let path = PathBuf::from_str(&issue_cmd.path)?;
    let name = get_file_name(&path);
    if issues.get(&name).is_none() {
        bail!(format!("Issue {} doen't exists", name));
    }
    println!("{:?}", path.is_dir());
    Ok(())
}

pub fn list_all_issues() -> Result<()> {
    let issues = get_all_issues()?;
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    if issues.is_empty() {
        writeln!(writer,"No issues at this repo")?;
        return Ok(());
    }
    for (name, path) in issues.iter() {
        writeln!(writer,"Issue: {} ({})", name, path.display())?;
    }
    Ok(())
}

pub fn create_issue(issue_cmd: &CreateIssue) -> Result<()> {
    let issues = get_all_issues()?;
    let kanban_dirs = KanbanDirs::new();
    kanban_dirs.write()?;
    let name = slug(&issue_cmd.name);
    if issues.get(&name).is_some() {
        bail!(format!("Issue with name {} already exists, rename it before continue", name));
    };

    let mut issue_dir = kanban_dirs.backlog;
    issue_dir.push(&name);
    create_dir_all(&issue_dir)
        .with_context(|| format!("could not create issue_dir {}", issue_dir.display()) )?;

    let mut desc_file_path = issue_dir.clone();
    desc_file_path.push("description.md");
    let mut desc_file = File::create(&desc_file_path)
        .with_context(|| "could not create issue description.md")?;
    desc_file.write_all(format!("# {}", name).as_bytes())
        .with_context(|| format!("could not write description title at file: {}", desc_file_path.display()))?;

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout);
    writeln!(writer,"Issue: {} ({}) created", name, issue_dir.display())?;

    Ok(())
}
