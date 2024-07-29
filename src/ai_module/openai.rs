use crate::ai_module::ai_model::{AiModel, Message};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::future::Future;
use std::pin::Pin;

/// Struct representing the completion of a chat interaction.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
    system_fingerprint: Option<String>,
}

/// Struct representing a choice in the chat completion.
#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: usize,
    pub message: InnerMessage,
    finish_reason: String,
}

/// Struct representing the usage statistics of the chat completion.
#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

/// Struct representing a message in the chat interaction.
#[derive(Serialize, Deserialize, Debug)]
pub struct InnerMessage {
    pub role: String,
    pub content: String,
}

/// Struct representing the request to be sent to the chat API.
#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<InnerMessage>,
}

/// Client for interacting with the OpenAI API.
pub struct OpenAIClient<'a> {
    pub client: Client,
    api_key: &'a str,
    base_url: String,
    model: &'a str,
}

impl<'a> Default for OpenAIClient<'a> {
    fn default() -> Self {
        Self::new("gpt-3.5-turbo", "")
    }
}

impl<'a> OpenAIClient<'a> {
    /// Creates a new instance of `OpenAIClient`.
    pub fn new(model: &'a str, api_key: &'a str) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: String::from("https://api.openai.com"),
            model,
        }
    }

    /// Fetches chat completion from the OpenAI API.
    pub async fn fetch_chat_completion(
        &self,
        messages: Vec<InnerMessage>,
    ) -> Result<ChatCompletion, reqwest::Error> {
        let request_body = ChatRequest {
            model: self.model.to_string(),
            messages,
        };

        let response = self
            .client
            .post(&format!("{}/v1/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .await?
            .json::<ChatCompletion>()
            .await?;

        Ok(response)
    }
}

impl<'a> AiModel<'a> for OpenAIClient<'a> {
    fn comment_with_issue(
        &self,
        issue: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, Box<dyn StdError>>> + '_>> {
        Box::pin(async move {
            let instructions = include_str!("./prompt_comment_with_issue.md");
            let diff = self.get_git_diff();

            let messages = vec![
                InnerMessage {
                    role: "system".to_string(),
                    content: format!("# Issue \n {} {}", issue, instructions),
                },
                InnerMessage {
                    role: "user".to_string(),
                    content: format!("# Git diff: \n {}", diff),
                },
            ];

            let chat_completion = self.fetch_chat_completion(messages).await?;

            let result_messages = chat_completion
                .choices
                .into_iter()
                .map(|choice| Message(choice.message.content))
                .collect();

            Ok(result_messages)
        })
    }

    fn comment_without_issue(
        &'a self,
        with_body: bool,
        reviewer: Option<&'a str>,
        refs: Option<Vec<&'a str>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, Box<dyn StdError>>> + '_>> {
        Box::pin(async move {
            let body_instruction = if with_body {
                "commit the message with multi-paragraph body and/or multiple footers"
            } else {
                "commit the message with no body"
            };

            let reviewer_instructions = reviewer
                .map(|rev| rev.to_string())
                .unwrap_or_else(|| "Do not add Reviewed-by information".to_string());

            let refs_string = refs
                .map(|r| r.join(", "))
                .unwrap_or_else(|| "Do not add Refs information".to_string());

            let instructions = include_str!("./prompt_comment_without_an_issue.md");
            let conventions = include_str!("./prompt_conventional_commits.md");
            let diff = self.get_git_diff();

            let messages = vec![
                InnerMessage {
                    role: "system".to_string(),
                    content: format!(
                        "{} # Message body \n You MUST {} , # Reviewed-by \n {}, # Refs \n {} \n {} \n",
                        instructions,
                        body_instruction,
                        reviewer_instructions,
                        refs_string,
                        conventions
                    ),
                },
                InnerMessage {
                    role: "user".to_string(),
                    content: format!("# Git diff: \n {}", diff),
                },
            ];

            let chat_completion = self.fetch_chat_completion(messages).await?;

            let result_messages = chat_completion
                .choices
                .into_iter()
                .map(|choice| Message(choice.message.content))
                .collect();

            Ok(result_messages)
        })
    }
}
