use serde::Serialize;
use std::{fmt::Display, str::FromStr, time::Duration};

pub struct ModleConfig {
    /// The Qwen version used
    pub engine: QwenEngine,
    /// URL of the api/v1/services/aigc/text-generation/generation endpoint. Can be used to set a proxy
    pub api_url: url::Url,
    /// Timeout for the http requests sent to avoid potentially permanently hanging requests.
    pub timeout: Duration,
    /// Controls the maximum number of tokens to generate in the completion
    pub max_tokens: Option<u32>,
}

impl Default for ModleConfig {
    fn default() -> Self {
        ModleConfig {
            engine: QwenEngine::default(),
            api_url: url::Url::from_str(
                "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation",
            )
            .unwrap(),
            timeout: Duration::from_secs(100),
            max_tokens: None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub enum QwenEngine {
    #[default]
    QwenMax,
    QwenMaxLongContext,
    QwenPlus,
    QwenMax0428,
    QwenTurbo,
    Qwen2_72bInstruct,
    Qwen2_57bA14bInstruct,
    Qwen2_7bInstruct,
    Qwen2_1_5bInstruct,
    Qwen2_0_5bInstruct,
    Custom(&'static str),
}

impl Display for QwenEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for QwenEngine {
    fn as_ref(&self) -> &'static str {
        match self {
            QwenEngine::QwenMax => "qwen-max",
            QwenEngine::QwenMaxLongContext => "qwen-max-longcontext",
            QwenEngine::QwenPlus => "qwen-plus",
            QwenEngine::QwenMax0428 => "qwen-max-0428",
            QwenEngine::QwenTurbo => "qwen-turbo",
            QwenEngine::Qwen2_72bInstruct => "qwen2-72b-instruct",
            QwenEngine::Qwen2_57bA14bInstruct => "qwen2-57-a14b-instruct",
            QwenEngine::Qwen2_7bInstruct => "qwen2-7b-instrcut",
            QwenEngine::Qwen2_1_5bInstruct => "qwen2-1,5-instruct",
            QwenEngine::Qwen2_0_5bInstruct => "qwen2-0.5-instruct",
            QwenEngine::Custom(custom) => custom,
        }
    }
}
