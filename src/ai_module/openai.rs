use crate::ai_module::ai_model::{AiModel, Message};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::future::Future;
use std::pin::Pin;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: usize,
    pub message: InnerMessage,
    logprobs: Option<Logprobs>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Logprobs {
    content: Vec<TokenLogprob>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenLogprob {
    token: String,
    logprob: f64,
    bytes: Option<Vec<u8>>,
    top_logprobs: Vec<TopLogprob>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TopLogprob {
    token: String,
    logprob: f64,
    bytes: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnerMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<InnerMessage>,
}

pub struct OpenAIClient {
    pub client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIClient {
    pub fn new() -> Self {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        OpenAIClient {
            client: Client::new(),
            api_key,
            base_url: String::from("https://api.openai.com"),
        }
    }

    pub async fn fetch_chat_completion(
        &self,
        messages: Vec<InnerMessage>,
        model: String,
    ) -> Result<ChatCompletion, reqwest::Error> {
        let request_body = ChatRequest { model, messages };

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

impl AiModel for OpenAIClient {
    fn comment(
        &self,
        diff: String,
        model: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, reqwest::Error>> + '_>> {
        Box::pin(async move {
            let instructions = include_str!("./instructions.md");

            let messages = vec![
                InnerMessage {
                    role: "system".to_string(),
                    content: format!("{}", instructions),
                },
                InnerMessage {
                    role: "user".to_string(),
                    content: format!("# Git diff: \n {}", diff),
                },
            ];

            let chat_completion = self.fetch_chat_completion(messages, model).await?;

            let result_messages = chat_completion
                .choices
                .into_iter()
                .map(|choice| Message(choice.message.content))
                .collect();

            Ok(result_messages)
        })
    }
}
