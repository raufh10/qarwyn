use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;

// 1. Existing Rust Module Tree
pub mod input;
pub mod prompt;
pub mod llm;
pub mod output;
pub mod pipelines;

pub use input::Payload;
pub use pipelines::GradingPipeline;
pub use output::GradeReport;

#[pyfunction]
#[pyo3(name = "run_grading_pipeline")]
pub fn python_run_pipeline(payload: input::payload::Payload) -> PyResult<output::types::GradeReport> {
  let rt = tokio::runtime::Runtime::new()
    .map_err(|e| PyRuntimeError::new_err(format!("Failed to boot Tokio runtime: {}", e)))?;

  rt.block_on(async {
    pipelines::grading::GradingPipeline::run(payload)
      .await
      .map_err(|e| PyRuntimeError::new_err(format!("Pipeline Error: {:?}", e)))
  })
}

#[pymodule]
fn qarwyn(m: &Bound<'_, PyModule>) -> PyResult<()> {
  // Export Data Classes
  m.add_class::<input::payload::Payload>()?;
  m.add_class::<input::payload::Essay>()?;
  m.add_class::<input::payload::Rubric>()?;
  m.add_class::<input::payload::Criterion>()?;
  m.add_class::<output::types::GradeReport>()?;
  m.add_class::<output::types::CriterionGrade>()?;

  m.add_function(wrap_pyfunction!(python_run_pipeline, m)?)?;

  Ok(())
}
