use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use ripissue::ai_module::ai_model::AiModel;
use ripissue::ai_module::openai::OpenAIClient;

#[test]
fn test_get_git_diff() {
    dotenv().ok();

    let api_key = env::var("OPENAI_KEY").expect("The environment variable OPENAI_KEY must be set.");
    let file_path = Path::new("test_file.txt");

    fs::write(&file_path, "Initial content").expect("Failed to write test file");

    Command::new("git")
        .arg("add")
        .arg(file_path)
        .output()
        .expect("Failed to add file to git");

    let client = OpenAIClient::new("gpt-3.5-turbo", &api_key);
    let diff = client.get_git_diff();

    Command::new("git")
        .arg("reset")
        .arg(file_path)
        .output()
        .expect("Failed to unstage the file");

    fs::remove_file(&file_path).expect("Failed to remove test file");

    assert!(diff.contains("Initial content"));
}
