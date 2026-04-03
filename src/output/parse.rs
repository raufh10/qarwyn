use serde_json::Value;
use anyhow::{Result, Context};
use crate::output::types::{GradeReport, OutputFilter};

impl OutputFilter {
  pub fn parse_response(raw_content: String) -> Result<GradeReport> {
    let results: Value = serde_json::from_str(&raw_content)
      .context("Failed to parse LLM content string into JSON")?;

    Ok(GradeReport { results })
  }
}

