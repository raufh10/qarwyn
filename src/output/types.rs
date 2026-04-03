use serde::{Deserialize, Serialize};
use serde_json::Value;

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

