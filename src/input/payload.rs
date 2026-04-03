use serde::{Deserialize, Serialize};
use std::io;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Criterion {
  #[pyo3(get, set)]
  pub name: String,
  #[pyo3(get, set)]
  pub max_score: f64,
  #[pyo3(get, set)]
  pub description: String,
}

#[pymethods]
impl Criterion {
  #[new]
  pub fn new(name: String, max_score: f64, description: String) -> Self {
    Self { name, max_score, description }
  }
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rubric {
  #[pyo3(get, set)]
  pub title: String,
  #[pyo3(get, set)]
  pub criteria: Vec<Criterion>,
  #[pyo3(get, set)]
  pub total_score: f64,
}

#[pymethods]
impl Rubric {
  #[new]
  pub fn new(title: String, total_score: f64, criteria: Vec<Criterion>) -> Self {
    Self { title, total_score, criteria }
  }

  pub fn validate_sum(&self) -> bool {
    let sum: f64 = self.criteria.iter().map(|c| c.max_score).sum();
    (sum - self.total_score).abs() < 1e-9
  }
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Essay {
  #[pyo3(get, set)]
  pub title: String,
  #[pyo3(get, set)]
  pub content: String,
  #[pyo3(get, set)]
  pub author: Option<String>,
}

#[pymethods]
impl Essay {
  #[new]
  pub fn new(title: String, content: String, author: Option<String>) -> Self {
    Self { title, content, author }
  }
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
  #[pyo3(get, set)]
  pub api_key: String,
  #[pyo3(get, set)]
  pub model: String,
  #[pyo3(get, set)]
  pub name: String,
  #[pyo3(get, set)]
  pub system_prompt: String,
  #[pyo3(get, set)]
  pub rubric: Rubric,
  #[pyo3(get, set)]
  pub essays: Vec<Essay>,
}

#[pymethods]
impl Payload {
  #[new]
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

  pub fn validate_py(&self) -> PyResult<()> {
    self.validate().map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
  }
}

impl Payload {
  pub fn validate(&self) -> Result<(), String> {
    if self.api_key.trim().is_empty() { return Err("API Key cannot be empty".into()); }
    if self.model.trim().is_empty() { return Err("Model identifier cannot be empty".into()); }
    if self.name.trim().is_empty() { return Err("Schema name cannot be empty".into()); }
    if self.system_prompt.trim().is_empty() { return Err("System prompt cannot be empty".into()); }
    if self.essays.is_empty() { return Err("Payload must contain at least one essay".into()); }

    for (i, essay) in self.essays.iter().enumerate() {
      if essay.content.trim().is_empty() {
        return Err(format!("Essay at index {} is empty", i));
      }
    }

    if !self.rubric.validate_sum() {
      return Err("Rubric criteria sum does not match total_score".into());
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

