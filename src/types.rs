use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Input<'a> {
    pub messages: &'a Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
    Tool,
}
/// 用于控制模型生成的参数
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_output: Option<bool>,
}

/// A request struct sent to the API to request a message completion
#[derive(Serialize)]
pub struct CompletionRequest<'a> {
    pub model: &'a str,
    pub input: Input<'a>,
    pub parameters: Parameter,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ServerResponse {
    Error(CompletionError),
    Completion(CompletionResponse),
}

#[derive(Deserialize, Serialize)]
pub struct CompletionResponse {
    pub output: RespnseOutput,
    usage: TokenUsage,
    request_id: String,
}
impl CompletionResponse {
    pub fn content(&self) -> String {
        self.output.choices.first().unwrap().message.content.clone()
    }
}

#[derive(Deserialize, Serialize)]
pub struct RespnseOutput {
    pub choices: Vec<ResponseChoices>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseChoices {
    pub finish_reason: FininshReson,
    pub message: ChatMessage,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FininshReson {
    Null,
    Stop,
    Lenght,
}

#[derive(Deserialize, Serialize)]
pub struct TokenUsage {
    total_tokens: u32,
    output_tokens: u32,
    input_tokens: u32,
}

#[derive(Deserialize)]
pub struct CompletionError {
    pub code: String,
    pub message: String,
    pub request_id: String,
}
