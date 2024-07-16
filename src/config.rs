use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    ai_api: AiApiConfig,
}

#[derive(Serialize, Deserialize)]
pub struct AiApiConfig {
    token: String,
}

impl Default for AiApiConfig {
    fn default() -> Self {
        Self {
            token: "<TOKEN_API>".to_owned(),
        }
    }
}
