use std::fs::create_dir;

use anyhow::Result;
use ripissue::{error_msgs::ERROR_MSG_NOT_PROJECT_ROOT, Context};
use tempfile::tempdir;

#[test]
fn test_context_new() -> Result<()> {
    let temp_dir = tempdir()?;
    let temp_dir_path = temp_dir.path().to_path_buf();

    let res = Context::new(temp_dir_path.clone());
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), ERROR_MSG_NOT_PROJECT_ROOT);

    let git_dir_path = temp_dir_path.join(".git");
    create_dir(git_dir_path)?;

    let res = Context::new(temp_dir_path.clone());
    assert!(res.is_ok());

    let ctx = res.unwrap();
    assert_eq!(ctx.wd(), temp_dir_path);

    Ok(())
}
