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
    (sum - self.total_score).abs() < f64::EPSILON
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
  pub essays: Vec<Essay>,
  pub rubric: Rubric,
}

impl Payload {
  pub fn new(essays: Vec<Essay>, rubric: Rubric) -> Self {
    Self { essays, rubric }
  }

  pub fn validate(&self) -> Result<(), String> {
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

/// Filter logic for input processing
pub struct InputFilter;

impl InputFilter {
  pub fn process_payload(payload: Payload) -> io::Result<Payload> {
    payload.validate().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(payload)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rubric_validation() {
    let rubric = Rubric {
      title: "Test".to_string(),
      total_score: 10.0,
      criteria: vec![
        Criterion { name: "C1".into(), max_score: 5.0, description: "D".into() },
        Criterion { name: "C2".into(), max_score: 5.0, description: "D".into() },
      ],
    };
    assert!(rubric.validate_sum());
  }

  #[test]
  fn test_invalid_payload() {
    let rubric = Rubric {
      title: "Test".to_string(),
      total_score: 10.0,
      criteria: vec![],
    };
    let payload = Payload::new(vec![], rubric);
    assert!(payload.validate().is_err());
  }
}
