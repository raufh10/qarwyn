use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
  pub role: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonFormat {
  pub name: String,
  pub r#type: String,
  pub strict: bool,
  pub schema: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextConfig {
  pub format: JsonFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmPayload {
  pub model: String,
  pub input: Vec<Message>,
  pub text: TextConfig,
}

impl LlmPayload {
  pub fn new_structured(
    model: &str,
    name: &str,
    system_msg: &str,
    user_msg: &str,
    schema: Value,
  ) -> Self {
    Self {
      model: model.to_string(),
      input: vec![
        Message {
          role: "system".into(),
          content: system_msg.into(),
        },
        Message {
          role: "user".into(),
          content: user_msg.into(),
        },
      ],
      text: TextConfig {
        format: JsonFormat {
          name: name.to_string(),
          r#type: "json_schema".into(),
          strict: true,
          schema,
        },
      },
    }
  }
}

