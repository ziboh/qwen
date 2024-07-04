use crate::{
    config::ModleConfig,
    types::{
        ChatMessage, CompletionRequest, CompletionResponse, Input, Parameter, Role, ServerResponse,
    },
};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

#[cfg(feature = "streams")]
use futures_util::Stream;
#[cfg(feature = "streams")]
use reqwest::Response;

pub struct Qwen {
    pub client: Client,
    pub config: ModleConfig,
}

impl Qwen {
    pub fn new<S>(api_key: S) -> crate::Result<Self>
    where
        S: Into<String>,
    {
        Self::new_with_config(api_key, ModleConfig::default())
    }

    pub fn new_with_config<S>(api_key: S, config: ModleConfig) -> crate::Result<Self>
    where
        S: Into<String>,
    {
        let api_key = api_key.into();
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {}", api_key).as_bytes())?,
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()?;
        Ok(Self { client, config })
    }
    pub async fn send_message<S: Into<String>>(
        &self,
        question: S,
    ) -> crate::Result<CompletionResponse> {
        let input = vec![
            ChatMessage {
                role: Role::System,
                content: "You are a helpful assistant.".to_string(),
            },
            ChatMessage {
                role: Role::User,
                content: question.into(),
            },
        ];
        self.send_history_message(&input).await
    }

    pub async fn send_history_message(
        &self,
        history: &Vec<ChatMessage>,
    ) -> crate::Result<CompletionResponse> {
        let response: ServerResponse = self
            .client
            .post(self.config.api_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                input: Input { messages: history },
                parameters: Parameter {
                    result_format: Some("message".to_string()),
                    incremental_output: None,
                },
            })
            .send()
            .await?
            .json()
            .await?;
        match response {
            ServerResponse::Error(error) => Err(crate::err::Error::BackendError {
                message: error.message,
                code: error.code,
            }),
            ServerResponse::Completion(completion) => Ok(completion),
        }
    }

    #[cfg(feature = "streams")]
    pub async fn send_history_message_streaming(
        &self,
        history: &Vec<ChatMessage>,
    ) -> crate::Result<impl Stream<Item = String>> {
        let response = self
            .client
            .post(self.config.api_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("X-DashScope-SSE", "enable")
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                input: Input { messages: history },
                parameters: Parameter {
                    result_format: Some("message".to_string()),
                    incremental_output: Some(true),
                },
            })
            .send()
            .await?;
        self.process_streaming_response(response)
    }

    #[cfg(feature = "streams")]
    pub async fn send_message_streaming<S>(
        &self,
        question: S,
    ) -> crate::Result<impl Stream<Item = String>>
    where
        S: Into<String>,
    {
        let input = vec![
            ChatMessage {
                role: Role::System,
                content: "You are a helpful assistant.".to_string(),
            },
            ChatMessage {
                role: Role::User,
                content: question.into(),
            },
        ];
        self.send_history_message_streaming(&input).await
    }
    #[cfg(feature = "streams")]
    fn process_streaming_response(
        &self,
        response: Response,
    ) -> crate::Result<impl Stream<Item = String>> {
        use eventsource_stream::Eventsource;
        use futures_util::StreamExt;

        let mut a = String::new();
        response
            .error_for_status()
            .map(|response| {
                let response_stream = response.bytes_stream().eventsource();
                a.push_str("hello");
                response_stream.map(|part| {
                    let data = part.expect("Stream closed abruptly!").data;
                    let res_data: CompletionResponse = serde_json::from_str(&data).expect("eror");
                    res_data.content()
                })
            })
            .map_err(crate::err::Error::from)
    }
}
