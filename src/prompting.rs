use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::input::Payload;

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
  pub fn new_structured(model: &str, system_msg: &str, user_msg: &str, schema: Value) -> Self {
    Self {
      model: model.to_string(),
      input: vec![
        Message { role: "system".into(), content: system_msg.into() },
        Message { role: "user".into(), content: user_msg.into() },
      ],
      text: TextConfig {
        format: JsonFormat {
          name: "essay_grading_response".into(),
          r#type: "json_schema".into(),
          strict: true,
          schema,
        },
      },
    }
  }
}

pub struct PromptFilter;

impl PromptFilter {
  pub fn build_llm_payload(payload: Payload) -> LlmPayload {
    let schema = Self::generate_grading_schema(&payload);
    
    let system_prompt = "You are an objective academic evaluator. \
      Provide a numerical score and feedback for each criterion.";

    let user_prompt = format!(
      "Assignment: {}\n\nEssays: {:?}",
      payload.rubric.title,
      payload.essays
    );

    LlmPayload::new_structured(
      "gpt-4o-2024-08-06",
      system_prompt,
      &user_prompt,
      schema,
    )
  }

  fn generate_grading_schema(payload: &Payload) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for criterion in &payload.rubric.criteria {
      let key = criterion.name.to_lowercase().replace(' ', "_");
      
      properties.insert(
        key.clone(),
        json!({
          "type": "object",
          "properties": {
            "score": { "type": "number" },
            "feedback": { "type": "string" }
          },
          "required": ["score", "feedback"],
          "additionalProperties": false
        }),
      );
      required.push(key);
    }

    json!({
      "type": "object",
      "properties": properties,
      "required": required,
      "additionalProperties": false
    })
  }
}
