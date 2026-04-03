use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct CriterionGrade {
  pub score: f64,
  pub feedback: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeReport {
  pub results: Value,
}

pub struct OutputFilter;

impl OutputFilter {
  pub fn parse_response(raw_content: String) -> Result<GradeReport> {
    let results: Value = serde_json::from_str(&raw_content)
      .context("Failed to parse LLM content string into JSON")?;

    Ok(GradeReport { results })
  }
}
