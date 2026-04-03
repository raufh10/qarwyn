use serde::{Deserialize, Serialize};
use serde_json::Value;
use pyo3::prelude::*;
use pythonize::pythonize;

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CriterionGrade {
  #[pyo3(get, set)]
  pub score: f64,
  #[pyo3(get, set)]
  pub feedback: String,
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GradeReport {
  pub results: Value,
}

#[pymethods]
impl GradeReport {
  #[getter]
  pub fn results(&self, py: Python<'_>) -> PyResult<PyObject> {
    pythonize(py, &self.results).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
  }
}

pub struct OutputFilter;
