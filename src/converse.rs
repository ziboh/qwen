use futures_util::Stream;

use crate::{
    client::Qwen,
    types::{ChatMessage, CompletionResponse, Role},
};

pub struct Conversation {
    pub(crate) client: Qwen,
    history: Vec<ChatMessage>,
}

impl Conversation {
    pub fn new(client: Qwen, first_message: String) -> Self {
        Self {
            client,
            history: vec![ChatMessage {
                role: crate::types::Role::System,
                content: first_message,
            }],
        }
    }
    pub fn new_with_history(client: Qwen, history: Vec<ChatMessage>) -> Self {
        Self { client, history }
    }

    /// Rollbacks the history by 1 message, removing the last sent and received message.
    pub fn rollback(&mut self) -> Option<ChatMessage> {
        let last = self.history.pop();
        self.history.pop();
        last
    }

    /// Sends the message to the ChatGPT API and returns the completion response.
    ///
    /// Execution speed depends on API response times.
    pub async fn send_message<S: Into<String> + Send + Sync>(
        &mut self,
        message: S,
    ) -> crate::Result<CompletionResponse> {
        self.history.push(ChatMessage {
            role: Role::User,
            content: message.into(),
        });
        let response = self.client.send_history_message(&self.history).await;
        if let Ok(complete) = response.as_ref() {
            self.history.push(ChatMessage {
                role: Role::Assistant,
                content: complete.content(),
            })
        }
        response
    }
    pub async fn send_message_streaming<S: Into<String> + Send + Sync>(
        &mut self,
        message: S,
    ) -> crate::Result<impl Stream<Item = String>> {
        self.history.push(ChatMessage {
            role: Role::User,
            content: message.into(),
        });
        self.client.send_history_message_streaming(&self.history).await
    }

    pub fn add_history(&mut self,content:String){
        self.history.push(ChatMessage{
            role:Role::Assistant,
            content,
        })
    }

    pub fn clear_history(&mut self){
        self.history.truncate(1);
    }

}
