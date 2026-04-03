use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Criterion {
  pub name: String,
  pub max_score: f64,
  pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rubric {
  pub title: String,
  pub criteria: Vec<Criterion>,
  pub total_score: f64,
}

impl Rubric {
  pub fn validate_sum(&self) -> bool {
    let sum: f64 = self.criteria.iter().map(|c| c.max_score).sum();
    (sum - self.total_score).abs() < 1e-9
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Essay {
  pub title: String,
  pub content: String,
  pub author: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
  pub api_key: String,
  pub model: String,
  pub name: String,
  pub system_prompt: String,
  pub rubric: Rubric,
  pub essays: Vec<Essay>,
}

impl Payload {
  pub fn new(
    api_key: String,
    model: String,
    name: String,
    system_prompt: String,
    rubric: Rubric,
    essays: Vec<Essay>,
  ) -> Self {
    Self {
      api_key,
      model,
      name,
      system_prompt,
      rubric,
      essays,
    }
  }

  pub fn validate(&self) -> Result<(), String> {
    if self.api_key.trim().is_empty() {
      return Err("API Key cannot be empty".to_string());
    }

    if self.model.trim().is_empty() {
      return Err("Model identifier cannot be empty".to_string());
    }

    if self.name.trim().is_empty() {
      return Err("Schema name cannot be empty".to_string());
    }

    if self.system_prompt.trim().is_empty() {
      return Err("System prompt cannot be empty".to_string());
    }

    if self.essays.is_empty() {
      return Err("Payload must contain at least one essay".to_string());
    }

    for (i, essay) in self.essays.iter().enumerate() {
      if essay.content.trim().is_empty() {
        return Err(format!("Essay at index {} is empty", i));
      }
    }

    if !self.rubric.validate_sum() {
      return Err("Rubric criteria sum does not match total_score".to_string());
    }

    Ok(())
  }
}

pub struct InputFilter;

impl InputFilter {
  pub fn process_payload(payload: Payload) -> io::Result<Payload> {
    payload
      .validate()
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(payload)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn mock_rubric() -> Rubric {
    Rubric {
      title: "Test".into(),
      total_score: 10.0,
      criteria: vec![
        Criterion {
          name: "C1".into(),
          max_score: 10.0,
          description: "D".into(),
        },
      ],
    }
  }

  #[test]
  fn test_valid_payload() {
    let payload = Payload::new(
      "sk-123".into(),
      "gpt-4o".into(),
      "grading_schema".into(),
      "You are a helpful assistant".into(),
      mock_rubric(),
      vec![Essay {
        title: "T".into(),
        content: "C".into(),
        author: None,
      }],
    );
    assert!(payload.validate().is_ok());
  }

  #[test]
  fn test_missing_fields() {
    let payload = Payload::new(
      "sk-123".into(),
      "".into(),
      "".into(),
      "Prompt".into(),
      mock_rubric(),
      vec![Essay {
        title: "T".into(),
        content: "C".into(),
        author: None,
      }],
    );
    assert!(payload.validate().is_err());
  }
}

