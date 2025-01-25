use super::llm::{DeepSeekLLM, Message};
use std::error::Error;
use futures_util::{Stream, StreamExt};
use reqwest::Client;
use bytes::Bytes;

pub struct Chain {
    llm: DeepSeekLLM,
    client: Client,
}

impl Chain {
    pub fn new(llm: DeepSeekLLM) -> Self {
        Self {
            llm,
            client: Client::new(),
        }
    }

    pub async fn invoke(&self, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        self.llm.invoke(messages).await
    }

    pub async fn stream(&self, messages: Vec<Message>) -> Result<impl Stream<Item = Result<String, Box<dyn Error>>>, Box<dyn Error>> {
        let config = &self.llm.config;
        let request = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "stream": true
        });

        let response = self.client
            .post(&format!("{}/chat/completions", config.base_url))
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await?;
            return Err(format!("API request failed with status {}: {}", status, error_body).into());
        }

        let stream = response.bytes_stream()
            .map(|chunk| {
                chunk.map_err(|e| Box::new(e) as Box<dyn Error>)
                    .and_then(|bytes| {
                        String::from_utf8(bytes.to_vec())
                            .map_err(|e| Box::new(e) as Box<dyn Error>)
                    })
            })
            .filter_map(|result| async move {
                match result {
                    Ok(data) => Some(Ok(data)),
                    Err(e) => Some(Err(e)),
                }
            });

        Ok(stream)
    }
}
