use futures_core::Stream;
use futures_util::{StreamExt, TryStreamExt};
use pin_project::pin_project;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct DeepSeekLLM {
    pub config: DeepSeekConfig,
    client: Client,
}

#[derive(Debug, Clone, Serialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl ToString for MessageRole {
    fn to_string(&self) -> String {
        match self {
            MessageRole::System => "system".to_string(),
            MessageRole::User => "user".to_string(),
            MessageRole::Assistant => "assistant".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

impl Message {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self { role, content }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

impl From<Message> for ChatMessage {
    fn from(msg: Message) -> Self {
        ChatMessage {
            role: msg.role.to_string(),
            content: msg.content,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    frequency_penalty: Option<f32>,
    max_tokens: Option<u32>,
    presence_penalty: Option<f32>,
    response_format: Option<ResponseFormat>,
    stop: Option<Vec<String>>,
    stream: bool,
    stream_options: Option<StreamOptions>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<String>,
    logprobs: Option<bool>,
    top_logprobs: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamOptions {
    include_usage: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tool {
    #[serde(rename = "type")]
    tool_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatChoice {
    message: ChatMessage,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[pin_project]
pub struct PinnedStream {
    #[pin]
    inner: Pin<
        Box<dyn Stream<Item = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send>,
    >,
}

impl Stream for PinnedStream {
    type Item = Result<String, Box<dyn std::error::Error + Send + Sync>>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();
        this.inner.as_mut().poll_next(cx)
    }
}

impl PinnedStream {
    pub fn new(
        stream: impl Stream<Item = Result<String, Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + 'static,
    ) -> Self {
        Self {
            inner: Box::pin(stream),
        }
    }
}

impl DeepSeekLLM {
    pub fn new(config: DeepSeekConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    pub async fn invoke(&self, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        self._invoke(messages, false).await
    }

    pub async fn stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<
        impl Stream<Item = Result<String, Box<dyn std::error::Error + Send + Sync>>>,
        Box<dyn std::error::Error>,
    > {
        let chat_messages: Vec<ChatMessage> = messages.into_iter().map(|m| m.into()).collect();

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: chat_messages,
            frequency_penalty: None,
            max_tokens: None,
            presence_penalty: None,
            response_format: None,
            stop: None,
            stream: true,
            stream_options: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            logprobs: None,
            top_logprobs: None,
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await?;
            return Err(
                format!("API request failed with status {}: {}", status, error_body).into(),
            );
        }

        let stream = response
            .bytes_stream()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            .and_then(|chunk| async move {
                String::from_utf8(chunk.to_vec())
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            })
            .and_then(|s| async move {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                    if let Some(choices) = json.get("choices") {
                        if let Some(first_choice) = choices.get(0) {
                            if let Some(delta) = first_choice.get("delta") {
                                if let Some(content) = delta.get("content") {
                                    return Ok(content.as_str().unwrap_or("").to_string());
                                }
                            }
                        }
                    }
                }
                Ok(s)
            });

        Ok(PinnedStream::new(stream))
    }

    pub async fn _invoke(
        &self,
        messages: Vec<Message>,
        _stream: bool,
    ) -> Result<String, Box<dyn Error>> {
        let chat_messages: Vec<ChatMessage> = messages.into_iter().map(|m| m.into()).collect();

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: chat_messages,
            frequency_penalty: None,
            max_tokens: None,
            presence_penalty: None,
            response_format: None,
            stop: None,
            stream: false,
            stream_options: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            logprobs: None,
            top_logprobs: None,
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await?;
            return Err(
                format!("API request failed with status {}: {}", status, error_body).into(),
            );
        }

        let response_json: serde_json::Value = response.json().await?;

        if let Some(choices) = response_json.get("choices") {
            if let Some(first_choice) = choices.get(0) {
                if let Some(message) = first_choice.get("message") {
                    if let Some(content) = message.get("content") {
                        return Ok(content.as_str().unwrap_or("").to_string());
                    }
                }
                if let Some(finish_reason) = first_choice.get("finish_reason") {
                    if finish_reason == "length" {
                        return Err("Max tokens reached".into());
                    }
                }
            }
        }

        Err(format!("Invalid API response format: {}", response_json).into())
    }
}
