use std::error::Error as StdError;
use std::future::Future;
use std::pin::Pin;
use std::process::Command;

#[derive(Debug)]
pub struct Message(pub String);

pub trait AiModel<'a> {
    fn comment_with_issue(
        &self,
        issue: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, Box<dyn StdError>>> + '_>>;

    fn comment_without_issue(
        &'a self,
        with_body: bool,
        reviewer: Option<&'a str>,
        refs: Option<Vec<&'a str>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, Box<dyn StdError>>> + '_>>;

    fn get_git_diff(&self) -> String {
        let output = Command::new("git")
            .args(&["diff", "--staged"])
            .output()
            .expect("Failed to execute git diff command");

        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
