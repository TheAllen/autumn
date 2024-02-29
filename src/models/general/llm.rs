use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    pub content: String
}

#[derive(Debug, Deserialize)]
pub struct APIChoice {
    pub message: APIMessage
}

#[derive(Debug, Deserialize)]
pub struct APIUsage {
    pub completion_tokens: u16,
    pub prompt_tokens: u16,
    pub total_tokens: u16
}

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub choice: Vec<APIChoice>
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String
}