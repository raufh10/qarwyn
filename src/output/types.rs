use serde::{Deserialize, Serialize};
use serde_json::Value;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

#[pyclass(from_py_object)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CriterionGrade {
  #[pyo3(get, set)]
  pub score: f64,
  #[pyo3(get, set)]
  pub feedback: String,
}

#[pymethods]
impl CriterionGrade {
  #[new]
  pub fn new(score: f64, feedback: String) -> Self {
    Self { score, feedback }
  }
}

#[pyclass(from_py_object)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GradeReport {
  pub results: Value,
}

#[pymethods]
impl GradeReport {
  #[new]
  pub fn new(results: Bound<'_, PyAny>) -> PyResult<Self> {
    let val: Value = depythonize(&results)
      .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(Self { results: val })
  }

  #[getter]
  pub fn results<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    pythonize(py, &self.results)
      .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
  }
}

pub struct OutputFilter;
