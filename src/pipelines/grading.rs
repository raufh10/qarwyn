use crate::input::{Payload, InputFilter};
use crate::prompt::PromptFilter;
use crate::llm::LlmFilter;
use crate::output::{OutputFilter, GradeReport};
use anyhow::{Result, Context};

pub struct GradingPipeline;

impl GradingPipeline {
  pub async fn run(payload: Payload) -> Result<GradeReport> {

    let validated_payload = InputFilter::process_payload(payload)
      .context("Pipeline failed during input validation")?;

    let api_key = validated_payload.api_key.clone();

    let llm_payload = PromptFilter::build_llm_payload(validated_payload);

    let llm_filter = LlmFilter::new(api_key);
    let raw_response = llm_filter.call_openai(llm_payload)
      .await
      .context("Pipeline failed during LLM communication")?;

    let report = OutputFilter::parse_response(raw_response)
      .context("Pipeline failed to parse LLM response")?;

    Ok(report)
  }
}

