use reqwest::Error;
use std::future::Future;
use std::pin::Pin;
use std::process::Command;

#[derive(Debug)]
pub struct Message(pub String);

pub trait AiModel {
    fn comment_with_issue(&self)
        -> Pin<Box<dyn Future<Output = Result<Vec<Message>, Error>> + '_>>;

    fn comment_without_issue(
        &self,
        with_body: bool,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, Error>> + '_>>;

    fn get_git_diff(&self) -> String {
        let output = Command::new("git")
            .args(&["diff", "--staged"])
            .output()
            .expect("Failed to execute git diff command");

        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
