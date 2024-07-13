use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use assert_cmd::Command;
use tempfile::tempdir;

fn get_temp_dir_path() -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    Ok(temp_dir.into_path())
}

fn cmd_create_issue(path: &Path) -> Result<Command> {
    let mut cmd = Command::cargo_bin("ripi")?;
    cmd.current_dir(path).arg("create").arg("my new issue");
    Ok(cmd)
}

#[test]
fn test_create_issue_file_creation() -> Result<()> {
    let temp_dir_path = get_temp_dir_path().context("Failed to create a temp dir")?;
    let mut cmd =
        cmd_create_issue(&temp_dir_path).context("Failed to run the 'create issue' command")?;
    cmd.assert().success();
    let expect_file_path = temp_dir_path.join("ripi/my_new_issue/README.md");
    assert!(expect_file_path.exists());
    let mut cmd = Command::cargo_bin("ripi").unwrap();
    cmd.assert().stdout("Hello, World!\n");
    Ok(())
}

#[test]
fn test_create_issue_file_content() -> Result<()> {
    let temp_dir_path = get_temp_dir_path().context("Failed to create a temp dir")?;
    let mut cmd =
        cmd_create_issue(&temp_dir_path).context("Failed to run the 'create issue' command")?;
    cmd.assert().success();
    let expect_file_path = temp_dir_path.join("ripi/my_new_issue/README.md");
    let file_contents = read_to_string(expect_file_path).expect("Failed to read file");
    assert_eq!(file_contents, "# my_new_issue");
    Ok(())
}
