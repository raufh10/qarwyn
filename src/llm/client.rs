use reqwest::Client;

pub struct LlmFilter {
  pub client: Client,
  pub api_key: String,
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
}

