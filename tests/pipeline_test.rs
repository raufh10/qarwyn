use qarwyn::input::{Payload, Essay, Rubric, Criterion};
use qarwyn::pipelines::GradingPipeline;
use std::env;

#[tokio::test]
async fn test_full_pipeline_flow() {

  // 1. Setup Input with the latest 2026 model
  let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| "sk-placeholder".into());

  let payload = Payload::new(
    api_key.clone(),
    "gpt-5.4-nano-2026-03-17".into(),
    "essay_grading_schema".into(),
    "You are an academic grader. Return valid JSON only.".into(),
    Rubric {
      title: "Software Design".into(),
      total_score: 10.0,
      criteria: vec![Criterion {
        name: "Grammar".into(),
        max_score: 10.0,
        description: "Standard English usage.".into(),
      }],
    },
    vec![Essay {
      title: "Architecture".into(),
      content: "The pipe and filter pattern is efficient.".into(),
      author: Some("Dev User".into()),
    }],
  );

  // 2. Execute via the Pipeline (The Orchestrator)
  if api_key != "sk-placeholder" {
    println!("\n[INFO] Running Grading Pipeline...");
    
    let result = GradingPipeline::run(payload).await;

    match result {
      Ok(report) => {
        println!("\n--- PIPELINE SUCCESS ---");
        println!("Results: {:?}", report.results);
        println!("------------------------\n");

        assert!(report.results.get("grammar").is_some());
      }
      Err(e) => panic!("Pipeline execution failed: {:?}", e),
    }
  } else {
    println!("\n[SKIP] No API key found. Skipping live pipeline test.");
    
    assert!(payload.validate().is_ok());
  }
}

