use qarwyn::input::{Payload, Essay, Rubric, Criterion};
use qarwyn::prompting::PromptFilter;
use qarwyn::llm::LlmFilter;
use std::env;

#[tokio::test]
async fn test_full_flow_with_llm() {
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

  // 2. Transform to LLM Request
  let llm_request = PromptFilter::build_llm_payload(payload);

  // 3. Initialize LLM Filter and Execute
  if api_key != "sk-placeholder" {
    let llm_filter = LlmFilter::new(api_key);

    println!("\n[INFO] Calling OpenAI Responses API...");
    let result = llm_filter.call_openai(llm_request).await;

    match result {
      Ok(inner_json_string) => {
        println!("\n--- EXTRACTED GRADE JSON ---");
        println!("{}", inner_json_string);
        println!("----------------------------\n");

        let final_check: serde_json::Value = serde_json::from_str(&inner_json_string)
          .expect("Extracted content is not valid JSON");

        assert!(final_check.get("grammar").is_some());
      }
      Err(e) => panic!("Full flow failed: {:?}", e),
    }
  } else {
    println!("\n[SKIP] No API key found. Skipping live test.");
    assert_eq!(llm_request.model, "gpt-5.4-nano-2026-03-17");
  }
}

