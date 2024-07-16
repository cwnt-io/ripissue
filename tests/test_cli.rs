use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use assert_cmd::Command;
use predicates::str::contains;
use slug::slugify;
use tempfile::tempdir;

// tests/common/mod.rs
fn get_temp_dir_path() -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    Ok(temp_dir.into_path())
}

// tests/common/mod.rs
fn get_cmd_create_issue(path: &Path, issue_name: &str) -> Result<Command> {
    let mut cmd = Command::cargo_bin("ripi")?;
    cmd.current_dir(path).arg("create").arg(issue_name);
    Ok(cmd)
}

#[ignore]
#[test]
fn test_cli_init() -> Result<()> {
    let temp_dir_path = get_temp_dir_path().context("Failed to create a temp dir")?;
    let mut cmd = Command::cargo_bin("ripi")?;

    cmd.current_dir(&temp_dir_path).arg("init");
    cmd.assert().success();

    let config_file_name = "ripissue.toml";
    let expect_file_path = temp_dir_path.join(config_file_name);
    assert!(expect_file_path.exists(), "Config file should be created");

    let file_contents = read_to_string(expect_file_path).expect("Failed to read file");
    assert_eq!(
        file_contents, "# my_new_issue",
        "Config file should have default content"
    );

    cmd.current_dir(&temp_dir_path).arg("init");
    cmd.assert().failure().stderr(contains(
        "Error: The 'ripissue.toml' file already exists in the current directory.",
    ));

    Ok(())
}

#[ignore]
#[test]
fn test_cli_create_issue_file() -> Result<()> {
    let issue_name = "issue name integration test";
    let issue_name_slug = slugify(issue_name);
    let project_files = ["README.md"];
    let temp_dir_path = get_temp_dir_path().context("Failed to create a temp dir")?;
    let mut cmd = get_cmd_create_issue(&temp_dir_path, issue_name)
        .context("Failed to run the 'create issue' command")?;
    cmd.assert().success();
    for file in project_files {
        let expect_file_path = temp_dir_path.join(format!("ripi/{}/{}", issue_name_slug, file));
        assert!(expect_file_path.exists());
    }
    Ok(())
}

// #[test]
// fn test_create_issue_file_content() -> Result<()> {
//     let temp_dir_path = get_temp_dir_path().context("Failed to create a temp dir")?;
//     let mut cmd =
//         get_cmd_create_issue(&temp_dir_path).context("Failed to run the 'create issue' command")?;
//     cmd.assert().success();
//     let expect_file_path = temp_dir_path.join("ripi/my_new_issue/README.md");
//     let file_contents = read_to_string(expect_file_path).expect("Failed to read file");
//     assert_eq!(file_contents, "# my_new_issue");
//     Ok(())
// }
