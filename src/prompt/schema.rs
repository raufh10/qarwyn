use serde_json::{json, Value};
use crate::input::Payload;
use crate::prompt::llm_payload::LlmPayload;

pub struct PromptFilter;

impl PromptFilter {
  pub fn build_llm_payload(payload: Payload) -> LlmPayload {
    let schema = Self::generate_grading_schema(&payload);

    let user_prompt = format!(
      "Assignment: {}\n\nEssays: {:?}",
      payload.rubric.title, payload.essays
    );

    LlmPayload::new_structured(
      &payload.model,
      &payload.name,
      &payload.system_prompt,
      &user_prompt,
      schema,
    )
  }

  pub fn generate_grading_schema(payload: &Payload) -> Value {
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

