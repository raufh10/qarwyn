use reqwest::Client;
use crate::prompting::LlmPayload;
use anyhow::{Result, Context};
use serde_json::Value;

pub struct LlmFilter {
  client: Client,
  api_key: String,
}

impl LlmFilter {
  pub fn new(api_key: String) -> Self {
    Self {
      client: Client::builder()
        .build()
        .expect("Failed to create HTTP client"),
      api_key,
    }
  }

  pub async fn call_openai(&self, payload: LlmPayload) -> Result<String> {
    let url = "https://api.openai.com/v1/responses";

    let response = self.client
      .post(url)
      .header("Authorization", format!("Bearer {}", self.api_key))
      .json(&payload)
      .send()
      .await
      .context("Failed to send request to OpenAI")?;

    let status = response.status();
    let raw_body = response.text().await
      .context("Failed to read response body as text")?;

    if !status.is_success() {
      return Err(anyhow::anyhow!("OpenAI API Error ({}): {}", status, raw_body));
    }

    let parsed: Value = serde_json::from_str(&raw_body)
      .context("Failed to parse API response as JSON")?;

    let content = parsed["output"][0]["content"][0]["text"]
      .as_str()
      .context("Could not find inner 'text' string in OpenAI response structure")?
      .to_string();

    Ok(content)
  }
}
